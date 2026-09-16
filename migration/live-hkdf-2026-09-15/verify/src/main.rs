//! Independent fixed-schedule evidence verifier; never imports the engine or producer.
//! The two expected hashes are external custody inputs, not claims from the ledger.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

const PERIOD: u64 = 500_000_000;
const DEADLINE: u64 = 1_000_000_000;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Response {
    id: u64,
    generation: String,
    phase_before: u8,
    phase_after: u8,
    authority_floor: u64,
    value: Vec<u8>,
    error: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Observation {
    id: u64,
    scheduled_ns: u64,
    dispatch_ns: u64,
    start_ns: u64,
    end_ns: u64,
    worker: u8,
    route: u8,
    sample: u8,
    consumer_hold: bool,
    endpoint: Option<String>,
    response: Option<Response>,
    transport_error: String,
}
#[derive(Default, Serialize)]
struct Report {
    accepted: bool,
    failures: Vec<String>,
    artifacts_sha256: BTreeMap<String, String>,
    issued: usize,
    completed: usize,
    scheduled_during_outage: usize,
    interrupted_inflight: usize,
    expected_retired_source_refusals: usize,
    on_time_by_phase_route: BTreeMap<String, usize>,
    activation_crossings: usize,
    protected_emission_overlap: bool,
    measured_protected_callback_interval_ns: Option<u64>,
    availability_latency_ns: Vec<u64>,
}
impl Report {
    fn check(&mut self, condition: bool, message: impl Into<String>) {
        if !condition {
            self.failures.push(message.into());
        }
    }
}
fn number(v: &Value, field: &str) -> Result<u64, String> {
    v.get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing/invalid integer {field}"))
}
fn string<'a>(v: &'a Value, field: &str) -> Result<&'a str, String> {
    v.get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing/invalid string {field}"))
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn read(root: &Path, name: &str, report: &mut Report) -> Result<Vec<u8>, String> {
    let path = root.join(name);
    let metadata = fs::symlink_metadata(&path).map_err(|e| format!("{name}: {e}"))?;
    if !metadata.is_file() || metadata.len() > 128 * 1024 * 1024 {
        return Err(format!("{name}: expected bounded regular file"));
    }
    let bytes = fs::read(path).map_err(|e| format!("{name}: {e}"))?;
    report
        .artifacts_sha256
        .insert(name.into(), hex(&Sha256::digest(&bytes)));
    Ok(bytes)
}
fn object(root: &Path, name: &str, report: &mut Report) -> Result<Value, String> {
    serde_json::from_slice(&read(root, name, report)?).map_err(|e| format!("{name}: {e}"))
}
fn rows(root: &Path, name: &str, report: &mut Report) -> Result<Vec<Observation>, String> {
    let bytes = read(root, name, report)?;
    if !bytes.ends_with(b"\n") {
        return Err(format!("{name}: missing durable final newline"));
    }
    bytes[..bytes.len() - 1]
        .split(|b| *b == b'\n')
        .enumerate()
        .map(|(line, raw)| {
            if raw.len() > 64 * 1024 {
                return Err(format!("{name}: oversized row {}", line + 1));
            }
            serde_json::from_slice(raw).map_err(|e| format!("{name}: row {}: {e}", line + 1))
        })
        .collect()
}

