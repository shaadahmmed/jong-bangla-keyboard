# Development progress

This is the handoff log for developers and future coding sessions. Update it
after each small, completed step. Requirements and behavioral decisions belong
in [v1.md](v1.md); this file records actual progress and evidence.

Dates use the project's Asia/Dhaka timezone.

## Current position

- **Completed:** initial requirements, contributor guide, MIT license, and progress
  tracking documentation; phonetic behavior reference selected and pinned.
- **Implementation:** not started. No engine code, Rust workspace, Fcitx5 addon,
  or runnable keyboard exists yet.
- **Next:** create the first small Rust conversion slice described below and
  compare its independently authored cases against the pinned reference.
- **Version history:** verified on 2026-10-28: this workspace is now a Git
  repository, with `238b329` (`initial commit`) as its latest commit at inspection.
  The initial documentation entries below describe the earlier workspace state.

## Next small implementation step

1. Use the [pinned jsAvroPhonetic v1.1.4 reference](v1.md#phonetic-behavior-reference)
   to check independently authored examples. Record the conversion entry point,
   invocation, runtime version, and exact Unicode results. Do not import another
   engine or its rule tables into the project.
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

### 2026-10-28 — Phonetic reference and development tools

- Selected jsAvroPhonetic v1.1.4, commit
  `d3790172ebff92d43f0294bc51fed1fca7c80039`, as the deterministic conversion
  reference. Documented its scope, MPL-1.1 provenance, and comparison requirements
  in [v1.md](v1.md#phonetic-behavior-reference). No upstream source or data imported.
- Recorded the user's preference for the latest stable tools where practical in
  [AGENTS.md](../AGENTS.md). Continue explaining each small step before edits.
- Added the user's commit-per-step workflow: after checks and the progress update,
  commit each completed increment with a descriptive message and only its changes.
- Tools inspected: distro-provided `rustc 1.99.1`, `cargo 1.99.1`,
  `rustfmt 1.10.0`, and `clippy 0.1.99`. The official Rust
  [latest release announcement](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)
  confirms 1.99.1 as current stable on the inspection date. `rustup` is absent;
  no tools were installed or upgraded. A minimum Rust version is not yet set.
- Reference checks: official OmicronLab project link, upstream Git tag identifiers,
  and pinned README/package metadata verified. Direct sandbox network access
  failed DNS resolution; approved read-only network commands verified the tag
  and metadata. Reference conversion has not been executed.
- Git checks: repository root and latest commit verified; working tree was clean
  before this documentation increment. Corrected the current status above.
- Documentation checks: local Markdown links and heading anchors, UTF-8 decoding,
  final newlines, and trailing whitespace passed across all four Markdown files;
  `git diff --check` passed. Reviewed the diff for requirement/status consistency.
- Limitations: no Rust library, executable conversion tests, compatibility result,
  performance result, or platform validation yet.
- Next: build the dependency-free Rust conversion slice, list supported sequences,
  compare authored fixtures with the reference, and document actual Cargo checks.

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
