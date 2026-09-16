use super::*;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    path: PathBuf,
    issued: Vec<Observation>,
    results: Vec<Observation>,
    oracle_pin: String,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
impl Fixture {
    fn write(&self, name: &str, value: Value) {
        fs::write(self.path.join(name), serde_json::to_vec(&value).unwrap()).unwrap();
    }
    fn edit(&self, name: &str, f: impl FnOnce(&mut Value)) {
        let mut value = serde_json::from_slice(&fs::read(self.path.join(name)).unwrap()).unwrap();
        f(&mut value);
        self.write(name, value);
    }
    fn flush(&self) {
        for (name, rows) in [
            ("parent-issued.jsonl", &self.issued),
            ("parent-results.jsonl", &self.results),
        ] {
            let text = rows
                .iter()
                .map(|r| serde_json::to_string(r).unwrap() + "\n")
                .collect::<String>();
            fs::write(self.path.join(name), text).unwrap();
        }
    }
    fn audit(&self) -> Report {
        self.flush();
        verify(
            &self.path,
            &"01".repeat(32),
            &"02".repeat(32),
            &self.oracle_pin,
        )
    }
}
fn fixture() -> Fixture {
    let path = std::env::temp_dir().join(format!(
        "p11-ledger-control-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).unwrap();
    fs::create_dir(path.join("resume")).unwrap();
    let mut f = Fixture {
        path,
        issued: Vec::new(),
        results: Vec::new(),
        oracle_pin: String::new(),
    };
    f.write("schedule-contract.json",json!({"version":1,"period_ns":PERIOD,"deadline_ns":DEADLINE,
        "workers":2,"queue_per_worker":4,"source_sha256":"01".repeat(32),"binary_sha256":"02".repeat(32)}));
    f.write(
        "schedule-stop.json",
        json!({"stop_ns":39*PERIOD,"issued_count":40,"expected_last_id":39}),
    );
    let endpoint = |id| json!({"version":1,"generation":id,"ports":[12001,12002],"source_pin":vec![1;32],"binary_pin":vec![2;32]});
    f.write("owned-interruption.json",json!({"version":1,"kill_start_ns":10_100_000_000u64,
        "kill_end_ns":10_200_000_000u64,"resume_endpoint_ns":11_000_000_000u64,
        "killed_exit_success":false,"killed_signal":9,"original_child_pid":100,"resumed_child_pid":101,
        "killed_after_accounted":50048,"initial_authority_floor":1,"checkpoint_authority_floor":3,
        "resumed_authority_floor":4,"fixture_sha256":"03".repeat(32),
        "original_endpoint":endpoint("first"),"resumed_endpoint":endpoint("second"),"candidate_inflight_ids":[20]}));
    f.write(
        "response-oracle.json",
        json!({"version":1,"fixture_sha256":"03".repeat(32),
        "old_source":[[0],[1],[2],[3]],"live_facade":[[4],[5],[6],null],
        "expected_missing_error":"section not found","expected_retired_error":"retired"}),
    );
    f.write("operation-result.json",json!({"source_count":100000,"target_count":100000,"copied":99998,
        "superseded_by_write":1,"superseded_by_delete":1,"failed_attempts":0,"authority_floor":10,
        "observations_count":40,"operation":"HkdfReplacement","source_kdf":"HkdfSha256",
        "target_kdf":"HkdfSha384","source_cipher":"Aes256GcmSiv","target_cipher":"Aes256GcmSiv",
        "required_index":"WordnetLexicalV1","cold_reopen_exact_typed_index":true,"availability_failures":[]}));
    f.write("resume/cutover-overlap.json",json!({"endpoint_generation":"second","clock_domain":"child-clock",
        "hold_budget_ns":25_000_000,"actual_finish_succeeded":true,"markers":{"request_id":24,
        "request_held_ns":100_000_000,"finish_invocation_ns":101_000_000,"callback_release_ns":126_000_000,
        "response_end_ns":130_000_000,"finish_return_ns":128_000_000,"response_written":true}}));
    for id in 0..40u64 {
        let route = if id >= 12 { 1 } else { ((id / 2) % 2) as u8 };
        let sample = if id == 24 { 0 } else { ((id / 4) % 4) as u8 };
        let endpoint = if id == 21 {
            None
        } else {
            Some(if id >= 22 { "second" } else { "first" }.into())
        };
        let request = Observation {
            id,
            scheduled_ns: id * PERIOD,
            dispatch_ns: id * PERIOD + 1_000_000,
            start_ns: 0,
            end_ns: 0,
            worker: (id % 2) as u8,
            route,
            sample,
            consumer_hold: id == 24,
            endpoint,
            response: None,
            transport_error: String::new(),
        };
        f.issued.push(request.clone());
        let mut result = request;
        result.start_ns = id * PERIOD + 2_000_000;
        result.end_ns = id * PERIOD + 10_000_000;
        if id == 24 {
            result.end_ns += 30_000_000;
        }
        if id == 20 || id == 21 {
            result.transport_error = "owned process interruption".into();
            if id == 20 {
                result.end_ns = 10_200_000_000;
            }
        } else {
            let phase = match id {
                0..=3 => 1,
                4..=7 => 3,
                8..=11 => 4,
                12 => 5,
                24 => 7,
                _ => 6,
            };
            let absent = route == 1 && sample == 3;
            result.response = Some(Response {
                id,
                generation: result.endpoint.clone().unwrap(),
                phase_before: phase,
                phase_after: if id == 12 || id == 24 { 6 } else { phase },
                authority_floor: if id >= 22 { 4 } else { 1 },
                value: if absent {
                    vec![]
                } else {
                    vec![sample + if route == 1 { 4 } else { 0 }]
                },
                error: if absent {
                    "section not found".into()
                } else {
                    String::new()
                },
            });
        }
        f.results.push(result);
    }
    let fixture_bytes = b"synthetic verifier-control bytes; not a database fixture";
    fs::write(
        f.path.join("expected-typed-sections.bincode"),
        fixture_bytes,
    )
    .unwrap();
    let fixture_pin = hex(&Sha256::digest(fixture_bytes));
    f.edit("response-oracle.json", |v| {
        v["fixture_sha256"] = json!(&fixture_pin)
    });
    f.edit("owned-interruption.json", |v| {
        v["fixture_sha256"] = json!(&fixture_pin)
    });
    f.oracle_pin = hex(&Sha256::digest(
        fs::read(f.path.join("response-oracle.json")).unwrap(),
    ));
    f
}
fn reject(f: &Fixture, reason: &str) {
    let report = f.audit();
    assert!(!report.accepted, "negative accepted");
    assert!(
        report.failures.iter().any(|v| v.contains(reason)),
        "{reason}: {:?}",
        report.failures
    );
}
#[test]
fn complete_schedule_passes_and_partitions_interruption() {
    let f = fixture();
    let r = f.audit();
    assert!(r.accepted, "{:?}", r.failures);
    assert_eq!(r.issued, 40);
    assert_eq!(r.interrupted_inflight, 1);
    assert_eq!(r.scheduled_during_outage, 1);
}
#[test]
fn asynchronous_result_order_is_valid() {
    let mut f = fixture();
    f.results.reverse();
    assert!(f.audit().accepted);
}
#[test]
fn missing_scheduled_tick_is_rejected() {
    let mut f = fixture();
    f.issued.remove(18);
    reject(&f, "missing issued schedule ticks");
}
#[test]
fn missing_completion_is_rejected() {
    let mut f = fixture();
    f.results.remove(18);
    reject(&f, "no outcome");
}
#[test]
fn duplicate_completion_is_rejected() {
    let mut f = fixture();
    f.results[18] = f.results[17].clone();
    reject(&f, "duplicate completed ID");
}
#[test]
fn unissued_completion_is_rejected() {
    let mut f = fixture();
    let mut row = f.results[18].clone();
    row.id = 999;
    f.results.push(row);
    reject(&f, "never issued");
}
#[test]
fn issue_order_cannot_hide_a_stall() {
    let mut f = fixture();
    f.issued.swap(17, 18);
    reject(&f, "issued ID ordering");
}
#[test]
fn scheduled_deadline_includes_dispatch_delay() {
    let mut f = fixture();
    f.results[18].dispatch_ns += DEADLINE;
    f.issued[18].dispatch_ns += DEADLINE;
    f.results[18].start_ns += DEADLINE;
    f.results[18].end_ns += DEADLINE;
    reject(&f, "scheduled deadline missed 18");
}
#[test]
fn reset_scheduled_time_is_rejected() {
    let mut f = fixture();
    f.results[18].scheduled_ns += DEADLINE;
    reject(&f, "immutable request changed");
}
#[test]
fn wrong_payload_is_rejected() {
    let mut f = fixture();
    f.results[18].response.as_mut().unwrap().value = vec![99];
    reject(&f, "typed response mismatch");
}
#[test]
fn wrong_generation_is_rejected() {
    let mut f = fixture();
    f.results[18].response.as_mut().unwrap().generation = "forged".into();
    reject(&f, "response identity mismatch");
}
#[test]
fn expired_before_kill_cannot_be_excluded() {
    let mut f = fixture();
    f.edit("owned-interruption.json", |v| {
        v["candidate_inflight_ids"] = json!([18, 20])
    });
    let r = &mut f.results[18];
    r.response = None;
    r.transport_error = "reset".into();
    r.end_ns = 10_200_000_000;
    reject(&f, "scheduled deadline missed 18");
}
#[test]
fn unobserved_inflight_cannot_be_excluded() {
    let f = fixture();
    f.edit("owned-interruption.json", |v| {
        v["candidate_inflight_ids"] = json!([])
    });
    reject(&f, "availability transport failure 20");
}
#[test]
fn corrupt_response_during_outage_is_still_rejected() {
    let mut f = fixture();
    let mut response = f.results[22].response.clone().unwrap();
    response.id = 21;
    response.generation = "second".into();
    response.value = vec![99];
    f.issued[21].endpoint = Some("second".into());
    f.results[21].endpoint = Some("second".into());
    f.results[21].response = Some(response);
    f.results[21].transport_error.clear();
    reject(&f, "typed response mismatch 21");
}
#[test]
fn authority_floor_regression_is_rejected() {
    let f = fixture();
    f.edit("owned-interruption.json", |v| {
        v["resumed_authority_floor"] = json!(0)
    });
    reject(&f, "authority floor regression");
}
#[test]
fn external_build_pin_is_required() {
    let f = fixture();
    f.flush();
    let r = verify(&f.path, &"04".repeat(32), &"02".repeat(32), &f.oracle_pin);
    assert!(!r.accepted);
}
#[test]
fn stop_truncation_is_rejected() {
    let f = fixture();
    f.edit("schedule-stop.json", |v| v["stop_ns"] = json!(41 * PERIOD));
    reject(&f, "stop does not cover");
}
#[test]
fn no_crash_is_not_a_crash_pass() {
    let f = fixture();
    f.edit("owned-interruption.json", |v| {
        v["killed_signal"] = json!(15)
    });
    reject(&f, "SIGKILL");
}
#[test]
fn unknown_observation_fields_are_rejected() {
    let f = fixture();
    f.flush();
    let p = f.path.join("parent-issued.jsonl");
    let raw = fs::read_to_string(&p)
        .unwrap()
        .replacen("{", "{\"ignored_failure\":true,", 1);
    fs::write(p, raw).unwrap();
    let r = verify(&f.path, &"01".repeat(32), &"02".repeat(32), &f.oracle_pin);
    assert!(!r.accepted);
    assert!(r.failures[0].contains("unknown field"));
}
#[test]
fn held_consumer_is_not_exempt_from_deadline() {
    let mut f = fixture();
    f.results[24].end_ns += DEADLINE;
    reject(&f, "held consumer failed");
}
#[test]
fn no_real_activation_overlap_is_rejected() {
    let f = fixture();
    f.edit("resume/cutover-overlap.json", |v| {
        v["markers"]["finish_invocation_ns"] = json!(127_000_000)
    });
    reject(&f, "do not overlap");
}
#[test]
fn failed_activation_is_rejected() {
    let f = fixture();
    f.edit("resume/cutover-overlap.json", |v| {
        v["actual_finish_succeeded"] = json!(false)
    });
    reject(&f, "did not complete");
}
#[test]
fn post_write_bookkeeping_delay_does_not_invent_parent_latency() {
    let f = fixture();
    f.edit("resume/cutover-overlap.json", |v| {
        v["markers"]["response_end_ns"] = json!(200_000_000)
    });
    assert!(f.audit().accepted);
}

#[test]
fn actual_callback_hold_must_fit_in_measured_rpc() {
    let f = fixture();
    f.edit("resume/cutover-overlap.json", |v| {
        v["markers"]["callback_release_ns"] = json!(150_000_000);
        v["markers"]["response_end_ns"] = json!(200_000_000);
        v["markers"]["finish_return_ns"] = json!(180_000_000);
    });
    reject(&f, "exceeds measured parent");
}

#[test]
fn zero_length_marker_receipt_does_not_prove_overlap() {
    let f = fixture();
    f.edit("resume/cutover-overlap.json", |v| {
        for field in [
            "request_held_ns",
            "finish_invocation_ns",
            "callback_release_ns",
            "response_end_ns",
            "finish_return_ns",
        ] {
            v["markers"][field] = json!(0);
        }
    });
    reject(&f, "do not overlap");
}

#[test]
fn activation_at_callback_release_is_not_overlap() {
    let f = fixture();
    f.edit("resume/cutover-overlap.json", |v| {
        v["markers"]["finish_invocation_ns"] = v["markers"]["callback_release_ns"].clone();
    });
    reject(&f, "do not overlap");
}

#[test]
fn sequential_floor_regression_inside_global_bounds_is_rejected() {
    let mut f = fixture();
    f.results[23].response.as_mut().unwrap().authority_floor = 9;
    reject(&f, "floor regressed across non-overlapping");
}

#[test]
fn overlapping_reply_order_is_not_a_false_floor_regression() {
    let mut f = fixture();
    f.results[23].response.as_mut().unwrap().authority_floor = 9;
    f.results[23].end_ns = f.results[24].end_ns + 1;
    for row in &mut f.results[25..] {
        row.response.as_mut().unwrap().authority_floor = 9;
    }
    assert!(
        f.audit().accepted,
        "overlapping reads are allowed to observe different current revisions"
    );
}

#[test]
fn retained_fixture_mutation_is_rejected() {
    let f = fixture();
    fs::write(f.path.join("expected-typed-sections.bincode"), b"changed").unwrap();
    reject(&f, "typed fixture hash mismatch");
}

#[test]
fn oracle_rewrite_cannot_be_self_approved_after_the_run() {
    let f = fixture();
    f.edit("response-oracle.json", |v| v["old_source"][0] = json!([99]));
    reject(&f, "pre-campaign oracle pin mismatch");
}
#[test]
fn extra_held_consumers_are_rejected() {
    let mut f = fixture();
    f.issued[25].consumer_hold = true;
    f.results[25].consumer_hold = true;
    reject(&f, "exactly one");
}