fn audit(
    root: &Path,
    source: &str,
    binary: &str,
    oracle_pin: &str,
    report: &mut Report,
) -> Result<(), String> {
    for pin in [source, binary, oracle_pin] {
        if pin.len() != 64
            || !pin
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err("external source/binary pins must be lowercase SHA256".into());
        }
    }
    let contract = object(root, "schedule-contract.json", report)?;
    let stop = object(root, "schedule-stop.json", report)?;
    let outage = object(root, "owned-interruption.json", report)?;
    let oracle = object(root, "response-oracle.json", report)?;
    report.check(
        report
            .artifacts_sha256
            .get("response-oracle.json")
            .map(String::as_str)
            == Some(oracle_pin),
        "external pre-campaign oracle pin mismatch",
    );
    let fixture = read(root, "expected-typed-sections.bincode", report)?;
    report.check(
        hex(&Sha256::digest(&fixture)) == string(&oracle, "fixture_sha256")?,
        "retained typed fixture hash mismatch",
    );
    drop(fixture);
    let operation = object(root, "operation-result.json", report)?;
    let overlap = object(root, "resume/cutover-overlap.json", report)?;
    let markers = &overlap["markers"];
    let issued = rows(root, "parent-issued.jsonl", report)?;
    let results = rows(root, "parent-results.jsonl", report)?;
    report.issued = issued.len();
    report.completed = results.len();
    for (field, value) in [
        ("version", 1),
        ("period_ns", PERIOD),
        ("deadline_ns", DEADLINE),
        ("workers", 2),
        ("queue_per_worker", 4),
    ] {
        report.check(
            number(&contract, field)? == value,
            format!("contract {field} changed"),
        );
    }
    report.check(
        string(&contract, "source_sha256")? == source,
        "external source pin mismatch",
    );
    report.check(
        string(&contract, "binary_sha256")? == binary,
        "external binary pin mismatch",
    );
    let stop_ns = number(&stop, "stop_ns")?;
    let expected_count = stop_ns / PERIOD + 1;
    report.check(
        number(&stop, "issued_count")? == expected_count,
        "stop does not cover every scheduled tick",
    );
    report.check(
        number(&stop, "expected_last_id")? == expected_count - 1,
        "last scheduled ID inconsistent",
    );
    report.check(
        issued.len() as u64 == expected_count,
        "missing issued schedule ticks",
    );
    report.check(issued.len() == results.len(), "issued/result counts differ");
    report.check(
        number(&operation, "observations_count")? == results.len() as u64,
        "operation/result count mismatch",
    );
    let killed = number(&outage, "kill_start_ns")?;
    let kill_end = number(&outage, "kill_end_ns")?;
    let resumed = number(&outage, "resume_endpoint_ns")?;
    report.check(
        killed < kill_end && kill_end <= resumed && resumed < stop_ns,
        "invalid kill/resume/stop order",
    );
    report.check(
        outage["killed_exit_success"] == false && number(&outage, "killed_signal")? == 9,
        "owned child was not observed SIGKILL",
    );
    let original_pid = number(&outage, "original_child_pid")?;
    let resumed_pid = number(&outage, "resumed_child_pid")?;
    report.check(
        original_pid > 0 && resumed_pid > 0 && original_pid != resumed_pid,
        "cold resume process identity invalid",
    );
    let accounted = number(&outage, "killed_after_accounted")?;
    report.check(
        (50_000..=50_128).contains(&accounted),
        "kill outside preregistered halfway boundary",
    );
    let initial_floor = number(&outage, "initial_authority_floor")?;
    let checkpoint_floor = number(&outage, "checkpoint_authority_floor")?;
    let resumed_floor = number(&outage, "resumed_authority_floor")?;
    let final_floor = number(&operation, "authority_floor")?;
    report.check(
        initial_floor <= checkpoint_floor
            && checkpoint_floor <= resumed_floor
            && resumed_floor <= final_floor,
        "authority floor regression",
    );
    report.check(
        string(&outage, "fixture_sha256")? == string(&oracle, "fixture_sha256")?,
        "fixture identity mismatch",
    );
    let original = &outage["original_endpoint"];
    let successor = &outage["resumed_endpoint"];
    let original_id = string(original, "generation")?;
    let resumed_id = string(successor, "generation")?;
    report.check(
        original_id != resumed_id,
        "endpoint generation reused on cold resume",
    );
    for endpoint in [original, successor] {
        report.check(
            number(endpoint, "version")? == 1,
            "unsupported endpoint version",
        );
        for (field, pin) in [("source_pin", source), ("binary_pin", binary)] {
            let bytes: Vec<u8> = serde_json::from_value(endpoint[field].clone())
                .map_err(|e| format!("endpoint {field}: {e}"))?;
            report.check(
                hex(&bytes) == pin,
                format!("endpoint {field} custody mismatch"),
            );
        }
        let ports: Vec<u16> =
            serde_json::from_value(endpoint["ports"].clone()).map_err(|e| format!("ports: {e}"))?;
        report.check(
            ports.len() == 2 && ports.iter().all(|p| *p != 0) && ports[0] != ports[1],
            "endpoint ports invalid",
        );
    }
    let inflight: Vec<u64> = serde_json::from_value(outage["candidate_inflight_ids"].clone())
        .map_err(|e| format!("inflight: {e}"))?;
    let inflight_set: BTreeSet<_> = inflight.iter().copied().collect();
    report.check(
        inflight.len() == inflight_set.len() && inflight.len() <= 2,
        "invalid inflight candidate inventory",
    );
    let old_values: Vec<Vec<u8>> = serde_json::from_value(oracle["old_source"].clone())
        .map_err(|e| format!("old oracle: {e}"))?;
    let live_values: Vec<Option<Vec<u8>>> = serde_json::from_value(oracle["live_facade"].clone())
        .map_err(|e| format!("live oracle: {e}"))?;
    if old_values.len() != 4
        || live_values.len() != 4
        || live_values[3].is_some()
        || live_values[..3].iter().any(Option::is_none)
    {
        return Err("unexpected typed oracle shape".into());
    }
    let missing = string(&oracle, "expected_missing_error")?;
    let retired = string(&oracle, "expected_retired_error")?;
    let mut by_id = BTreeMap::new();
    for result in &results {
        report.check(
            by_id.insert(result.id, result).is_none(),
            format!("duplicate completed ID {}", result.id),
        );
    }
    let mut previous_dispatch = 0;
    let mut held_ids = Vec::new();
    for (index, request) in issued.iter().enumerate() {
        let id = request.id;
        report.check(id == index as u64, format!("issued ID ordering {index}"));
        report.check(
            request.scheduled_ns == id.saturating_mul(PERIOD),
            format!("shifted scheduled time {id}"),
        );
        report.check(
            request.dispatch_ns >= request.scheduled_ns && request.dispatch_ns >= previous_dispatch,
            format!("invalid dispatch ordering {id}"),
        );
        previous_dispatch = request.dispatch_ns;
        let expected_sample = if request.consumer_hold {
            0
        } else {
            ((id / 4) % 4) as u8
        };
        report.check(
            request.worker == (id % 2) as u8
                && request.sample == expected_sample
                && request.route <= 1,
            format!("invalid sampler selection {id}"),
        );
        report.check(
            request.start_ns == 0
                && request.end_ns == 0
                && request.response.is_none()
                && request.transport_error.is_empty(),
            format!("issued row contains invented result {id}"),
        );
        let Some(row) = by_id.remove(&id) else {
            report.check(false, format!("no outcome for scheduled ID {id}"));
            continue;
        };
        report.check(
            row.scheduled_ns == request.scheduled_ns
                && row.dispatch_ns == request.dispatch_ns
                && row.worker == request.worker
                && row.route == request.route
                && row.sample == request.sample
                && row.endpoint == request.endpoint
                && row.consumer_hold == request.consumer_hold,
            format!("immutable request changed {id}"),
        );
        if request.consumer_hold {
            held_ids.push(id);
            report.check(
                row.route == 1 && row.endpoint.as_deref() == Some(resumed_id),
                format!("invalid held consumer route/endpoint {id}"),
            );
        }
        report.check(
            row.dispatch_ns <= row.start_ns && row.start_ns <= row.end_ns,
            format!("invalid completion clock {id}"),
        );
        let outage_scheduled = (killed..resumed).contains(&row.scheduled_ns);
        let interrupted = row.endpoint.as_deref() == Some(original_id)
            && inflight_set.contains(&id)
            && row.start_ns < killed
            && row.end_ns >= killed
            && row.scheduled_ns.saturating_add(DEADLINE) >= killed
            && row.response.is_none()
            && !row.transport_error.is_empty();
        if outage_scheduled {
            report.scheduled_during_outage += 1;
        } else if interrupted {
            report.interrupted_inflight += 1;
        }
        let excluded = outage_scheduled || interrupted;
        if row.dispatch_ns < killed {
            report.check(
                row.endpoint.as_deref() == Some(original_id),
                format!("wrong pre-kill endpoint {id}"),
            );
        }
        if row.dispatch_ns >= resumed {
            report.check(
                row.endpoint.as_deref() == Some(resumed_id),
                format!("wrong resumed endpoint {id}"),
            );
        }
        let on_time = row.end_ns <= row.scheduled_ns.saturating_add(DEADLINE);
        if !excluded {
            report
                .availability_latency_ns
                .push(row.end_ns.saturating_sub(row.scheduled_ns));
            report.check(on_time, format!("scheduled deadline missed {id}"));
        }
        let Some(response) = &row.response else {
            report.check(
                !row.transport_error.is_empty(),
                format!("missing response and failure {id}"),
            );
            report.check(
                excluded,
                format!(
                    "availability transport failure {id}: {}",
                    row.transport_error
                ),
            );
            continue;
        };
        report.check(
            row.transport_error.is_empty(),
            format!("response plus transport error {id}"),
        );
        report.check(
            response.id == id && Some(response.generation.as_str()) == row.endpoint.as_deref(),
            format!("response identity mismatch {id}"),
        );
        // Phase7 is coordination after verification and before activation.
        let rank = |p: u8| match p {
            1..=4 => p * 2,
            7 => 9,
            5 => 10,
            6 => 12,
            _ => 255,
        };
        report.check(
            rank(response.phase_before) != 255
                && rank(response.phase_after) != 255
                && rank(response.phase_before) <= rank(response.phase_after),
            format!("invalid read phase {id}"),
        );
        let min_floor = if row.endpoint.as_deref() == Some(resumed_id) {
            resumed_floor
        } else {
            initial_floor
        };
        report.check(
            (min_floor..=final_floor).contains(&response.authority_floor),
            format!("reply authority floor outside custody bounds {id}"),
        );
        let retired_source = row.route == 0
            && response.phase_after == 6
            && response.error == retired
            && response.value.is_empty();
        if retired_source {
            report.expected_retired_source_refusals += 1;
            continue;
        }
        let sample = row.sample as usize;
        if sample >= 4 || row.route > 1 {
            continue;
        }
        let valid_content = if row.route == 1 && sample == 3 {
            response.value.is_empty() && response.error == missing
        } else {
            let expected = if row.route == 0 {
                &old_values[sample]
            } else {
                live_values[sample].as_ref().unwrap()
            };
            response.error.is_empty() && &response.value == expected
        };
        report.check(valid_content, format!("typed response mismatch {id}"));
        if request.consumer_hold {
            report.check(
                valid_content && on_time && !excluded,
                "held consumer failed normal fixed-schedule acceptance",
            );
            report.check(
                row.sample == 0 && response.phase_before == 7,
                "held consumer did not emit the changed record while ReadyToActivate",
            );
            report.check(
                number(markers, "request_id")? == id,
                "overlap receipt request mismatch",
            );
            report.check(
                string(&overlap, "endpoint_generation")? == resumed_id,
                "overlap endpoint mismatch",
            );
            report.check(
                !string(&overlap, "clock_domain")?.is_empty(),
                "missing child monotonic clock identity",
            );
            let held = number(markers, "request_held_ns")?;
            let invoke = number(markers, "finish_invocation_ns")?;
            let release = number(markers, "callback_release_ns")?;
            let response_end = number(markers, "response_end_ns")?;
            let finish_return = number(markers, "finish_return_ns")?;
            report.check(
                held < invoke
                    && invoke < release
                    && release <= response_end
                    && release <= finish_return,
                "protected emission and activation do not overlap in one clock domain",
            );
            report.check(
                number(&overlap, "hold_budget_ns")? == 25_000_000,
                "consumer hold contract changed",
            );
            report.measured_protected_callback_interval_ns = release.checked_sub(held);
            // Callback release causally precedes the bytes the parent receives.
            // A post-write bookkeeping marker can be delayed after the parent
            // already received its response, so it is not an RPC lower bound.
            report.check(
                release.saturating_sub(held) <= row.end_ns.saturating_sub(row.start_ns),
                "child hold exceeds measured parent RPC",
            );
            report.check(
                markers["response_written"] == true && overlap["actual_finish_succeeded"] == true,
                "protected emission/activation did not complete",
            );
            report.protected_emission_overlap = valid_content && on_time && !excluded;
        }
        if valid_content && on_time && !excluded {
            if response.phase_before == response.phase_after {
                *report
                    .on_time_by_phase_route
                    .entry(format!("{}:{}", response.phase_before, row.route))
                    .or_default() += 1;
            }
            if row.route == 1 && response.phase_before <= 5 && response.phase_after == 6 {
                report.activation_crossings += 1;
            }
        }
    }
    report.check(by_id.is_empty(), "completed requests were never issued");
    // Compare only reads that demonstrably do not overlap. Concurrent replies
    // can be reordered, so request ID or response arrival alone is insufficient.
    let mut starts: Vec<_> = results.iter().filter(|r| r.response.is_some()).collect();
    let mut ends = starts.clone();
    starts.sort_by_key(|r| r.start_ns);
    ends.sort_by_key(|r| r.end_ns);
    let mut previous_floors: BTreeMap<&str, u64> = BTreeMap::new();
    let mut end_index = 0;
    for row in starts {
        while end_index < ends.len() && ends[end_index].end_ns <= row.start_ns {
            let prior = ends[end_index].response.as_ref().unwrap();
            previous_floors
                .entry(&prior.generation)
                .and_modify(|floor| *floor = (*floor).max(prior.authority_floor))
                .or_insert(prior.authority_floor);
            end_index += 1;
        }
        let response = row.response.as_ref().unwrap();
        if let Some(prior) = previous_floors.get(response.generation.as_str()) {
            report.check(
                response.authority_floor >= *prior,
                format!(
                    "authority reply floor regressed across non-overlapping reads {}",
                    row.id
                ),
            );
        }
    }
    for phase in [1, 3, 4] {
        for route in [0, 1] {
            let count = *report
                .on_time_by_phase_route
                .get(&format!("{phase}:{route}"))
                .unwrap_or(&0);
            report.check(
                count >= 2,
                format!("insufficient phase {phase} route {route}: {count}"),
            );
        }
    }
    report.check(
        *report.on_time_by_phase_route.get("6:1").unwrap_or(&0) >= 4,
        "insufficient Active facade reads",
    );
    report.check(
        held_ids.len() == 1,
        "expected exactly one scheduled held-consumer request",
    );
    report.check(
        report.protected_emission_overlap,
        "actual protected emission overlap unobserved",
    );
    for (field, value) in [
        ("source_count", 100000),
        ("target_count", 100000),
        ("copied", 99998),
        ("superseded_by_write", 1),
        ("superseded_by_delete", 1),
        ("failed_attempts", 0),
    ] {
        report.check(
            number(&operation, field)? == value,
            format!("operation {field} mismatch"),
        );
    }
    for (field, value) in [
        ("operation", "HkdfReplacement"),
        ("source_kdf", "HkdfSha256"),
        ("target_kdf", "HkdfSha384"),
        ("source_cipher", "Aes256GcmSiv"),
        ("target_cipher", "Aes256GcmSiv"),
        ("required_index", "WordnetLexicalV1"),
    ] {
        report.check(
            string(&operation, field)? == value,
            format!("operation {field} mismatch"),
        );
    }
    report.check(
        operation["cold_reopen_exact_typed_index"] == true,
        "missing final cold-reopen typed/index oracle",
    );
    report.check(
        operation["availability_failures"] == json!([]),
        "producer reported failures",
    );
    report.availability_latency_ns.sort_unstable();
    Ok(())
}

fn verify(root: &Path, source: &str, binary: &str, oracle_pin: &str) -> Report {
    let mut report = Report::default();
    if let Err(error) = audit(root, source, binary, oracle_pin, &mut report) {
        report.failures.push(error);
    }
    report.accepted = report.failures.is_empty();
    report
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("usage: p11-availability-audit EVIDENCE_DIR SOURCE_SHA256 BINARY_SHA256 PRE_CAMPAIGN_ORACLE_SHA256");
        std::process::exit(2);
    }
    let report = verify(Path::new(&args[1]), &args[2], &args[3], &args[4]);
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    if !report.accepted {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests;
