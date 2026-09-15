//! Public artifact verification only. No 8DB implementation or private material.
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384};
use std::{collections::HashSet, fs, path::{Component, Path, PathBuf}};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn check(ok: bool, message: &str) -> Result<()> { if ok { Ok(()) } else { Err(message.into()) } }
fn hex(bytes: impl AsRef<[u8]>) -> String { bytes.as_ref().iter().map(|x| format!("{x:02x}")).collect() }
fn json(path: &Path) -> Result<Value> { Ok(serde_json::from_slice(&fs::read(path)?)?) }
fn array(v: &Value) -> Result<&Vec<Value>> { v.as_array().ok_or_else(|| "array required".into()) }
fn u(v: &Value) -> Result<u64> { v.as_u64().ok_or_else(|| "unsigned integer required".into()) }
fn s(v: &Value) -> Result<&str> { v.as_str().ok_or_else(|| "string required".into()) }
fn exact_line(text: &str, expected: &str) -> Result<()> {
    let prefix = format!("{}=", expected.split_once('=').ok_or("wrapper key required")?.0);
    check(text.lines().filter(|line| line.starts_with(&prefix)).count() == 1 && text.lines().any(|line| line == expected), "missing, duplicate or conflicting wrapper fact")
}
fn main() {
    if let Err(error) = verify() { eprintln!("REFUSED: {error}"); std::process::exit(1); }
}
fn verify() -> Result<()> {
    let root = PathBuf::from(std::env::args_os().nth(1).unwrap_or_else(|| "..".into())).canonicalize()?;
    let publication = json(&root.join("PUBLICATION-MANIFEST.json"))?;
    let mut listed = HashSet::new();
    for entry in array(&publication["files"])? {
        let name = s(&entry["path"])?;
        check(!name.is_empty() && name.bytes().all(|c| c.is_ascii_alphanumeric() || b"/._-".contains(&c)) && Path::new(name).components().all(|c| matches!(c, Component::Normal(_))), "unsafe publication path")?;
        check(listed.insert(name.to_owned()), "duplicate publication member")?;
        let path=root.join(name);
        check(!fs::symlink_metadata(&path)?.file_type().is_symlink() && path.canonicalize()?.starts_with(&root), "linked artifact or escaped publication root")?;
        let bytes = fs::read(path)?;
        check(bytes.len() as u64 == u(&entry["bytes"])? && hex(Sha256::digest(&bytes)) == s(&entry["sha256"])?, "published bytes or SHA256 differ")?;
    }
    let originals = root.join("originals");
    let required = ["a-final.json","b-final.json","b-open-first.json","b-open-rejoin.json","a-wrapper.txt","b-wrapper.txt","a-test.txt","b-test.txt","a-clock.jsonl","b-clock.jsonl","wire-audit.jsonl","expected-manifest.json","expected-sections.jsonl"];
    for name in required { check(listed.contains(&format!("originals/{name}")), "required original missing from publication")?; }
    for entry in fs::read_dir(&originals)? { let entry=entry?; check(entry.file_type()?.is_file() && listed.contains(&format!("originals/{}",entry.file_name().to_string_lossy())), "unlisted original")?; }
    for name in ["summary.json","negative-history.json","ORIGINAL-PROVENANCE.json"] { check(listed.contains(name), "required metadata missing")?; }
    let summary = json(&root.join("summary.json"))?;
    check(summary["source"] == "7db11180d1a71f7d95cd9698c72ffd1f044d1e6e" && summary["elf_sha256"] == "9596da2abf37ee0e22c714ecf407db138acfe4153fcbd96093b779f911377009", "wrong qualified source or ELF")?;
    check(summary["build"]["test_opt_level"] == 2 && summary["build"]["debug_assertions"] == true && summary["build"]["overflow_checks"] == true, "qualified build profile differs")?;
    let expected = json(&originals.join("expected-manifest.json"))?;
    let rows = array(&expected["rows"])?;
    let sections = fs::read_to_string(originals.join("expected-sections.jsonl"))?;
    let lines: Vec<_> = sections.lines().collect();
    check(rows.len()==512 && lines.len()==512 && expected["count"]==512, "expected cardinality differs")?;
    let mut aggregate = Sha384::new(); aggregate.update(b"8DB/required-physical/manifest/v1\0");
    for (i, line) in lines.iter().enumerate() {
        let mut hash = Sha384::new(); hash.update(b"8DB/required-physical/section/v1\0");
        hash.update((i as u64).to_le_bytes()); hash.update((line.len() as u64).to_le_bytes()); hash.update(line.as_bytes());
        let hash=hex(hash.finalize());
        check(rows[i] == serde_json::json!([i,hash]), "independent serialized row digest differs")?;
        aggregate.update((i as u64).to_le_bytes()); aggregate.update(hash.as_bytes());
    }
    let digest=hex(aggregate.finalize());
    check(digest=="4ba387c2eabf6995bf9b9d61111d2b5c0f09f4106aece1a02b181d91414843e759501fa8af07620c9e93ccb7fb7123c9" && expected["expected_sha384"]==digest, "independent aggregate differs")?;
    let a=json(&originals.join("a-final.json"))?; let b=json(&originals.join("b-final.json"))?;
    for (role,m) in [("a",&a),("b",&b)] {
        check(m["run"]==expected["run"] && m["role"]==role && m["expected_records"]==512, "final identity differs")?;
        check(m["complete"]==true && m["drained"]==true && m["exact_key_closure"]==true, "final incomplete or not drained")?;
        check(m["actual_records"]==expected["rows"] && m["actual_sha384"]==digest && m["expected_sha384"]==digest && array(&m["missing"])?.is_empty(), "full typed closure differs")?;
        check(u(&m["commit"])?==513 && m["commit"]==m["applied"], "commit/apply not drained")?;
        check(m["profile"]["channel_suite"]=="Compliance" && m["profile"]["native_at_rest_cipher"]=="AWS-LC AES-256-GCM" && m["profile"]["native_kdf"]=="HKDF-SHA384" && m["profile"]["module_validation_claim"]==false, "profile or attribution differs")?;
        check(m["physical"]["authenticated_sections"]==512 && m["physical"]["committed_sections"]==512 && m["physical"]["legacy_sections"]==0 && m["physical"]["legacy_indexes"]==0 && m["physical"]["anchor_matches_snapshot"]==true, "native physical scope differs")?;
        let actor_wrapper=fs::read_to_string(originals.join(format!("{role}-wrapper.txt")))?;
        exact_line(&actor_wrapper,"wrapper_exit=0")?;
        exact_line(&actor_wrapper,&format!("final_pid={}",u(&m["pid"]) ?))?;
        check(m["phase"] == if role=="a" {"first"} else {"rejoin"}, "final phase differs")?;
        check(fs::read_to_string(originals.join(format!("{role}-test.txt")))?.contains("test result: ok. 1 passed; 0 failed; 0 ignored;"), "actual native test did not pass")?;
        let clock:Vec<Value>=fs::read_to_string(originals.join(format!("{role}-clock.jsonl")))?.lines().map(serde_json::from_str).collect::<std::result::Result<_,_>>()?;
        check(clock.len()==2 && clock[0]["event"]=="start" && clock[1]["event"]=="complete" && clock[0]["duration_seconds"]==270, "clock incomplete")?;
        check(u(&clock[1]["elapsed_ns"])? >=270_000_000_000 && u(&clock[1]["samples"])? >0 && clock[1]["system_time_backwards"]==0 && clock[1]["instant_backwards"]==0, "clock regressed or incomplete")?;
        check(u(&clock[0]["wall_ns"])? /1_000_000 <=1789493160000 && u(&clock[1]["end_wall_ns"])? /1_000_000 >=u(&m["utc_ms"])?, "clock coverage differs")?;
    }
    let first=json(&originals.join("b-open-first.json"))?; let reopened=json(&originals.join("b-open-rejoin.json"))?;
    check(first["data_dir"]==reopened["data_dir"] && first["authority_dir"]==reopened["authority_dir"] && first["pid"]!=reopened["pid"], "same-store different-process reopen missing")?;
    check(first["run"]==expected["run"] && reopened["run"]==expected["run"] && first["phase"]=="first" && reopened["phase"]=="rejoin" && first["role"]=="b" && reopened["role"]=="b" && reopened["pid"]==b["pid"], "reopen identity differs")?;
    check(first["start_ms"]==1789493160000u64 && reopened["start_ms"]==1789493160000u64 && reopened["end_ms"]==1789493400000u64 && reopened["quiesce_ms"]==1789493370000u64, "reopen schedule differs")?;
    let retained=array(&reopened["initial_records"])?; let missing=array(&reopened["initial_missing"])?; let mut ids=HashSet::new();
    check(retained.len()==172 && retained.len()+missing.len()==512, "retained partition differs")?;
    for row in retained { let id=u(&row[0])? as usize; check(id<512 && ids.insert(id) && *row==rows[id], "retained row differs")?; }
    for id in missing { let id=u(id)? as usize; check(id<512 && ids.insert(id), "missing IDs overlap or duplicate")?; }
    check(ids.len()==512 && b["rejoin_deadline_ms"]==30000 && u(&b["rejoin_new_content_ms"])?==1074, "deadline/observed recovery differs")?;
    let wrapper=fs::read_to_string(originals.join("b-wrapper.txt"))?;
    for fact in ["killed_exit=137".to_string(),"process_absent=true".to_string(),"down_s=15".to_string(),"kill_at_ms=1789493205000".to_string(),format!("first_pid={}",u(&first["pid"])?),format!("final_pid={}",u(&reopened["pid"])?) ] { exact_line(&wrapper,&fact)?; }
    let wire:Vec<Value>=fs::read_to_string(originals.join("wire-audit.jsonl"))?.lines().map(serde_json::from_str).collect::<std::result::Result<_,_>>()?;
    check(wire.len()==4, "wire report incomplete")?; let mut wire_bytes=0; let mut wire_frames=0;
    for row in wire { check(row["invalid"]==0 && row["plaintext_fixture_markers"]==0 && row["terminal_partial_bytes"]==0, "wire report failed")?;wire_bytes+=u(&row["bytes"])?;wire_frames+=u(&row["declared_compliance_raft_frames"])?; }
    check(wire_bytes==14_969_769 && wire_frames==2236, "wire accounting differs")?;
    let history=json(&root.join("negative-history.json"))?;
    check(history["history"][0]["a_observed"]==293 && history["history"][0]["b_observed"]==293 && history["history"][1]["a_wrapper_exit"]==143 && history["history"][1]["b_actor_created"]==false, "negative history changed")?;
    println!("{}",serde_json::json!({"result":"PASS_PUBLISHED_ARTIFACTS","published_files":listed.len(),"rows_per_host":512,"retained_rows":172,"rejoin_ms":1074,"wire_frames":wire_frames,"scope":"Checks this published fixed-corpus record; does not execute 8DB or independently attest the hosts"}));
    Ok(())
}
