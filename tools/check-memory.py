#!/usr/bin/env python3
"""Check synthetic heap ownership; reject an unavailable/broken leak detector."""

import os
from pathlib import Path
import subprocess
import sys


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: python3 tools/check-memory.py SANITIZED_BUILD_DIRECTORY")
    build = Path(sys.argv[1]).resolve()
    env = dict(os.environ)
    env.update(
        ASAN_OPTIONS="detect_leaks=1:halt_on_error=1",
        LSAN_OPTIONS="exitcode=23",
        UBSAN_OPTIONS="halt_on_error=1:print_stacktrace=1",
    )
    control = subprocess.run(
        [str(build / "jong-ffi-memory"), "--leak-control"],
        env=env, capture_output=True, text=True, timeout=120, check=False,
    )
    if (control.returncode != 23 or
            "ERROR: LeakSanitizer: detected memory leaks" not in control.stderr):
        sys.stderr.write(control.stdout + control.stderr)
        sys.exit("FAIL: detector did not identify the intentional Rust session leak")
    print("PASS: detector identified intentional Rust session leak (exit 23)", flush=True)
    for name in ["jong-ffi-memory", "jong-fcitx5-harness"]:
        result = subprocess.run(
            [str(build / name)], env=env, capture_output=True, text=True,
            timeout=120, check=False,
        )
        if result.returncode != 0 or "Sanitizer" in result.stderr:
            sys.stderr.write(result.stdout + result.stderr)
            sys.exit(f"FAIL: {name} (exit {result.returncode})")
        print(f"PASS: {name}; ASan/UBSan/LSan enabled; no reports", flush=True)
        print(result.stdout.strip(), flush=True)


if __name__ == "__main__":
    main()
