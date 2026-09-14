#!/usr/bin/env python3
"""Read-only verification of the published 8DB evidence artifacts."""

import argparse
import hashlib
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys

MANIFEST = "ARTIFACT-SHA256SUMS"
RAW_SUFFIXES = {".txt", ".json", ".bin", ".der", ".hex"}
RUNS = ("x86_64-windows_2026-09-11", "aarch64-linux_2026-09-12")
WINDOWS_DEVICES = {"CON", "PRN", "AUX", "NUL"} | {
    prefix + str(number) for prefix in ("COM", "LPT") for number in range(1, 10)
}


def eligible(path):
    return path.suffix.lower() in RAW_SUFFIXES or path.name == "SHA256SUMS"


def linked(path):
    return path.is_symlink() or getattr(path, "is_junction", lambda: False)()


def artifact_path(root, name):
    parts = name.split("/")
    if (not name or any(p in ("", ".", "..") for p in parts)
            or any(p.lower() == ".git" for p in parts)
            or any(p.endswith((".", " ")) or p.split(".")[0].upper() in WINDOWS_DEVICES for p in parts)
            or any(ord(c) < 32 or ord(c) == 127 for c in name)
            or any(c in name for c in '\\<>:"|?*')
            or PurePosixPath(name).is_absolute()):
        raise ValueError("unsafe manifest path: " + repr(name))
    path = root
    for part in parts:
        path = path / part
        if linked(path):
            raise ValueError("linked artifact path: " + repr(name))
    try:
        path.resolve().relative_to(root)
    except (ValueError, OSError) as exc:
        raise ValueError("artifact escapes repository: " + repr(name)) from exc
    return path


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def raw_artifacts(root):
    found = set()
    def walk_error(error):
        raise error

    for directory, dirs, files in os.walk(root, followlinks=False, onerror=walk_error):
        dirs[:] = [d for d in dirs if d.lower() != ".git"]
        for name in dirs:
            path = Path(directory) / name
            artifact_path(root, path.relative_to(root).as_posix())
        for name in files:
            path = Path(directory) / name
            if eligible(path):
                relative = path.relative_to(root).as_posix()
                artifact_path(root, relative)
                found.add(relative)
    return found


class Results:
    def __init__(self):
        self.statuses = []

    def report(self, status, check, detail):
        self.statuses.append(status)
        print("{} {}: {}".format(status, check, detail))

    def finish(self):
        code = 1 if "FAIL" in self.statuses else 2 if "INCONCLUSIVE" in self.statuses else 0
        status = ("PASS", "FAIL", "INCONCLUSIVE")[code]
        print("{} overall: published-artifact verification".format(status))
        return code


def verify_manifest(root, results):
    try:
        manifest = artifact_path(root, MANIFEST)
        entries = {}
        for number, line in enumerate(manifest.read_text(encoding="utf-8").splitlines(), 1):
            if not line or line.startswith("#"):
                continue
            match = re.fullmatch(r"([0-9a-f]{64})  (.+)", line)
            if not match:
                raise ValueError("invalid manifest syntax at line {}".format(number))
            expected, name = match.groups()
            path = artifact_path(root, name)
            if not eligible(path):
                raise ValueError("manifest entry is outside the raw-artifact scope: " + name)
            if name in entries:
                raise ValueError("duplicate manifest entry: " + name)
            entries[name] = expected
        if not entries:
            raise ValueError("empty artifact manifest")
        actual = raw_artifacts(root)
    except (OSError, UnicodeError, ValueError) as exc:
        results.report("FAIL", "manifest", str(exc))
        return False

    missing = sorted(set(entries) - actual)
    unlisted = sorted(actual - set(entries))
    if missing or unlisted:
        results.report("FAIL", "manifest completeness",
                       "missing={}; unlisted={}".format(missing, unlisted))
    else:
        results.report("PASS", "manifest completeness", "{} raw artifacts".format(len(entries)))

    failures = []
    for name, expected in entries.items():
        try:
            if digest(artifact_path(root, name)) != expected:
                failures.append(name + " (SHA256 mismatch)")
        except (OSError, ValueError) as exc:
            failures.append(name + " (" + str(exc) + ")")
    if failures:
        results.report("FAIL", "artifact hashes", "; ".join(failures))
    else:
        results.report("PASS", "artifact hashes", "{} SHA256 hashes match".format(len(entries)))
    return not (missing or unlisted or failures)


