# Development progress

This is the handoff log for developers and future coding sessions. Update it
after each small, completed step. Requirements and behavioral decisions belong
in [v1.md](v1.md); this file records actual progress and evidence.

Dates use the project's Asia/Dhaka timezone.

## Current position

- **Completed:** initial requirements, contributor guide, MIT license, and progress
  tracking documentation; pinned phonetic reference; first Rust conversion slice;
  lowercase `a`/`i` vowels with context. There are 93 reference-checked cases.
- **Implementation:** a dependency-free library and terminal example exist.
  Other vowels, conjuncts, composition, Fcitx5 integration, and a usable desktop
  keyboard are not implemented. Full Avro compatibility is not established.
- **Next:** add the `u`/`U` vowel group, checking its independent forms, vowel
  signs, case sensitivity, and neighboring vowels against the pinned reference.
- **Version history:** Git is available. Each completed step is committed after
  checks and the progress update. Initial entries below describing unavailable
  Git refer to the earlier workspace state.

## Next small implementation step

1. Independently author `u`/`U` cases for word-initial vowels, vowels after
   supported consonants, repeated vowels, and interaction with existing `a`/`i`.
2. Run them through the [pinned reference](v1.md#phonetic-behavior-reference)
   using the external parser, recording exact Unicode output before implementing.
3. Add the required context handling to the Rust core without platform or
   composition state. Keep remaining coverage gaps explicit.
4. Extend regression fixtures, run the README checks, update documentation, and
   commit the completed increment.

**Done when:** the `u`/`U` cases match the reference, existing conversion
cases still pass, limitations are documented, and this log records the results.
Fcitx5 integration and composition controls belong to later steps.

## Completed steps

### 2026-10-28 — First contextual vowel group

- Added lowercase `a` and `i`: independent আ/ই, dependent া/ি after a supported
  consonant, and য়া after `i` (`kia` → `কিয়া`). Context uses matched input tokens
  during a single conversion; digits and unmatched characters reset it. Existing
  Bangla and other copied Unicode do not establish consonant context.
- Authored 66 new [vowel fixtures](../tests/fixtures/vowels.tsv) with exact Unicode
  sequences, and checked them against the pinned parser before implementing the
  Rust rules. The updated reference checker runs these plus the initial 27 cases:
  all 93 pass, using the SHA-256-verified external parser with Node.js v24.21.0.
  No upstream code, rules, or fixtures were added to the repository.
- Added regressions for vowel adjacency, all 13 supported consonants with both
  vowels, boundaries, mixed Unicode, and separate conversion calls. Updated
  earlier temporary-limit tests where `a` is now supported. Fixture readers now
  reject files with no cases, avoiding an empty fixture passing silently.
- Updated [README.md](../README.md) examples and [v1.md](v1.md#first-vowel-group)
  behavior/limits. Kept the original conversion-slice description as history,
  explicitly marking `a` and `ka` gaps as resolved by this increment.
- Checks: `cargo test --offline` passed (six regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. `node --check` passed; the reference
  comparison passed all 93 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'ami khani kia'` printed
  `আমি খানি কিয়া`; six additional documented conversions passed direct CLI checks.
  Rust/Cargo 1.99.1 and Node.js v24.21.0 were inspected and reused; no dependencies
  or tools were installed.
- Documentation checks: UTF-8, local links/heading anchors, final newlines,
  trailing whitespace, and `git diff --check` passed. The pre-existing untracked
  `.vscode/` directory was left outside this step.
- Limitations: uppercase `A`/`I`, other vowels, unsupported-consonant context,
  conjuncts, punctuation conversion, and escapes remain incomplete. The precise
  reference sequence for inserted `য়` is U+09DF; no normalization is performed.
  Composition, platform support, and performance remain unvalidated.
- Next: implement `u`/`U` as the next small vowel group after reference checks.

### 2026-10-28 — First Rust conversion slice

- Added the `jong-bangla-core` library, Rust 2024 edition, with Rust/Cargo 1.99.1
  as the initially validated baseline. No external dependencies; unsafe code is
  forbidden. Added a terminal example and ignored Cargo's generated `target/`.
- Implemented all ten Bangla digits and 13 independently authored consonant
  sequences: `k`, `kh`, `g`, `gh`, `t`, `th`, `T`, `Th`, `n`, `N`, `b`, `bh`, `m`.
  Matching selects the longest supported sequence at each position. Unmatched
  characters pass through at valid UTF-8 boundaries; no input is retained/logged.
- Added 27 authored fixtures with explicit Unicode code points. All matched
  `OmicronLab.Avro.Phonetic.parse(input)` from the pinned reference, executed with
  Node.js v24.21.0. The parser and its license were downloaded into a temporary
  directory outside the repository; no upstream source/table/fixture was added
  to the project. The comparison tool verifies the parser's SHA-256 first.
- Documented reproducible build, example, and reference-comparison commands in
  [README.md](../README.md); recorded supported sequences and five observed gaps
  in [v1.md](v1.md#first-conversion-slice).
- Checks: `cargo fmt --all -- --check` passed after applying rustfmt;
  `cargo clippy --offline --all-targets -- -D warnings` passed;
  `cargo test --offline` passed (four regression tests and one documentation
  example); `cargo build --offline --release` passed. Regression coverage includes
  the 27 fixtures, empty input, mixed Unicode/combining marks, literal unsupported
  characters, temporary coverage limits, and a repeated long input buffer.
- Example checks: `cargo run --offline --example convert -- 'kh12T'` printed
  `খ১২ট`; three further direct example invocations matched expected output.
  Missing/extra arguments returned usage and exit status 2. `node --check` passed;
  the reference comparison passed all 27 cases and rejected a mismatched source
  file before evaluation. Five reference gap probes were recorded in v1.md.
- Documentation checks: UTF-8, local links/heading anchors, final newlines,
  trailing whitespace, and `git diff --check` passed. Rust 1.99.1 was rechecked
  against the official latest stable announcement; no tool installation occurred.
- Limitations: vowels, conjuncts, broader case handling, phonetic punctuation and
  escapes remain absent. Passing these examples does not establish arbitrary-word
  compatibility. The long-buffer test checks correctness, not performance.
  No composition/controller/platform tests or resource measurements have run.
- Next: implement the small vowel group described above, keeping this increment
  separate from composition and platform integration.

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
