#!/usr/bin/env bash
# Offline replay: 0=verified original outcomes; 1=mismatch; 2=incomplete.
set -u
package=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P) || exit 2
cd "$package" || exit 2
incomplete() { printf 'INCOMPLETE: %s\n' "$*" >&2; exit 2; }
failed() { printf 'FAILED: %s\n' "$*" >&2; exit 1; }
for tool in sha256sum find sort cmp mktemp rustup grep cc; do
  command -v "$tool" >/dev/null 2>&1 || incomplete "required tool $tool is unavailable"
done
[[ -f SHA256SUMS ]] || incomplete 'SHA256SUMS is missing'
while IFS= read -r entry; do
  [[ "$entry" =~ ^[0-9a-f]{64}'  '.+ ]] || failed 'malformed package manifest'
  name=${entry:66}
  [[ "$name" != /* && "$name" != *\\* && "$name" != '..' && "$name" != ../* && "$name" != */../* && "$name" != */.. ]] || failed 'unsafe package manifest path'
  [[ -f "$name" ]] || incomplete "required file $name is missing"
  [[ ! -L "$name" ]] || failed "linked file $name"
done < SHA256SUMS
sha256sum --check --status SHA256SUMS || failed 'package hash mismatch'
rustup run 1.94.1 cargo --version >/dev/null 2>&1 || incomplete 'Rust 1.94.1 toolchain is not installed'
rustup run 1.94.1 rustc --version >/dev/null 2>&1 || incomplete 'Rust 1.94.1 compiler is unavailable'
out=$(mktemp -d /tmp/8db-p11-public-replay.XXXXXX) || incomplete 'cannot create external replay directory'
printf 'Replay output: %s\n' "$out"
find . -type f ! -path './SHA256SUMS' -printf '%P\n' | LC_ALL=C sort > "$out/actual-files.txt"
while IFS= read -r entry; do printf '%s\n' "${entry:66}"; done < SHA256SUMS | LC_ALL=C sort > "$out/expected-files.txt"
cmp -s "$out/actual-files.txt" "$out/expected-files.txt" || failed 'package has missing or unlisted files'
rustup run 1.94.1 cargo test --offline --locked --manifest-path verify/Cargo.toml --target-dir "$out/target" -- --test-threads=1 > "$out/controls.stdout" 2> "$out/controls.stderr" || failed "auditor controls failed; see $out"
grep -Fq 'test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;' "$out/controls.stdout" || failed 'auditor control inventory changed'
rustup run 1.94.1 cargo build --offline --locked --manifest-path verify/Cargo.toml --target-dir "$out/target" > "$out/build.stdout" 2> "$out/build.stderr" || failed "auditor build failed; see $out"
auditor="$out/target/debug/p11-availability-audit"
"$auditor" r5/evidence 503b24adb008a918dfaecb2358181c0880dcc6a61bc1b9c7cf4409cc4d9d076b 3cf0b545cf042343aedf50e4bd317580425d4fb437af9ab275f4e5243fbf1df8 29003532af97e7a114acb8df2e76963aefa7007b9f09e578215ec58a3f9e9516 > "$out/r5-report.json" 2> "$out/r5.stderr"
r5_exit=$?
printf '%s\n' "$r5_exit" > "$out/r5.exit"
[[ "$r5_exit" == 1 ]] || failed 'R5 must retain its original rejection'
cmp -s "$out/r5-report.json" r5/original-audit-report.json || failed 'R5 replay differs from the original report'
"$auditor" r6/evidence dc16933f75a04fe3af6baaad3a4dcbe4454a9ef485156eadd79395ef363faee7 2d92663bb69608ecc911d9f2a08bf807775982e4b3f5a8360202df1a870a753b 6fada689973c19dc9ed7772f9f96ba2c9c67c3902802dfc4cd05b79671770135 > "$out/r6-report.json" 2> "$out/r6.stderr"
r6_exit=$?
printf '%s\n' "$r6_exit" > "$out/r6.exit"
[[ "$r6_exit" == 0 ]] || failed 'R6 acceptance did not reproduce'
cmp -s "$out/r6-report.json" r6/original-audit-report.json || failed 'R6 replay differs from the original report'
replay_supplement() {
  local label=$1 source_pin=$2 engine_pin=$3 oracle_pin=$4 expected_exit=$5 actual_exit
  local evidence="four-cpu-2026-09-16/$label"
  "$auditor" "$evidence/evidence" "$source_pin" "$engine_pin" "$oracle_pin" > "$out/$label-report.json" 2> "$out/$label.stderr"
  actual_exit=$?
  printf '%s\n' "$actual_exit" > "$out/$label.exit"
  [[ "$actual_exit" == "$expected_exit" ]] || failed "$label original verdict did not reproduce"
  cmp -s "$out/$label-report.json" "$evidence/original-audit-report.json" || failed "$label replay differs from the original report"
}
replay_supplement r6-offline-206f dc16933f75a04fe3af6baaad3a4dcbe4454a9ef485156eadd79395ef363faee7 2d92663bb69608ecc911d9f2a08bf807775982e4b3f5a8360202df1a870a753b a9fb77cfbde8a13d2c9f520f4a5cc28042538900d1514334671a9122db6e2dfc 1
replay_supplement completion-028 69cdbd07f480a2de06d5773e69327b23136038b6144e03771691cb66eb9844be 0271554acc27b1d45a637573bbc8f252c34c9e353c8404ec448dca0867c4b922 9e1a022bae9d3f93ca63619239680370e9250a23b57ec3a018ed9201e88f257b 1
replay_supplement copy-41ca 1b719c4cf7defc3e483893f1541a5623a983830e74c44c026b22de481faa3745 690bfa7ca3bdfd2df4f69eae539b7959d6a23d5b21c54e46e78664c53350a3ee 13f7291bfed95790105acb237401d4b4dfaccade53e1bf8806c6e7cf2eb4dd82 1
replay_supplement diagnostic-9c5 c824699b1c523699e1d9751d29a8d7ddcc713644d072372090280695b5a263d2 fecb52a1364e8dbd575e5f2f91361276cb7afd494d486ce3d218f8347f4c3694 7cfa70ea2b3d5aa049c6237f9d263cf4556c8ff4925542a96c1619d031d57831 0
sha256sum --check --status SHA256SUMS || failed 'package changed during replay'
printf 'VERIFIED: 31 controls passed; all six original reports reproduced byte-for-byte (four rejected, two accepted).\n'