def compare_secrets(root, results):
    for run in RUNS:
        for direction in ("A_kem_8db_keygen", "B_kem_openssl_keygen"):
            check = run + "/" + direction
            try:
                folder = root / "interop" / run / direction
                left = (folder / "ss_8db.bin").read_bytes()
                right = (folder / "ss_openssl.bin").read_bytes()
                passed = len(left) == len(right) == 32 and left == right
                results.report("PASS" if passed else "FAIL", check,
                               "published 32-byte ML-KEM shared secrets " +
                               ("match" if passed else "differ or have incorrect size"))
            except OSError as exc:
                results.report("FAIL", check, str(exc))


def run_openssl(executable, arguments):
    return subprocess.run([executable] + arguments, shell=False,
                          stdin=subprocess.DEVNULL, capture_output=True,
                          text=True, encoding="utf-8", errors="replace", timeout=20)


def verify_signatures(root, requested, results):
    executable = shutil.which(requested)
    problem = None
    if executable is None:
        problem = "OpenSSL executable unavailable; use --openssl PATH"
    else:
        try:
            version = run_openssl(executable, ["version"])
            match = re.match(r"OpenSSL (\d+)\.(\d+)\.", version.stdout)
            if version.returncode or not match or tuple(map(int, match.groups())) < (3, 5):
                problem = "OpenSSL 3.5 or newer is required"
            else:
                results.report("PASS", "OpenSSL availability", version.stdout.strip())
        except (OSError, subprocess.SubprocessError) as exc:
            problem = "OpenSSL could not run: " + str(exc)
    if problem:
        for run in RUNS:
            results.report("INCONCLUSIVE", run + "/ML-DSA valid and negative controls", problem)
        return

    for run in RUNS:
        folder = root / "interop" / run
        positive = folder / "C_dsa_8db_sign"
        signature = positive / "sig_8db.bin"
        negative = folder / "E_negative" / "sig_8db_flipped.bin"
        arguments = ["pkeyutl", "-verify", "-pubin", "-keyform", "DER",
                     "-inkey", str(positive / "vk_8db.der"), "-rawin",
                     "-in", str(positive / "msg.bin"), "-sigfile"]
        try:
            valid = run_openssl(executable, arguments + [str(signature)])
            message = valid.stdout + valid.stderr
            passed = valid.returncode == 0 and "Signature Verified Successfully" in message
            if not passed:
                status = "FAIL" if "Signature Verification Failure" in message else "INCONCLUSIVE"
                results.report(status, run + "/ML-DSA valid signature",
                               "OpenSSL exit {}: {}".format(valid.returncode, message.strip()))
                results.report("INCONCLUSIVE", run + "/ML-DSA flipped signature",
                               "requires a successful positive control")
                continue
            results.report("PASS", run + "/ML-DSA valid signature", "OpenSSL verifies the 8DB signature")
            original, modified = signature.read_bytes(), negative.read_bytes()
            altered = len(original) == len(modified) and original != modified
            if not altered:
                results.report("FAIL", run + "/ML-DSA flipped signature",
                               "negative artifact must be an altered copy of the same length")
                continue
            invalid = run_openssl(executable, arguments + [str(negative)])
            message = invalid.stdout + invalid.stderr
            if invalid.returncode == 0:
                results.report("FAIL", run + "/ML-DSA flipped signature", "modified signature was accepted")
            elif "Signature Verification Failure" in message:
                results.report("PASS", run + "/ML-DSA flipped signature", "OpenSSL rejects the modified signature")
            else:
                results.report("INCONCLUSIVE", run + "/ML-DSA flipped signature",
                               "OpenSSL exit {}: {}".format(invalid.returncode, message.strip()))
        except (OSError, subprocess.SubprocessError) as exc:
            results.report("INCONCLUSIVE", run + "/ML-DSA verification", str(exc))


def main():
    parser = argparse.ArgumentParser(
        description="Check published artifact hashes, recorded ML-KEM secret pairs and ML-DSA signatures.",
        epilog="Read-only. Exit 0: all checks pass; 1: artifact/check defect; 2: checks incomplete. "
               "Engine execution, KAT reruns and database migration require the separate evaluator package.")
    parser.add_argument("--openssl", default="openssl", metavar="PATH",
                        help="OpenSSL 3.5+ executable (default: openssl on PATH)")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    print("8DB published-artifact and interoperability verification")
    results = Results()
    if verify_manifest(root, results):
        compare_secrets(root, results)
        verify_signatures(root, args.openssl, results)
    else:
        results.report("INCONCLUSIVE", "interoperability checks", "repair manifest/artifact defects first")
    return results.finish()


if __name__ == "__main__":
    sys.exit(main())
