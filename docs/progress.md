# Development progress

This is the handoff log for developers and future coding sessions. Update it
after each small, completed step. Requirements and behavioral decisions belong
in [v1.md](v1.md); this file records actual progress and evidence.

Dates use the project's Asia/Dhaka timezone.

## Current position

- **Completed:** initial requirements, contributor guide, MIT license, and progress
  tracking documentation.
- **Implementation:** not started. No engine code, Rust workspace, Fcitx5 addon,
  or runnable keyboard exists yet.
- **Next:** choose and document the phonetic behavior reference, then implement
  the first small Rust conversion slice described below.
- **Version history:** `git status --short` reports that this workspace is not a
  Git repository. No commit history has been verified. This log records progress
  but does not provide Git version history.

## Next small implementation step

1. Select and record an Avro behavior reference/version. Use it to establish
   expected typing behavior, while writing our own engine. Do not import another
   engine or its rule tables as part of this step.
2. Create a minimal Rust library with no external runtime dependencies for this
   first slice. Document the chosen toolchain and actual build/test commands.
3. Implement Bangla digit conversion and a small, explicitly listed set of
   phonetic sequences. Keep unsupported input intact in this initial slice.
   Document that this is partial coverage, not full Avro compatibility.
4. Add independently authored input/output regression cases, run the applicable
   checks, and walk through how matching produces Unicode Bangla output.

**Done when:** the documented examples pass, incomplete behavior is explicit,
the user can run the examples/tests, and this log records the results.
Fcitx5 integration and composition controls belong to later steps.

## Completed steps

### 2026-10-28 — Initial project agreement

- Added [README.md](../README.md), [AGENTS.md](../AGENTS.md),
  [v1.md](v1.md), and the [MIT license](../LICENSE).
- Recorded a from-scratch Rust engine, Linux/Fcitx5 first, CachyOS + KDE Plasma
  on Wayland, editable phonetic composition, and Bangla digits by default.
- Confirmed two choices: converted Bangla first and the original Latin input
  second. Autocorrection remains separate future functionality.
- Clarified that Avro compatibility describes behavior; it does not mean copying
  an existing engine. No engine source has been added.
- Checks: local Markdown targets, UTF-8 decoding, final newlines, trailing
  whitespace, and presence of key confirmed requirements passed.
- Limitations: no executable code or platform tests; compatibility reference,
  detailed controls, and Rust/Fcitx5 interoperability remain unresolved.

### 2026-10-28 — Progress tracking

- Added this log and linked it from the README and contributor guide.
- Required future contributors to record completed changes, actual verification,
  limitations, and the next task after each small step.
- Defined the first implementation increment without starting application code.
- Checks: local Markdown targets, UTF-8 decoding, final newlines, and trailing
  whitespace passed across all project documentation.
- Limitations: documentation tracking only; Git history remains unavailable in
  the current workspace. No Rust or platform checks apply yet.
