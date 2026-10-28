# Development progress

This is the handoff log for developers and future coding sessions. Update it
after each small, completed step. Requirements and behavioral decisions belong
in [v1.md](v1.md); this file records actual progress and evidence.

Dates use the project's Asia/Dhaka timezone.

## Current position

- **Completed:** initial requirements, contributor guide, MIT license, and progress
  tracking documentation; pinned phonetic reference; first Rust conversion slice;
  `a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo`/`OI`/`OU`/`rri` vowel forms.
  Basic `r`/`rr`, `p`/`ph`/`f`, `c`/`ch`/`j`/`jh`, `d`/`dh`/`D`/`Dh`, and `l`/`h`
  consonant matching is implemented. There are 1,359 reference-checked cases.
- **Implementation:** a dependency-free library and terminal example exist.
  Principal forms for all 11 basic vowels and 28 consonant sequences (26 letters)
  exist.
  Full vowel context, remaining consonants, conjuncts, composition, Fcitx5
  integration, and a usable desktop keyboard remain incomplete. Full Avro
  compatibility is not established.
- **Next:** add basic `s`/`sh`/`S` for স/শ/ষ, checking exact Unicode, case
  distinctions, and longest-sequence matching against the pinned reference.
- **Version history:** Git is available. Each completed step is committed after
  checks and the progress update. Initial entries below describing unavailable
  Git refer to the earlier workspace state.

## Next small implementation step

1. Independently author basic `s`/`sh`/`S` cases alone, with supported vowels/aliases,
   and around neighboring input. Check case distinctions and `sh` priority over
   single `s`, keeping conjunct and escape rules separate.
