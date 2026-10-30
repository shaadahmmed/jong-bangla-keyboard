#!/usr/bin/env python3
"""Stage an experimental install; check discovery and removal without a daemon."""

import os
from pathlib import Path
import subprocess
import sys
import tempfile


def run(args, *, env=None, expect_success=True):
    result = subprocess.run(
        [str(arg) for arg in args], env=env, capture_output=True, text=True,
        timeout=120, check=False,
    )
    if (result.returncode == 0) != expect_success:
        sys.stderr.write(result.stdout + result.stderr)
        sys.exit(f"FAIL: unexpected exit {result.returncode}: {args[0]}")
    return result


def require(condition, message):
    if not condition:
        sys.exit(f"FAIL: {message}")


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: python3 tools/check-install.py BUILD_DIRECTORY")
    build = Path(sys.argv[1]).resolve()
    destinations = (build / "install-layout.txt").read_text().splitlines()
    require(len(destinations) == 2, "expected addon and package-data destinations")
    for destination in destinations:
        path = Path(destination)
        require(path.is_absolute() and ".." not in path.parts,
                "installation destinations must be absolute without '..'")

    with tempfile.TemporaryDirectory(prefix="jong-install-check-") as directory:
        stage = Path(directory)
        addon_dir, data_dir = [
            stage / Path(path).relative_to("/") for path in destinations
        ]
        library = addon_dir / "libjong-bangla.so"
        metadata = data_dir / "addon/jong-bangla.conf"
        input_method = data_dir / "inputmethod/jong-bangla.conf"
        expected = {library, metadata, input_method}
        env = dict(os.environ, DESTDIR=str(stage))
        run(["cmake", "--install", build], env=env)
        actual = {path for path in stage.rglob("*") if path.is_file()}
        require(actual == expected,
                f"expected exactly three installation files; found {sorted(actual)}")
        require("Library=libjong-bangla\n" in metadata.read_text(),
                "installed metadata must use the relocatable library name")
        require(str(build) not in metadata.read_text(),
                "installed metadata must not contain the build path")
        require("Addon=jong-bangla\n" in input_method.read_text(),
                "input-method registration must match the addon identifier")
        print("PASS: exactly three staged files; metadata has no build path", flush=True)

        harness = [build / "jong-fcitx5-harness", "--installed", data_dir, addon_dir]
        result = run(harness)
        print(f"PASS: staged loading: {result.stdout.strip()}", flush=True)
        # An absent staged library must fail, even while the build-local copy
        # still exists. This detects fallback to a developer/system installation.
        hidden = library.with_suffix(".so.hidden")
        library.rename(hidden)
        try:
            result = run(harness, expect_success=False)
            require("Fcitx5 harness failed: load Jong addon from configured library path"
                    in result.stderr,
                    f"missing-library control failed for another reason: {result.stderr}")
        finally:
            hidden.rename(library)
        print("PASS: missing staged library rejected; no build/system fallback", flush=True)

        sentinel = addon_dir / "unrelated-addon.keep"
        sentinel.write_text("unrelated file must remain\n")
        for path in expected:
            path.unlink()
        remaining = {path for path in stage.rglob("*") if path.is_file()}
        require(remaining == {sentinel}, "removal must retain unrelated files")
        print("PASS: removed only Jong's three files; unrelated sentinel retained", flush=True)


if __name__ == "__main__":
    main()