2. Run them through the [pinned reference](v1.md#phonetic-behavior-reference)
   using the external parser, recording exact Unicode output before implementing.
3. Add only this basic consonant group. Keep incomplete consonants, conjuncts,
   reph, and other context behavior explicit; no composition/platform work.
4. Extend regression fixtures, run the README checks, update documentation, and
   commit the completed increment.

**Done when:** the selected `s`/`sh`/`S` cases match the reference, existing conversion
cases still pass, limitations are documented, and this log records the results.
Fcitx5 integration and composition controls belong to later steps.

## Completed steps

### 2026-10-28 — Basic l/h consonants

- Added lowercase `l` → ল (U+09B2) and standalone `h` → হ (U+09B9). Both
  establish consonant context for supported vowels. The core now implements
  28 consonant input sequences producing 26 distinct letters.
- Authored 110 [fixtures](../tests/fixtures/consonants-l-h.tsv), verified against
  the pinned parser before Rust implementation. They cover all supported
  vowels/aliases on either side, following-vowel transitions, digits, boundaries,
  copied Unicode, and words. All 1,359 cases match the SHA-256-verified external
  parser under Node.js v24.21.0. No upstream source, rules, or fixtures imported.
- Checked all ten existing two-key consonants with an extra `h`/`ha`; longest
  matching still consumes `kh` as খ, followed by standalone হ in `khh` → `খহ`.
  Moved now-supported `phh`/`chh`/`dhh`/`Dhh` cases into reference fixtures and
  updated partial uppercase/escape outputs to reflect converted `h`. Escapes
  remain unsupported: a backtick still appears in `` d`h `` → `` দ`হ ``.
- Extended Unicode boundary, separate-call, and long-buffer regressions. Updated
  [README.md](../README.md) and [v1.md](v1.md#basic-l-and-h-consonants) with exact
  Unicode, examples, current counts, and remaining gaps, including affected
  earlier gap tables. No dependencies, composition, or platform code added.
- Checks: `cargo fmt --all -- --check` passed after applying rustfmt;
  `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline`
  (17 regression tests and one documentation example), and
  `cargo build --offline --release` passed. The reference comparison passed all
  1,359 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'lal holud bhalO'` printed
  `লাল হলুদ ভালো`; 20 direct CLI checks passed for new letters, vowels,
  two-key matching, boundaries, words, and documented partial outputs. Reused
  inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools installed or upgraded.
  The cached external reference remained outside the repository.
- Documentation/tool checks: `node --check`, UTF-8 decoding, 108 local Markdown
  links/anchors across four documents, final newlines, trailing whitespace,
  fixture counts, and `git diff --check` passed. The pre-existing untracked
  `.vscode/` directory was left untouched.
- Limitations: `ll`/`hl` and other conjuncts, uppercase `L`/`H` and other aliases,
  remaining consonants, conjunct/reph forms, punctuation/escapes, composition,
  and platform integration remain incomplete. No full compatibility,
  performance, or platform validation claimed.
- Next: add basic `s`/`sh`/`S` for স/শ/ষ as a separate consonant increment after
  reference checks for case distinctions and longest-sequence matching.

### 2026-10-28 — Basic d/dh/D/Dh consonants

- Added exact `d` → দ (U+09A6), `dh` → ধ (U+09A7), `D` → ড (U+09A1), and
  `Dh` → ঢ (U+09A2). The existing longest-match rule consumes `dh`/`Dh` before
  their single-key prefixes. All four establish consonant context for supported
  vowels. There are now 26 consonant input sequences producing 24 letters.
- Authored 164 [fixtures](../tests/fixtures/consonants-d-dh.tsv) covering each
  pattern, all supported vowels/aliases before and after it, following-vowel
  transitions, case distinctions, digits, boundaries, copied Unicode, and words.
  The initial reference check caught a word example requiring uppercase `O`:
  corrected `dokan` to `dOkan` for `দোকান`. Reference comparison then passed before
  Rust implementation. The first Rust run caught two word examples using the
  unsupported uppercase `A`; changed `dhAn`/`DAn` to `dhan`/`Dan` and reran both
  checks. All 1,249 final cases match the SHA-256-verified external pinned parser
  under Node.js v24.21.0. No upstream source, rules, or fixtures imported.
- Extended long-buffer and separate-call regressions and documented temporary
  limits for conjuncts, uppercase-`H` aliases, standalone `h`, and escapes.
  Updated [README.md](../README.md) and
  [v1.md](v1.md#basic-d-dh-d-and-dh-consonants) with Unicode, examples, current
  counts, and gaps. No dependencies, composition, or platform code added.
- Checks after correcting the examples: `cargo test --offline` passed (16
  regression tests and one documentation example);
  `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  1,249 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'din dhan Dan Dhaka'`
  printed `দিন ধান ডান ঢাকা`; 20 direct CLI checks passed for new patterns,
  case distinctions, vowel signs, boundaries, words, and documented gaps.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools installed
  or upgraded. The cached external reference remained outside the repository.
- Documentation/tool checks: `node --check`, UTF-8 decoding, 101 local Markdown
  links/anchors across four documents, final newlines, trailing whitespace, and
  `git diff --check` passed. The pre-existing untracked `.vscode/` directory was
  left untouched.
- Limitations: `ddh`, `DDh`, `dr`, `dH`/`DH`, standalone `h`, remaining consonants,
  conjunct/reph forms, uppercase vowel aliases, punctuation/escapes, composition,
  and platform integration remain incomplete. No full compatibility,
  performance, or platform validation claimed.
- Next: add basic `l`/`h` for ল/হ as a separate consonant increment after checking
  reference behavior and interactions with existing two-key consonants.

### 2026-10-28 — Basic c/ch/j/jh consonants

- Added lowercase `c` → চ (U+099A), `ch` → ছ (U+099B), `j` → জ (U+099C), and
  `jh` → ঝ (U+099D). The existing longest-match rule handles `ch` and `jh` before
  single keys. All four establish consonant context for supported vowel signs.
  There are now 22 consonant input sequences producing 20 distinct letters.
- Authored 148 [fixtures](../tests/fixtures/consonants-c-ch-j-jh.tsv), checked
  against the pinned parser before Rust changes. Cases cover each pattern alone,
  all supported vowels/aliases before and after it, digits, boundaries, copied
  Unicode, and words. All 1,085 reference cases match with the SHA-256-verified
  external parser under Node.js v24.21.0. No upstream source, rules, or fixtures
  imported.
- Extended long-buffer and separate-call checks and recorded temporary limits
  for conjuncts, uppercase variants, standalone `h`, and escapes. Reference `Jh`
  produces জহ, distinct from `jh` → ঝ; no blanket case normalization added.
  Updated [README.md](../README.md) and
  [v1.md](v1.md#basic-c-ch-j-and-jh-consonants) with exact Unicode, current counts,
  examples, and remaining gaps. No dependencies or platform code added.
- Checks: `cargo test --offline` passed (15 regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  1,085 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'cabi chabi jani jhuri'`
  printed `চাবি ছাবি জানি ঝুরি`; 20 additional CLI checks passed for the new
  patterns, vowel signs, boundaries, words, and documented partial outputs.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, 94 local Markdown
  links/anchors across four files, final newlines, trailing whitespace, and
  `git diff --check` passed. The pre-existing untracked `.vscode/` directory was
  left untouched.
- Limitations: `cc`, `cch`, `jjh`, `cr`, uppercase variants, standalone `h`,
  remaining consonants, conjunct/reph forms, punctuation/escapes, composition,
  and platform integration remain incomplete. No full compatibility or
  performance/platform validation claimed.
- Next: add basic `d`/`dh`/`D`/`Dh` for দ/ধ/ড/ঢ as a separate consonant increment.

### 2026-10-28 — Basic p/ph/f consonants

- Added lowercase `p` → প (U+09AA), and `ph`/`f` → ফ (U+09AB). The existing
  longest-match rule consumes `ph` before `p`; both spellings of ফ establish
  identical consonant context (`phi` and `fi` → `ফি`). There are now 18 consonant
  input sequences producing 16 distinct letters.
- Authored 115 [fixtures](../tests/fixtures/consonants-p-ph-f.tsv), checked against
  the pinned parser before Rust changes. Cases cover the three patterns with all
  supported vowels/aliases before and after them, digits, boundaries, copied
  Unicode, and words. All 937 reference cases match with the SHA-256-verified
  external parser under Node.js v24.21.0. No upstream source, rules, or fixtures
  imported.
- Extended long-buffer and separate-call checks and recorded temporary limits
  for uppercase aliases, conjuncts, overlapping `pph`, and interrupted/escaped
  input. Updated [README.md](../README.md) and
  [v1.md](v1.md#basic-p-ph-and-f-consonants) with exact Unicode, current counts,
  examples, and remaining gaps. No dependencies or platform code added.
- Checks: `cargo test --offline` passed (14 regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  937 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'pani phOn fOn'` printed
  `পানি ফোন ফোন`; 19 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: `pr`, `pp`, `pph`, uppercase `P`/`F` and other aliases, standalone
  `h`, remaining consonants, conjunct/reph forms, punctuation/escapes, composition,
  and platform integration remain incomplete. No full compatibility or
  performance/platform validation claimed.
- Next: add basic `c`/`ch`/`j`/`jh` for চ/ছ/জ/ঝ as a separate consonant increment.

### 2026-10-28 — Basic r/rr consonants

- Added `r` → র (U+09B0) and basic `rr` → রর (U+09B0 U+09B0). The longest
  consonant match selects `rr` before `r`; the existing three-key `rri` still
  takes precedence over either. There are now 15 consonant input sequences
  producing 14 distinct letters.
- Resolved the prior lowercase-run gaps: `rrri` → `রররি`, `rrrri` → `ররৃ`.
  `rrI` → `ররী` stays distinct from `rri` → `ঋ`. Added 115 independently authored
  [fixtures](../tests/fixtures/consonants-r.tsv), verified against the pinned
  parser before Rust changes. Cases cover vowels/aliases, overlapping/repeated
  runs, basic `r` before supported consonants, boundaries, Unicode, and words.
- All 822 reference cases match with the SHA-256-verified external parser under
  Node.js v24.21.0. Updated temporary-limit tests and documented remaining conjunct,
  reph, and uppercase-`R` gaps in [v1.md](v1.md#basic-r-and-rr-consonants).
  Extended long-buffer and separate-call checks; updated [README.md](../README.md).
  No upstream source, rules, fixtures, dependencies, or platform code imported.
- Checks: `cargo test --offline` passed (13 regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  822 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'ram rani rrri rrrri'` printed
  `রাম রানি রররি ররৃ`; 22 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: `kr`, `rrk`, bare `rrr`, and other contextual/conjunct/reph forms
  still differ from the reference. Uppercase `R`, other remaining consonants,
  uppercase aliases, punctuation/escapes, composition, and platform integration
  remain incomplete. No full compatibility or performance/platform validation
  claimed.
- Next: add basic `p`/`ph`/`f` for প/ফ as a separate consonant increment.

### 2026-10-28 — rri vowel group

- Added exact lowercase `rri` before individual consonants: independent ঋ
  (U+098B), or dependent ৃ (U+09C3) after a supported consonant. Retained final
  raw `i` context (`rria` → `ঋয়া`, `rrio` → `ঋও`). The principal forms for all
  11 basic vowels now exist, with broader context/alias compatibility still partial.
- Authored 84 [fixtures](../tests/fixtures/vowels-rri.tsv), verified against the
  pinned parser before Rust changes. They cover all 13 supported consonants,
  surrounding vowels/aliases, repetition, transitions, digits, boundaries, Unicode,
  and words. All 707 reference cases match with the SHA-256-verified external
  parser under Node.js v24.21.0. No upstream source, rules, or fixtures imported.
- Extra reference probes exposed longer-`r`-run behavior tied to the still-missing
  `r`/`rr` consonant matching. Documented current outputs (`rrri` → `rঋ`,
  `rrrri` → `rrঋ`) as gaps for the next increment rather than introducing a
  temporary vowel-boundary restriction. Added regressions that make these
  temporary limits, case variants, and literal backticks explicit.
- Extended boundary, separate-call, and long-buffer tests to `rri`. Updated
  [README.md](../README.md) and [v1.md](v1.md#seventh-vowel-group) for the implemented
  forms, exact Unicode, count, examples, and remaining gaps. No dependencies,
  composition controls, or platform code added.
- Checks: `cargo test --offline` passed (12 regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  707 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'rri krri krriti rriN'` printed
  `ঋ কৃ কৃতি ঋণ`; 14 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools or dependencies
  were installed or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: standalone `r`/`rr`, their case variants, other remaining consonants,
  uppercase `A`/`E`, contextual/conjunct rules, punctuation/escapes, composition,
  and platform integration remain incomplete. No full compatibility, performance,
  or platform validation claimed.
- Next: implement basic `r`/`rr` and resolve the overlapping lowercase runs.

### 2026-10-28 — OI/OU two-key vowel group

- Added exact uppercase `OI`/`OU` before single `O`: independent ঐ/ঔ
  (U+0990/U+0994), or dependent ৈ/ৌ (U+09C8/U+09CC) after a supported consonant.
  Case variants remain distinct (`OI` → `ঐ`, `Oi` → `ওই`). Retained the final raw
  key (`I`/`U`) in conversion context for following vowels, including `OIa` →
  `ঐয়া` and `OIo` → `ঐও`.
- Authored 150 [fixtures](../tests/fixtures/vowels-oi-ou.tsv) and verified expected
  output/code points against the pinned parser before Rust implementation. They
  cover both pairs after all 13 consonants, neighboring vowels/aliases, longer
  transitions, case variants, digits, boundaries, copied Unicode, and words.
  All 623 reference cases match using the SHA-256-verified external parser under
  Node.js v24.21.0. No upstream source, rules, or fixtures imported.
- Extended boundary, separate-call, long-buffer, and literal-backtick regressions
  to both pairs; removed their obsolete temporary-limit expectations. Updated
  [README.md](../README.md) and [v1.md](v1.md#sixth-vowel-group) with current
  behavior, exact Unicode, count, examples, and remaining gaps.
- Checks: `cargo test --offline` passed (11 regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  623 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'OI kOI OU kOU nOUka'` printed
  `ঐ কৈ ঔ কৌ নৌকা`; 19 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools or dependencies
  were installed or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: `rri`, uppercase `A`/`E`, remaining consonants, other phonetic
  sequences, conjuncts, punctuation/escapes, composition, and platform integration
  remain incomplete. No full compatibility, performance, or platform validation
  claimed.
- Next: add `rri` for the remaining basic vowel ঋ and its sign ৃ.

### 2026-10-28 — Contextual o/O and oo vowel group

- Added lowercase `o`: independent অ (U+0985), no added character immediately
  after a supported consonant, and ও (U+0993) after other supported raw vowel
  keys. Added uppercase `O`: independent ও or dependent ো (U+09CB). Added greedy
  `oo` as short `u`: independent উ or dependent ু (U+09C1).
- Context now keeps the raw vowel key, including `a` and the ends of repeated-key
  aliases: `ao` → `আও`, `eeo` → `ঈও`, `ooo` → `উঅ`. Even when `o` adds no character,
  it establishes vowel context (`koO` → `কও`, `koa` → `কয়া`). Context remains
  conversion-local; copied characters and digits reset it.
- Authored 156 [fixtures](../tests/fixtures/vowels-o.tsv), checked against the
  pinned parser before Rust implementation. They cover all 13 consonants with
  all three forms, neighboring vowels/aliases, longer runs, boundaries, Unicode,
  and words. All 473 reference cases match with the SHA-256-verified external
  parser under Node.js v24.21.0. No upstream source, rules, or fixtures imported.
- Extended boundary, separate-call, long-buffer, and literal-backtick regressions.
  Replaced the temporary `kemon` gap expectation with its reference-checked result
  `কেমন`. Kept the two-key vowels `OI`/`OU` explicitly incomplete for the next step.
- Updated [README.md](../README.md) and [v1.md](v1.md#fifth-vowel-group) with exact
  Unicode, raw-context behavior, examples, current count, and remaining gaps.
  No dependencies, composition controls, or platform code added.
- Checks: `cargo test --offline` passed (ten regression tests and one documentation
  example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  473 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'tumi kemon tOmake'` printed
  `তুমি কেমন তোমাকে`; 18 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: `OI`/`OU`, uppercase `A`/`E`, other phonetic sequences, unsupported
  consonants, conjuncts, punctuation/escapes, composition, and platform integration
  remain incomplete. No full compatibility, performance, or platform validation
  claimed.
- Next: add the `OI`/`OU` two-key vowels after checking their reference behavior.

### 2026-10-28 — Contextual e and ee vowel group

- Added lowercase `e`: independent এ (U+098F), dependent ে (U+09C7) after a
  supported consonant. Added greedy `ee` matching as an alias for `I`: independent
  ঈ and dependent ী. An `a` after either form produces য়া with U+09DF, so
  `kea` → `কেয়া`, `keea` → `কীয়া`. Longer runs include `eee` → `ঈএ`.
- Authored 92 [fixtures](../tests/fixtures/vowels-e.tsv) for both sequences after
  all 13 supported consonants, neighboring vowels, longer runs, digits,
  boundaries, Unicode, and words. The initial reference check passed; Rust's
  first test run exposed an example outside this slice (`kemon` needs `o`).
  Replaced that example with `ami ke`, kept `o` explicitly unsupported, and
  rechecked the final set against the reference. All 317 cases match under
  Node.js v24.21.0, using the SHA-256-verified external parser.
- Extended boundary, separate-call, long-buffer, and literal-backtick regressions
  to `e`/`ee`. The two-key match advances only after an ASCII `ee` prefix, keeping
  UTF-8 boundaries intact. Existing consonant and vowel cases continue to pass.
- Updated [README.md](../README.md) and [v1.md](v1.md#fourth-vowel-group) with
  behavior, exact Unicode, examples, current fixture count, and remaining gaps.
  No dependencies, upstream source/rules/fixtures, or platform code imported.
- Checks after correcting the example: `cargo test --offline` passed (nine
  regression tests and one documentation example);
  `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  317 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'ami ke kee'` printed
  `আমি কে কী`; 15 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: uppercase `A`/`E`, other vowels, unsupported consonants, conjuncts,
  phonetic punctuation/escapes, composition, and platform integration remain
  incomplete. No full compatibility, performance, or platform validation claimed.
- Next: add the `o`/`O` vowel group after checking the reference's context rules.

### 2026-10-28 — Uppercase I vowel group

- Added uppercase `I`: independent ঈ (U+0988), and dependent ী (U+09C0) after a
  supported consonant. Kept `i`/`I` distinct (`ki` → `কি`, `kI` → `কী`) and
  extended the existing following-`a` behavior (`kIa` → `কীয়া`, using U+09DF).
- Authored 52 [new fixtures](../tests/fixtures/vowels-i-long.tsv), covering `I`
  with all 13 supported consonants, vowel pairs and longer transitions, digits,
  boundaries, Unicode, and words. Expected output and code points were verified
  against the pinned parser before Rust implementation.
- Added the fixtures to both Rust and reference runners; all 225 cases match the
  SHA-256-verified external reference under Node.js v24.21.0. Extended boundary,
  separate-call, long-buffer, and literal-backtick regressions to `I`. Removed
  the obsolete temporary-limit expectation that uppercase `I` remains Latin.
- Updated [README.md](../README.md) and [v1.md](v1.md#third-vowel-group) for current
  behavior, exact Unicode, examples, count, and remaining gaps. No dependencies
  or upstream source, rules, or fixtures were imported.
- Checks: `cargo test --offline` passed (eight regression tests and one
  documentation example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  225 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'ki kI nIti'` printed
  `কি কী নীতি`; nine additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: uppercase `A`, remaining vowels, unsupported consonants, conjuncts,
  phonetic punctuation/escapes, composition, and platform integration remain
  incomplete. No full compatibility, performance, or platform validation claimed.
- Next: add the `e` vowel group after checking its context rules against the
  pinned reference.

### 2026-10-28 — Case-sensitive u/U vowel group

- Added `u`/`U`: independent উ/ঊ (U+0989/U+098A), and dependent ু/ূ
  (U+09C1/U+09C2) after supported consonants. Extended the existing `i` context so
  `a` after either new vowel also produces য়া: `kua` → `কুয়া`, `kUa` → `কূয়া`.
  Preserved case-sensitive spelling (`bhumi` → `ভুমি`, `bhUmi` → `ভূমি`).
- Authored 80 [new fixtures](../tests/fixtures/vowels-u.tsv), including both vowels
  after all 13 supported consonants, vowel pairs, transitions with `a`/`i`,
  repetition, digits, boundaries, Unicode, and words. Expected output/code points
  were checked against the pinned parser before implementing Rust changes.
- Extended the Rust and reference fixture runners: all 173 cases match the
  SHA-256-verified external jsAvroPhonetic parser under Node.js v24.21.0. No
  upstream code, rules, fixtures, runtime dependencies, or platform code imported.
- Extended boundary and per-call reset regressions to the new vowels. Updated
  temporary-limit tests now that `u` is supported, including literal backticks;
  the long-buffer correctness test now includes both `u`/`U` and a following `a`.
- Updated [README.md](../README.md) usage/status and
  [v1.md](v1.md#second-vowel-group) for the new behavior, exact Unicode, current
  fixture count, and remaining gaps. Previous slice descriptions remain historical.
- Checks: `cargo test --offline` passed (seven regression tests and one
  documentation example); `cargo fmt --all -- --check`,
  `cargo clippy --offline --all-targets -- -D warnings`, and
  `cargo build --offline --release` passed. The reference comparison passed all
  173 cases and reported five remaining gap probes.
- Example: `cargo run --offline --example convert -- 'tumi bhUmi kua'` printed
  `তুমি ভূমি কুয়া`; 11 additional documented conversions passed direct CLI checks.
  Reused inspected Rust/Cargo 1.99.1 and Node.js v24.21.0; no tools were installed
  or upgraded.
- Documentation/tool checks: `node --check`, UTF-8 decoding, local Markdown links
  and heading anchors, final newlines, trailing whitespace, and `git diff --check`
  passed. The pre-existing untracked `.vscode/` directory was left untouched.
- Limitations: uppercase `A`/`I`, other vowels, unsupported consonants, conjuncts,
  phonetic punctuation/escapes, composition, and platform integration remain
  incomplete. No full compatibility, performance, or platform validation claimed.
- Next: add uppercase `I` with its own reference cases and case-sensitive checks.

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
