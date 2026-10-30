# Jong Bangla

A Bangla phonetic desktop keyboard, built from scratch with Rust.

The working project name is `jong-bangla-board`; **Jong Bangla** is the proposed
display name.

The first release targets **Linux + Fcitx5 on Wayland**. The first test environment
is **CachyOS + KDE Plasma**. IBus and Windows support are planned for later.

## First-release experience

- Avro-compatible phonetic typing, with an independently implemented engine.
- An editable current word; Backspace removes the last phonetic keystroke.
- Ctrl+Backspace clears the active word; otherwise the application handles it.
- Space and Enter commit Bangla, then reach the application on the same press.
  Punctuation stays editable with the word until a boundary.
- Converted Bangla in editable preedit, without a Bangla/Latin choice list for v1.
- Bangla digits by default.
- Fast, lightweight, offline typing.
- Room for modular autocorrection later; no automatic spelling changes or
  predictive suggestions in the initial implementation.

## Current status

A dependency-free Rust library now converts Bangla digits, 35 consonant letters
through 43 basic sequences, lowercase `ng` with selected vowel forms, and
`a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo`/`OI`/`OU`/`rri`
with context, including `rri` after `Ng`/`NG`/`ng`, plus contextual lowercase `y`/`w`/`x`
and uppercase `Z` with selected `aZ`/`oZ` forms. Following-`r` matching after
`q`/`z`/`R`/`Rh`/`g`/`gh`/`k`/`kh`/`Sh`/`y`/`Z`/`Y`/`w`/`x`/`kx`/`kkh`/`kSh`/`Ngx`/`Ngk`/`Ngkh`/`Ngkx`/`Ngkkh`/`NgkSh`/`Ngg`/`Nggh`
is also implemented. Exact
`kx`/`kkh`/`kSh` → `ক্ষ`, `Ngx` → `ঙ্ষ`, `Ngk` → `ঙ্ক`, `Ngkh` → `ঙ্খ`, and
`Ngkx`/`Ngkkh`/`NgkSh` → `ঙ্ক্ষ` are implemented.
Exact `kkhN`/`kShN`/`kxN` → `ক্ষ্ণ` and their selected vowel/following-`r` forms
are also implemented.
Exact `kkhm`/`kShm`/`kxm` → `ক্ষ্ম` and their selected vowel/following-`r` forms
are implemented as well.
Exact `kk` → `ক্ক`, its selected vowel/following-`r` forms, and priority against
existing k-conjuncts are also implemented.
Exact `ShN` → `ষ্ণ` and `Shm` → `ষ্ম`, including selected vowel/following-`r`
forms and the exposed `kkShN`/`kkShm` overlaps, now match as well.
Exact `Ngg` → `ঙ্গ` and `Nggh` → `ঙ্ঘ`, including selected vowel/following-`r`
forms, are implemented as well.
Exact `gg`/`gN`/`NN`/`hN`/`gn`/`ghn`/`hn`/`dg`/`dgh`/`Rg` and following
`r` after basic `h`/`n`/`N` are now implemented, including overlapping nasal matches.
Exact `ngh`/`nn`/`Nn`/`mn`/`hm`/`hl`/`nm`/`Nm`/`mm`/`ml`/`Ngm`/`gm`
and following `r` after basic `m`/`l` are also implemented.
Exact `dd`/`ddh`/`dhn`/`dhm`/`dm`/`dbh`/`dv`/`DD` and following `r` after
basic `d`/`dh`/`D`/`Dh`/`b`/`bh`/`v` are also implemented.
Exact `nd`/`ndh`/`nD`/`ND`/`NDh`/`nt`/`nth`/`nT`/`NT`/`nTh`/`NTh`
and following `r` after basic `t`/`th`/`T`/`Th` are also implemented.
Exact `tt`/`tth`/`tn`/`tm`/`TT`/`Tm`, including selected vowels, following
`r`, and priority against the nasal forms, are also implemented.
Exact `kt`/`kT`/`kl` → ক্ত/ক্ট/ক্ল, including selected vowels, following `r`,
and consuming priority over t-family forms, are implemented. `kn`/`km` remain
separate consonants (কন/কম), matching the reference.
Exact `nk`/`nkh`/`lk`/`sk`/`skh`/`skl`/`Shk`/`ks`, their longer
nonconjunct exceptions `lkh`/`ksh`, and following `r` after basic `s`/`sh`/`S`
are also implemented, including consuming priority over the k/t-family forms.
Exact `gl`/`ll`/`bl`/`pl`/`sl`/`shl`/`bhl`/`phl`/`vl`/`fl`/`Sl`, their
selected vowel/following-`r` forms, and basic `p`/`ph`/`f` following-`r`
attachments are also implemented, including consuming priority over `lk`.
Exact `mb`/`mbh`/`mv`/`mp`/`mph`/`mf`, longer `mpl` (মপ্ল), and
`lm`/`sm`/`shm`/`Sm` are also implemented, including consuming priority and
selected vowel/following-`r` forms.
Exact `lg`/`lb`/`lbh`/`lv`/`lp`/`lT`/`lD`/`ldh`, with longer
`lgh`/`lph` nonconjunct exceptions, are also implemented, including consuming
priority and selected vowel/following-`r` forms.
Exact `sp`/`sph`/`sf`/`spl` and `Shp`/`Shph`/`Shf` are also implemented,
including selected vowel/following-`r` forms and consuming priority. Only `spl`
joins and consumes the final `l`; `sphl`/`sfl`/`Shpl`/`Shphl`/`Shfl` leave
`l` available for a fresh match.
Exact `bb`/`pp` → ব্ব/প্প and `pn`/`sn`/`shn`/`Sn` → প্ন/স্ন/শ্ন/শ্ন
are also implemented, including selected vowels, following `r`, and consuming
priority. Longer `bbh`/`pph` leave `h` available; n-ending forms consume `n`
before `nk` and other nasal rules can reuse it.
Exact `ns`/`ps` → ন্স/প্স and longer `nsh`/`psh` → নশ/পশ are also
implemented, including consuming priority and selected vowel/following-`r` forms.
Exact `st`/`sth`/`sT`/`ShT`/`ShTh`/`sht`/`St` are also implemented,
including consuming priority and selected vowel/following-`r` forms. Longer
`sth`/`ShTh` consume `h`; `sTh`/`shth`/`Sth` leave `h` available.
The 25 remaining exact conjunct spellings identified in the standalone audit
are now implemented together: `pt`/`pT`/`bd`/`bdh`/`bj`, `cc`/`cch`/`cNG`,
`jj`/`jjh`/`jNG`, `gdh`/`mth`, `NGc`/`NGch`/`NGj`/`NGjh`,
`nc`/`nch`/`nj`/`njh`, and `shc`/`shch`/`Sc`/`Sch`.
Basic `c`/`ch`/`j`/`jh` also attach following `r`. Longer matches consume
their keys: `bdhm` → `ব্ধম`, while `pthm` → `প্তহ্ম` leaves `h` available.
See the [combined batch](docs/v1.md#remaining-exact-conjunct-batch) for exact
Unicode, per-spelling evidence, priorities, and remaining context/alias gaps.
The ASCII uppercase letters `ABCEFHKLMPQVWX` now behave like their lowercase
forms throughout matching: `AZ` → `অ্যা`, `kH` → `খ`, `SH` → `ষ`, and
`NgK` → `ঙ্ক`. Other capitals retain their case-sensitive meanings; explicit
`G`/`J` forms now match through explicit rules: `Gh` → `ঘ`, `Gg` → `জ্ঞ`,
`Jh` → `জহ`, while `jh` → `ঝ`. See the [case-alias batch](docs/v1.md#selective-ascii-case-alias-batch)
and [explicit G/J batch](docs/v1.md#explicit-gj-spelling-batch).
Punctuation and symbols now include period/double period/ellipsis, colon, caret,
double comma, dollar, symbol escapes, and standalone backtick removal. See the
[symbol batch](docs/v1.md#punctuation-and-symbol-batch).
All 13 vowel-sign escape spellings and their five A/E case variants now match;
the final backtick resets phonetic context, including when escaped o emits nothing.
See the [vowel-escape batch](docs/v1.md#vowel-sign-escape-batch).
The final t-double-backtick escape now emits the single letter ৎ (U+09CE);
see its [behavior and evidence](docs/v1.md#final-t-escape).
Following `r` after `q`/`z`/`R`/`Rh` now attaches, including their repeated-r
and vowel/ya exceptions; see the [batch record](docs/v1.md#following-r-after-q-z-r-and-rh).
Reph now attaches leading `rr` across basic consonants and existing conjuncts;
see the [reph behavior and evidence](docs/v1.md#reph-batch).
This is a partial engine with 522,933 reference-checked examples,
not a usable desktop keyboard or a full Avro compatibility claim. Further
context auditing remains. The v1 [Unicode case policy](docs/v1.md#unicode-case-preservation-policy)
preserves unsupported characters, with deliberate reference differences;
the platform-independent composition controller is implemented. A minimal Fcitx5
addon now builds and passes a synthetic dispatcher harness. The user reports
successful basic typing/field switching across GTK3, Firefox, KWrite and Kate.
GTK3/Firefox behaved as expected; pending Ctrl+Backspace also works in KWrite.
KWrite/Kate Bangla word-navigation/deletion issues are documented and deferred
for v1 at the user's request. See the [local experiment](#build-and-check-the-fcitx5-experiment).
See the [reference decision](docs/v1.md#phonetic-behavior-reference) and
[initial slice](docs/v1.md#first-conversion-slice) plus the
[first vowel group](docs/v1.md#first-vowel-group) and
[`u`/`U` group](docs/v1.md#second-vowel-group), followed by
[uppercase `I`](docs/v1.md#third-vowel-group) and
[`e`/`ee`](docs/v1.md#fourth-vowel-group) and
[`o`/`O`/`oo`](docs/v1.md#fifth-vowel-group), plus
[`OI`/`OU`](docs/v1.md#sixth-vowel-group) and
[`rri`](docs/v1.md#seventh-vowel-group), followed by
[basic `r`/`rr`](docs/v1.md#basic-r-and-rr-consonants) and
[`p`/`ph`/`f`](docs/v1.md#basic-p-ph-and-f-consonants), then
[`c`/`ch`/`j`/`jh`](docs/v1.md#basic-c-ch-j-and-jh-consonants), followed by
[`d`/`dh`/`D`/`Dh`](docs/v1.md#basic-d-dh-d-and-dh-consonants), then
[`l`/`h`](docs/v1.md#basic-l-and-h-consonants), and the confirmed
[`s`/`sh`/`S`/`Sh` group](docs/v1.md#confirmed-s-sh-s-and-sh-mapping), followed by
[basic `R`/`Rh`](docs/v1.md#basic-uppercase-r-and-rh-consonants), then
[lowercase `z`](docs/v1.md#basic-lowercase-z-consonant), followed by
[basic `Ng`/`NG`](docs/v1.md#basic-ng-and-ng-consonants), then
[lowercase `ng`](docs/v1.md#lowercase-ng-sign-and-vowel-forms), followed by
[`rri` after `Ng`/`NG`/`ng`](docs/v1.md#following-rri-after-ng-ng-and-ng), then
[uppercase `Y`](docs/v1.md#basic-uppercase-y-consonant), followed by
[lowercase `y`](docs/v1.md#contextual-lowercase-y), then
[uppercase `Z`](docs/v1.md#uppercase-z-with-az-and-oz-forms), then
[following `r` after `y`/`Z`](docs/v1.md#following-r-after-lowercase-y-and-uppercase-z), then
[following `r` after `Y`](docs/v1.md#following-r-after-uppercase-y), then
[lowercase `q`](docs/v1.md#basic-lowercase-q-consonant), then
[lowercase `v`](docs/v1.md#basic-lowercase-v-consonant), then
[lowercase `w`](docs/v1.md#contextual-lowercase-w), then
[following `r` after `w`](docs/v1.md#following-r-after-lowercase-w), then
[lowercase `x` and `kx`](docs/v1.md#contextual-lowercase-x-and-exact-kx), then
[following `r` after `x`/`kx`](docs/v1.md#following-r-after-lowercase-x-and-exact-kx), then
[exact `Ngx`](docs/v1.md#exact-ngx-conjunct), then
[exact `Ngk`](docs/v1.md#exact-ngk-conjunct), then
[exact `Ngkh`](docs/v1.md#exact-ngkh-conjunct), then
[exact `Ngkx`](docs/v1.md#exact-ngkx-conjunct), then
[exact `Ngkkh`](docs/v1.md#exact-ngkkh-conjunct), then
[exact `NgkSh`](docs/v1.md#exact-ngksh-conjunct), then
[exact `kkh`](docs/v1.md#exact-kkh-conjunct), then
[exact `kSh`](docs/v1.md#exact-ksh-conjunct), then
[exact `kkhN`/`kShN`/`kxN`](docs/v1.md#exact-kkhn-kshn-and-kxn-conjuncts), then
[exact `kkhm`/`kShm`/`kxm`](docs/v1.md#exact-kkhm-kshm-and-kxm-conjuncts), then
[exact `kk`](docs/v1.md#exact-kk-conjunct), then
[exact `ShN`/`Shm`](docs/v1.md#exact-shn-and-shm-conjuncts), then
[following `r` after `k`/`kh`/`Sh`](docs/v1.md#following-r-after-basic-k-kh-and-sh), then
[following `r` after `g`/`gh`](docs/v1.md#following-r-after-basic-g-and-gh), then
[exact `Ngg`/`Nggh`](docs/v1.md#exact-ngg-and-nggh-conjuncts), then
[the g and nasal conjunct batch](docs/v1.md#g-and-nasal-conjunct-batch), then
[the nasal and h conjunct batch](docs/v1.md#nasal-and-h-conjunct-batch), then
[the d and dh conjunct batch](docs/v1.md#d-and-dh-conjunct-batch), then
[the nasal dental and retroflex batch](docs/v1.md#nasal-dental-and-retroflex-conjunct-batch), then
[the t-family batch](docs/v1.md#t-family-conjunct-batch), then
[the k-family batch](docs/v1.md#k-family-conjunct-batch), then
[the consuming k-family batch](docs/v1.md#consuming-k-family-batch), then
[the l-ending batch](docs/v1.md#l-ending-conjunct-batch), then
[the m-prefix and m-ending batch](docs/v1.md#m-prefix-and-m-ending-conjunct-batch), then
[the consuming l-prefix batch](docs/v1.md#consuming-l-prefix-batch), then
[the s/Sh labial batch](docs/v1.md#s-and-sh-labial-conjunct-batch), then
[the repetition and n-ending batch](docs/v1.md#repetition-and-n-ending-conjunct-batch), then
[the consuming ns/ps batch](docs/v1.md#consuming-ns-and-ps-batch), then
[the s/Sh stop batch](docs/v1.md#s-and-sh-stop-conjunct-batch), and
[the remaining exact conjunct batch](docs/v1.md#remaining-exact-conjunct-batch), then
[selective ASCII case aliases](docs/v1.md#selective-ascii-case-alias-batch), then
[explicit G/J spellings](docs/v1.md#explicit-gj-spelling-batch).

Start with [v1 requirements and development steps](docs/v1.md).
See [the progress handoff](docs/progress.md) for current state, the latest completed
step and checks, and the next task. Detailed change history is in Git.
Rule coverage favors the [largest coherent related batches](docs/v1.md#small-development-steps)
that remain reliable to validate and review. Look for 3–6 related spellings
initially and expand when practical; there is no fixed upper cap. Each spelling
needs independent reference evidence, with one full validation, documentation
update, and commit per completed batch. The prior transliteration step added a
reproducible [mixed-context audit](docs/v1.md#bounded-mixed-context-audit): 83,353 distinct
inputs, no new discrepancies, and the three known Unicode case differences.
This is bounded evidence, not proof of complete compatibility. The earlier
1,350-input vowel-pair audit and 302-input inventory are prior evidence.
The Rust composition controller now supports editable Bangla, raw-key Backspace,
Ctrl+Backspace word deletion, and commit/cancel/reset. Space and Enter commit
before forwarding the same key. See the [controller contract](docs/v1.md#composition-controller-api).
The [adapter interaction contract](docs/v1.md#adapter-interaction-contract) now
settles Escape, navigation, shortcuts and focus/switching: Escape now commits
active Bangla and clears the buffer without forwarding that press; navigation
and shortcuts commit before forwarding; focus/switching finalize
to the original context when permitted, then clear state. Modifier-only presses
preserve composition. The experimental addon now exercises this policy through
Fcitx5's dispatcher with synthetic input. Frontend AltGr/compose handling, mouse
relocation, broader desktop behavior and Unicode case policy remain unverified or
unsettled; see the [feasibility evidence](docs/v1.md#fcitx5-local-feasibility-result).
Developers and coding agents should also read [AGENTS.md](AGENTS.md).

## Library structure

[`src/lib.rs`](src/lib.rs) exposes `jong_bangla_core::transliterate` and the
`Composition`, `CompositionKey`, and `KeyEffect` types.
[`src/composition.rs`](src/composition.rs) owns the editable raw buffer and
converted preedit, plus platform-independent editing actions. It reuses the
transliterator without retaining committed text. Regression sequences are in
[`tests/composition.rs`](tests/composition.rs).

The transliterator lives in private modules under `src/transliteration/`:

- [`mod.rs`](src/transliteration/mod.rs): the scanner, per-call context, and
  explicit matching priority across rule families.
- [`case.rs`](src/transliteration/case.rs): selective ASCII aliases before rule matching;
  Unicode and the remaining case distinctions are preserved.
- [`rules.rs`](src/transliteration/rules.rs): independently authored mapping
  tables, retaining their verified order and provenance comments.
- [`vowels.rs`](src/transliteration/vowels.rs): fixed vowel-sign escapes and
  ordinary contextual vowels; the scanner preserves ng/nasal-r priority and
  handles ordinary `rri` plus `aZ`/`oZ`.
- [`symbols.rs`](src/transliteration/symbols.rs): ordered punctuation, symbol escapes,
  and empty-output backticks; all reset raw phonetic context.
- [`reph.rs`](src/transliteration/reph.rs): leading-`rr` attachment, raw preceding-r
  guard, and terminal `rrr` exception.
- [`following_r.rs`](src/transliteration/following_r.rs): shared following-`r`
  attachment and exceptions based on raw keys.

The existing `transliterate` API and conversion behavior are unchanged.
Composition adds a separate API. The experimental Fcitx5 addon is under
[`adapters/fcitx5`](adapters/fcitx5/CMakeLists.txt), with a separate Rust C-ABI
crate and C++ framework code. The core still forbids unsafe code and has no
platform dependencies. See [the architecture plan](docs/v1.md#proposed-architecture).

Source comments keep the API contract and reasons for non-obvious matching
choices close to the code. Detailed rule behavior belongs in
[the behavior notes](docs/v1.md#transliteration-api); exhaustive examples live
in tests and fixtures. Crate documentation keeps three transliteration examples;
the `Composition` type includes an executable word-deletion example.

## Build and try the conversion slice

Use Rust/Cargo **1.99.1 or newer**, with rustfmt and Clippy. This initial baseline
is the toolchain actually validated, not evidence that older Rust cannot work.
The library uses the Rust 2024 edition and has no external dependencies.

From the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --offline --workspace --all-targets -- -D warnings
cargo test --offline --workspace
cargo build --offline --workspace --release
cargo run --offline --example convert -- 'kh12T'
```

The last command prints `খ১২ট`. Try `` '. .. ... .` : :` ^ ^` ,, $ k`a' ``
for punctuation, literal symbol escapes, and a backtick between consonant and vowel.
The exact outputs and Unicode are in the
[symbol table](docs/v1.md#punctuation-and-symbol-batch). Try `'t T th Th n N'` for `ত ট থ ঠ ন ণ`, or
`'0123456789'` for `০১২৩৪৫৬৭৮৯`. Try `'ami khani kia'` for `আমি খানি কিয়া`.
The `u`/`U` group also supports `'tumi bhUmi kua'` → `তুমি ভূমি কুয়া`.
Try `'ki kI nIti'` → `কি কী নীতি` for the `i`/`I` distinction.
Try `'ami ke kee'` → `আমি কে কী` for single `e` and double `ee`.
Try `'tumi kemon tOmake'` → `তুমি কেমন তোমাকে` for lowercase `o` and uppercase `O`.
Try `'OI kOI OU kOU nOUka'` → `ঐ কৈ ঔ কৌ নৌকা` for the two-key vowels.
Try `'rri krri krriti rriN'` → `ঋ কৃ কৃতি ঋণ` for the three-key vowel.
Try `'ram rani rrri rrrri'` → `রাম রানি রররি ররৃ` for basic `r`/`rr` matching.
Try `'pani phOn fOn'` → `পানি ফোন ফোন` for `p` and the two spellings of `ফ`.
Try `'cabi chabi jani jhuri'` → `চাবি ছাবি জানি ঝুরি` for `c`/`ch`/`j`/`jh`.
Try `'din dhan Dan Dhaka'` → `দিন ধান ডান ঢাকা` for `d`/`dh`/`D`/`Dh`.
Try `'lal holud bhalO'` → `লাল হলুদ ভালো` for `l`/`h`.
Try `'s sh S Sh'` → `স শ শ ষ` for the case-sensitive `s`/`sh`/`S`/`Sh` group.
Try `'A AZ kH SH NgK KSHM QX'` → `আ অ্যা খ ষ ঙ্ক ক্ষ্ম কক্স` for selective case aliases.
Try `'G Gh Ghn Gdh GN Gn Gm Gl GG Gg gG J Jh jh'` → `গ ঘ ঘ্ন গ্ধ গ্ণ গ্ন গ্ম গ্ল জ্ঞ জ্ঞ জ্ঞ জ জহ ঝ`.
Try `'sokal shishu SiSu bhaSha'` → `সকাল শিশু শিশু ভাষা` for words using that group.
Try `'baRi gaRi gaRh drriRh'` → `বাড়ি গাড়ি গাঢ় দৃঢ়` for `R`/`Rh`.
Try `'zai zabe zodi zokhon'` → `যাই যাবে যদি যখন` for lowercase `z`.
Try `'Ng NG aNgul raNga'` → `ঙ ঞ আঙুল রাঙা` for case-sensitive `Ng`/`NG`.
Try `'bangla rong nga ngi'` → `বাংলা রং ঙ্গা ঙ্গি` for lowercase `ng`.
Try `'Ngrri NGrrri ngrrrri'` → `ঙররি ঞরৃ ংররররি` for their following `r` runs.
Try `'zaY hoY hoYe maYa'` → `যায় হয় হয়ে মায়া` for uppercase `Y`.
Try `'y ay ky kiya'` → `ইয় আয় ক্য কিয়া` for contextual lowercase `y`.
Try `'kZa aZ oZ baZ'` → `ক্যা অ্যা অ্য বঅ্যা` for uppercase `Z` and its longer forms.
Try `'yrr yrri yry Zry'` → `ইয়্রর ইয়ৃ ইয়র‍্য ্য্র্য` for following-`r` distinctions.
Try `'Yr Yrri Yry Yrry'` → `য়্র য়ৃ য়্র্য য়রর‍্য` for uppercase `Y` and its exceptions.
Try `'qr zr Rr Rhr qry zrry Rrri RHrr'` → `ক্র য্র ড়্র ঢ়্র ক্র্য যরর‍্য ড়ৃ ঢ়্রর`
for the latest following-r batch.
Try `'q qa qi qOI qotha'` → `ক কা কি কৈ কথা` for lowercase `q` and vowel context.
Try `'v vi vU valO vUmi'` → `ভ ভি ভূ ভালো ভূমি` for lowercase `v` and vowel context.
Try `'w wa awa kw kwa'` → `ও ওয়া আওা ক্ব ক্বা` for contextual lowercase `w`.
Try `'wr wrri wrrri wrry'` → `ওর ওৃ ও্রররি ওরর‍্য` for its following-`r` priority.
Try `'x xa axa kx kxa'` → `এক্স এক্সা আক্সা ক্ষ ক্ষা` for contextual `x` and exact `kx`.
Try `'xr xrr xrri xrrri kxrr'` → `এক্সর এক্স্রর এক্সৃ এক্স্রররি ক্ষ্রর` for their following-`r` priority.
Try `'Ngx Ngxa Ngxrri Ngxrr'` → `ঙ্ষ ঙ্ষা ঙ্ষৃ ঙ্ষ্রর` for exact `Ngx` and its following forms.
Try `'Ngk Ngka Ngkr Ngkrri'` → `ঙ্ক ঙ্কা ঙ্ক্র ঙ্কৃ` for exact `Ngk` and its following forms.
Try `'Ngkh Ngkha Ngkhr Ngkhrri'` → `ঙ্খ ঙ্খা ঙ্খ্র ঙ্খৃ` for exact `Ngkh` and its following forms.
Try `'Ngkx Ngkxa Ngkxrri Ngkxrr'` → `ঙ্ক্ষ ঙ্ক্ষা ঙ্ক্ষৃ ঙ্ক্ষ্রর` for exact `Ngkx` and its following forms.
Try `'Ngkkh Ngkkha Ngkkhr Ngkkhrri'` → `ঙ্ক্ষ ঙ্ক্ষা ঙ্ক্ষ্র ঙ্ক্ষৃ` for exact `Ngkkh` and its following forms.
Try `'NgkSh NgkSha NgkShr NgkShrri'` → `ঙ্ক্ষ ঙ্ক্ষা ঙ্ক্ষ্র ঙ্ক্ষৃ` for exact `NgkSh` and its following forms.
Try `'kkh kkha kkhr kkhrri'` → `ক্ষ ক্ষা ক্ষ্র ক্ষৃ` for exact `kkh` and its following forms.
Try `'kSh kSha kShr kShrri'` → `ক্ষ ক্ষা ক্ষ্র ক্ষৃ` for exact `kSh` and its following forms.
Try `'kkhN kShNa kxNr kxNrri'` → `ক্ষ্ণ ক্ষ্ণা ক্ষ্ণ্র ক্ষ্ণৃ` for the three exact N-ending spellings.
Try `'biggan rugNa biShoNNo oporahNo bighno chihno udgom udghaton khaRgo'`
→ `বিজ্ঞান রুগ্ণা বিষণ্ণ অপরাহ্ণ বিঘ্ন ছিহ্ন উদ্গম উদ্ঘাতন খাড়্গ`.
Pass the input as one quoted argument. The example prints the result to the
terminal; the library retains no input state
and performs no logging, file access, or network access.

The API is `jong_bangla_core::transliterate(&str) -> String`; see
[src/lib.rs](src/lib.rs). At each position it chooses the longest supported
sequence, so `kh` becomes one `খ` instead of `কh`. Unmatched characters are
copied unchanged, including Bangla and emoji. `a`/`i` produce independent `আ`/`ই`
at the start or after a boundary, and vowel signs after supported consonants:
`ka` → `কা`, `ki` → `কি`. An `a` immediately after `i` inserts `য়` (`kia` → `কিয়া`).
Likewise, `u`/`U` produce `উ`/`ঊ` independently and signs after consonants:
`ku` → `কু`, `kU` → `কূ`. An `a` after either form also inserts `য়` (`kua` → `কুয়া`).
Uppercase `I` produces `ঈ` independently and `ী` after consonants (`kI` → `কী`);
`a` after `I` also inserts `য়` (`kIa` → `কীয়া`).
Single `e` produces `এ` independently and `ে` after consonants (`ke` → `কে`).
Double `ee` is matched first and acts like `I` (`ee` → `ঈ`, `kee` → `কী`). An `a`
after either form also inserts `য়` (`kea` → `কেয়া`, `keea` → `কীয়া`).
Lowercase `o` produces `অ` independently and adds no character after a supported
consonant (`ko` → `ক`). Uppercase `O` produces `ও` independently and `ো` after
consonants (`kO` → `কো`). Lowercase `o` after another vowel becomes `ও` (`ao` → `আও`),
except after a raw lowercase `o`. The sequence `oo` is matched first and acts like
`u` (`oo` → `উ`, `koo` → `কু`); `ooo` → `উঅ`.
The case-sensitive pairs `OI`/`OU` produce `ঐ`/`ঔ` independently and `ৈ`/`ৌ`
after consonants (`kOI` → `কৈ`, `kOU` → `কৌ`). They match before single `O`;
`Oi` still produces `ওই`.
Lowercase `rri` normally produces `ঋ` independently and `ৃ` after a supported consonant
(`krri` → `কৃ`). The principal forms for all 11 basic vowels are now implemented;
full context and some longer sequences are still incomplete. Vowel-sign escapes
force signs: `` a` `` → া, `` i` `` → ি, and `` rri` `` → ৃ. Escaped lowercase
o emits nothing: `` ko`a `` → কআ. Every escape ends at its backtick, so the
following vowel starts independently. See the [escape table](docs/v1.md#vowel-sign-escape-batch)
for all spellings and earlier-match exceptions.
The separate escape ``` t`` ``` → ৎ uses one Unicode letter. It resets context:
``` t``a ``` → ৎআ. Earlier conjuncts still consume their t:
``` nt`` ``` → ন্ত; ``` ttt`` ``` → ত্তৎ. Uppercase T stays distinct:
``` T`` ``` → ট. See the [final-escape record](docs/v1.md#final-t-escape).
Uppercase `Q` aliases `q`: `Qa` → `কা`; following `qr`/`Qr` produce `ক্র`.
Basic `r` produces `র`, and `rr`
produces `রর`; `rri` is matched before either. This resolves `rrri` → `রররি` and
`rrrri` → `ররৃ`. Basic `kr` produces `ক্র`. Reph attaches leading `rr`:
`'rrq rrz rrR rrRh rrk rrShN'` → `র্ক র্য র্ড় র্ঢ় র্ক র্ষ্ণ`.
Existing vowel and following-r rules still apply (`rrqry` → `র্ক্র্য`,
`rrqrry` → `র্করর‍্য`); lowercase y and uppercase Z keep the basic rr joiner.
Exactly terminal `rrr` → `র্র`, while `rrr!` → `ররর!` and `rrrq` → `রররক`.
A consumed r prevents a fresh reph match (`krrq` → `ক্ররক`); a backtick
separates raw matches (`` r`rrq `` → `রর্ক`). See the [reph batch](docs/v1.md#reph-batch).
Lowercase `p` produces `প`; `ph` and `f` both produce `ফ`. The longest supported
match handles `ph` before `p` (`phi` → `ফি`, matching `fi` → `ফি`).
`P`/`F` and uppercase `H` inside matches now work as aliases. Following `r`
now attaches: `'pr phr fr'` → `প্র ফ্র ফ্র`, with `rri`/`rry`/`rrZ` exceptions.
Lowercase `c`/`ch`/`j`/`jh` produce `চ`/`ছ`/`জ`/`ঝ`; `ch` and `jh` match before
their single-key prefixes. Exact `cc`/`cch`/`cNG` and `jj`/`jjh`/`jNG` now
match, including following `r`: `'cr chr jr jhr'` → `চ্র ছ্র জ্র ঝ্র`.
Exact `d`/`dh`/`D`/`Dh` produce `দ`/`ধ`/`ড`/`ঢ`; `dh` and `Dh` match before
their single-key prefixes. Case distinguishes `da` → `দা` from `Da` → `ডা`.
`ddh` → `দ্ধ` now matches, as described below. Uppercase-`H` aliases
also match (`dH` → `ধ`, `DH` → `ঢ`).
Lowercase `l`/`h` produce `ল`/`হ` and establish vowel context (`li` → `লি`,
`hu` → `হু`). Existing two-key matches still win: `kh` → `খ`, `khh` → `খহ`.
Exact `ll` → `ল্ল` now matches as described below. Uppercase `L`/`H`
are aliases.
Exact `s` → স, `sh`/`S` → শ, and `Sh` → ষ follow the confirmed mapping.
`sh` matches before `s`, and `Sh` before `S`; all four support vowel signs.
Uppercase-`H` aliases now match (`sH` → `শ`, `SH` → `ষ`).
Exact `R`/`Rh` produce ড়/ঢ় (U+09DC/U+09DD), with `Rh` matched before `R`.
They support vowel signs, and `Rrri` → `ড়ৃ` preserves the existing `rri` vowel.
Uppercase `H` also matches (`RH` → `ঢ়`). Following `r` attaches:
`Rri` → `ড়্রি`, `Rhr` → `ঢ়্র`, and `Rhry` → `ঢ়্র্য` (without U+200D).
Lowercase `z` produces য (U+09AF), with supported vowel signs (`zi` → `যি`).
It remains distinct from uppercase `Z` → ্য (U+09CD U+09AF).
Following `r` attaches: `zr` → `য্র`, `zry` → `য্র্য`, while `zrry` → `যরর‍্য`
retains U+200D.
Exact `Ng`/`NG` produce ঙ/ঞ (U+0999/U+099E), matching before single `N` → ণ.
They support vowel signs (`Ngi` → `ঙি`, `NGU` → `ঞূ`). Exact `Ngk` → `ঙ্ক`
now matches before `Ng` and supports vowels and aliases (`Ngki` → `ঙ্কি`).
It attaches single following `r` without U+200D (`Ngkr` → `ঙ্ক্র`, `Ngkry` → `ঙ্ক্র্য`),
sharing the two-`r` priority and exceptions (`Ngkrri` → `ঙ্কৃ`,
`Ngkrry` → `ঙ্করর‍্য`). Earlier `gN` now wins in `gNgk` → `গ্ণগক`.
Exact `Ngkh` → `ঙ্খ` now matches before `Ngk` and shares its vowel context
and following-`r` priority (`Ngkhi` → `ঙ্খি`, `Ngkhr` → `ঙ্খ্র`,
`Ngkhrri` → `ঙ্খৃ`, `Ngkhrry` → `ঙ্খরর‍্য`). Earlier `gN` now wins in
`gNgkh` → `গ্ণগখ`.
Exact `Ngkx` → `ঙ্ক্ষ` also matches before `Ngk`, sharing `Ngx`/`kx`
vowel context and following-`r` priority (`Ngkxi` → `ঙ্ক্ষি`,
`Ngkxrri` → `ঙ্ক্ষৃ`, `Ngkxrr` → `ঙ্ক্ষ্রর`). Single `Ngkxr` → `ঙ্ক্ষর`
stays separate and adds U+200D before `y`/`Z` (`Ngkxry` → `ঙ্ক্ষর‍্য`).
Earlier `gN` now wins in `gNgkx` → `গ্ণগক্ষ`.
Exact `Ngkkh` → `ঙ্ক্ষ` matches before `Ngk`, with vowel signs/aliases and
`Ngkh`'s following-`r` behavior (`Ngkkhr` → `ঙ্ক্ষ্র`, `Ngkkhrri` → `ঙ্ক্ষৃ`).
Its single following `r` attaches without a joiner (`Ngkkhry` → `ঙ্ক্ষ্র্য`),
while `Ngkxry` retains U+200D. Earlier `gN`/`NN`/`hN` overlaps now match.
Exact `NgkSh` shares `Ngkkh`'s output and vowel/following-`r` behavior
(`NgkShr` → `ঙ্ক্ষ্র`, `NgkShrri` → `ঙ্ক্ষৃ`, `NgkShry` → `ঙ্ক্ষ্র্য`).
Case-distinct `Ngksh` → `ঙ্কশ` stays separate. Earlier `gN`/`NN`/`hN` overlaps
now match.
Exact `kkh` → `ক্ষ` matches before basic `k`/`kh`, sharing `Ngkh`'s vowel
context and following-`r` behavior (`kkhr` → `ক্ষ্র`, `kkhrri` → `ক্ষৃ`,
`kkhry` → `ক্ষ্র্য`). Earlier exact `Ngkkh` keeps priority;
`NGkkh` → `ঞক্ষ` and `ngkkh` → `ংক্ষ` now also match the reference.
Earlier `nk`/`lk`/`sk`/`Shk` now consume `k` first (`nkkh` → `ঙ্কখ`,
`lkkh` → `ল্কখ`, `skkh` → `স্কখ`, `Shkkh` → `ষ্কখ`).
Exact `kSh` shares `kkh`'s output, vowel context, and following-`r` behavior
(`kShr` → `ক্ষ্র`, `kShrri` → `ক্ষৃ`, `kShry` → `ক্ষ্র্য`). Earlier exact
`NgkSh` keeps priority; `NGkSh` → `ঞক্ষ` and `ngkSh` → `ংক্ষ` now match.
Case-distinct `ksh` → `কশ` stays separate.
Exact `kkhN`/`kShN`/`kxN` → `ক্ষ্ণ` match before their shorter forms, with
supported vowel signs/aliases and single-`r` attachment (`kxNr` → `ক্ষ্ণ্র`,
`kxNry` → `ক্ষ্ণ্র্য`). Two `r` keys normally add ্রর, while `rri` and
`rry`/`rrZ` keep priority (`kxNrri` → `ক্ষ্ণৃ`, `kxNrry` → `ক্ষ্ণরর‍্য`).
Earlier `Ngkkh`/`NgkSh`/`Ngkx` still win (`NgkShN` → `ঙ্ক্ষণ`). Consuming
the final `N` prevents a following nasal match (`kShNgkSh` → `ক্ষ্ণগক্ষ`);
this also resolves `kxNgx`, `kxNgk`, `kxNgkh`, `kxNgkx`, `kxNgkkh`, and `kxNgkSh`.
Uppercase `G` also converts after these forms (`kShNG` → `ক্ষ্ণগ`).
Exact `kkhm`/`kShm`/`kxm` → `ক্ষ্ম` also match before their shorter forms,
with supported vowel signs/aliases and single-`r` attachment (`kxmr` → `ক্ষ্ম্র`,
`kxmry` → `ক্ষ্ম্র্য`). The `rri` and `rry`/`rrZ` exceptions keep priority
(`kxmrri` → `ক্ষ্মৃ`, `kxmrry` → `ক্ষ্মরর‍্য`). Try
`'lokkhmI lokShmI lokxmI sUkShmo'` → `লক্ষ্মী লক্ষ্মী লক্ষ্মী সূক্ষ্ম`.
Earlier `Ngkkh`/`NgkSh`/`Ngkx` still win (`NgkShm` → `ঙ্ক্ষম`).
Consuming `m` prevents it joining a following nasal (`kShmng` → `ক্ষ্মং`,
`kShmn` → `ক্ষ্মন`). Basic `mr`/`mm`/`mn` are now supported by the nasal/h
batch below.
Exact `kk` → `ক্ক` matches after longer `kkh`/`kkhN`/`kkhm` and before single `k`.
Its vowel context and following `r` share `Ngk`'s behavior: `kki` → `ক্কি`,
`kkr` → `ক্ক্র`, `kkrri` → `ক্কৃ`, `kkry` → `ক্ক্র্য`, and `kkrry` → `ক্করর‍্য`.
Try `'Dhakka dhakka pakka akkel'` → `ঢাক্কা ধাক্কা পাক্কা আক্কেল`.
Consuming both keys changes `kkkh` → `ক্কখ`, `kkSh` → `ক্কষ`, and
`kkx` → `ক্কক্স`; the second key cannot begin `kkh`/`kSh`/`kx`.
Earlier `Ngk` still wins (`Ngkk` → `ঙ্কক`). Repeated `kk` and neighboring
supported forms work (`kkkk` → `ক্কক্ক`, `kkhkk` → `ক্ষক্ক`).
Exact `ShN` → `ষ্ণ` and `Shm` → `ষ্ম` match before basic `Sh`, with supported
vowel signs/aliases and following-`r` attachment: `ShNi` → `ষ্ণি`, `Shmr` → `ষ্ম্র`,
`ShNrri` → `ষ্ণৃ`, `Shmry` → `ষ্ম্র্য`, and `Shmrry` → `ষ্মরর‍্য`.
Try `'biShNu uShNo uShma'` → `বিষ্ণু উষ্ণ উষ্মা`.
Earlier `kShN`/`kShm` and `NgkSh` still win; `kkShN` → `ক্কষ্ণ` and
`kkShm` → `ক্কষ্ম` now match. Consuming `N` prevents a following `Ng` match:
`ShNgx` → `ষ্ণগক্স`, `ShNgkSh` → `ষ্ণগক্ষ`. Consuming `m` keeps a following
nasal separate: `Shmng` → `ষ্মং`, `Shmn` → `ষ্মন`. Uppercase `G` now converts
after `ShN` (`ShNG` → `ষ্ণগ`). Basic
`mr`/`mm`/`mn` now match as described below.
Basic `k`/`kh`/`Sh` now attach following `r`: try `'kr khr Shr'` → `ক্র খ্র ষ্র`.
Vowels and ya forms work (`kri` → `ক্রি`, `khry` → `খ্র্য`, `ShrZ` → `ষ্র্য`).
The `rri` and `rry`/`rrZ` exceptions remain (`krri` → `কৃ`, `krry` → `করর‍্য`);
`krr` → `ক্রর`. Existing longer matches retain priority, while exposed basic
spellings now attach: `kkkhr` → `ক্কখ্র`, `kkShr` → `ক্কষ্র`, `Ngkkr` → `ঙ্কক্র`.
Copied Bangla does not supply raw context (`কr` → `কর`); raw `qr` gives `ক্র`.
Reph remains incomplete.
Basic `g`/`gh` now attach following `r`: `'gr ghr'` → `গ্র ঘ্র`.
Vowels and ya forms keep consonant context (`grI` → `গ্রী`, `ghry` → `ঘ্র্য`).
The exceptions remain (`grri` → `গৃ`, `ghrry` → `ঘরর‍্য`), while `grr` → `গ্রর`.
Try `'grIShmo ghraN ghrriNa'` → `গ্রীষ্ম ঘ্রাণ ঘৃণা`.
Existing conjuncts retain priority; exposed `g` now attaches in `ShNgr` → `ষ্ণগ্র`,
`Shmgr` → `ষ্মগ্র`, and `kShmgr` → `ক্ষ্মগ্র`. Earlier `dg`/`Rg`/`gg`/`gN`
matches now have their own rules. Copied Unicode resets context (`গr` → `গর`, `ঘr` → `ঘর`).
Exact `Ngg`/`Nggh` now produce `ঙ্গ`/`ঙ্ঘ`: try
`'Ngg Nggh Nggr Ngghr Nggrri Ngghry'` → `ঙ্গ ঙ্ঘ ঙ্গ্র ঙ্ঘ্র ঙ্গৃ ঙ্ঘ্র্য`.
Both allow supported vowel signs and ya forms; `Nggrr` → `ঙ্গ্রর`, while
`Nggrry` → `ঙ্গরর‍্য` preserves the joiner exception. Longer `Nggh` wins before
`Ngg`, while consuming `Ngg` first keeps later consonants separate
(`Nggg` → `ঙ্গগ`, `NggN` → `ঙ্গণ`). Case distinctions remain:
`NGgr` → `ঞগ্র`, `nggr` → `ংগ্র`. Earlier `ShN`/`kShN` still consume `N`
first; the remaining `gg` now matches (`ShNggr` → `ষ্ণজ্ঞ্র`).
Copied `ঙ্গ` resets context (`ঙ্গr` → `ঙ্গর`).
Exact `gg`/`gN`/`NN`/`hN` produce জ্ঞ/গ্ণ/ণ্ণ/হ্ণ, while `gn`/`ghn`/`hn`
produce গ্ন/ঘ্ন/হ্ন and `dg`/`dgh`/`Rg` produce দ্গ/দ্ঘ/ড়্গ. Each supports
vowel signs, ya forms, and following `r` (`ggr` → `জ্ঞ্র`, `ghnrri` → `ঘ্নৃ`).
The existing `rri`/`rry`/`rrZ` exceptions remain (`NNrry` → `ণ্ণরর‍্য`).
Longer `dgh` wins before `dg`, and `ghn` before basic `gh`. Consumed keys cannot
begin another match: `dgn` → `দ্গন`, `dggh` → `দ্গঘ`, `gNgg` → `গ্ণজ্ঞ`,
`NNgg` → `ণ্ণজ্ঞ`, and `hNgg` → `হ্ণজ্ঞ`. Exact `ggh` stays জ্ঞহ, while
`Nggh` stays ঙ্ঘ; exposed basic `h` attaches in `kShNgghr` → `ক্ষ্ণজ্ঞহ্র`.
Basic `h`/`n`/`N` now attach following `r`, with the same exceptions:
`hr nr Nr` → `হ্র ন্র ণ্র`, `hrri` → `হৃ`, and `nrry` → `নরর‍্য`.
Exact `ngh`/`nn`/`Nn`/`mn`/`hm`/`hl`/`nm`/`Nm`/`mm`/`ml`/`Ngm`/`gm` now
produce ঙ্ঘ/ন্ন/ণ্ন/ম্ন/হ্ম/হ্ল/ন্ম/ণ্ম/ম্ম/ম্ল/ঙ্ম/গ্ম, respectively.
Try `'ngh nn Nn mn hm hl nm Nm mm ml Ngm gm mr lr'` → `ঙ্ঘ ন্ন ণ্ন ম্ন হ্ম হ্ল ন্ম ণ্ম ম্ম ম্ল ঙ্ম গ্ম ম্র ল্র`.
Each allows supported vowel signs, ya, and following `r`, including `hlr` → `হ্ল্র`.
The `rri`/`rry`/`rrZ` exceptions remain (`mlrri` → `ম্লৃ`, `nnrry` → `ন্নরর‍্য`).
`ngh` wins before `ng`; consumed keys stay consumed (`nng` → `ন্নগ`,
`mng` → `ম্নগ`, `Ngmm` → `ঙ্মম`). Earlier matches can expose `gm`:
`ShNgm` → `ষ্ণগ্ম`, `kShNgmr` → `ক্ষ্ণগ্ম্র`.
Basic `m`/`l` now attach following `r` (`mr lr` → `ম্র ল্র`), resolving
`NgkShmr` → `ঙ্ক্ষম্র`. The new forms also resolve `Shmmm` → `ষ্মম্ম`
and `ShNmng` → `ষ্ণম্নগ`. Copied Bangla resets context (`হ্মr` → `হ্মর`).
Exact `dd`/`ddh`/`dhn`/`dhm`/`dm`/`dbh`/`dv`/`DD` now produce
দ্দ/দ্ধ/ধ্ন/ধ্ম/দ্ম/দ্ভ/দ্ভ/ড্ড, respectively. Try
`'dd ddh dhn dhm dm dbh dv DD'` → `দ্দ দ্ধ ধ্ন ধ্ম দ্ম দ্ভ দ্ভ ড্ড`.
Each supports vowel signs, ya, and following `r`, preserving the
`rri`/`rry`/`rrZ` exceptions (`ddrri` → `দ্দৃ`, `dvrry` → `দ্ভরর‍্য`).
Longer `ddh` wins before `dd`. Consumed keys cannot start another match:
`dddh` → `দ্দধ`, `ddhn` → `দ্ধন`, `ddbh` → `দ্দভ`, and `DDh` → `ড্ডহ`.
Basic `d`/`dh`/`D`/`Dh`/`b`/`bh`/`v` now attach following `r`:
`'dr dhr Dr Dhr br bhr vr'` → `দ্র ধ্র ড্র ঢ্র ব্র ভ্র ভ্র`.
This resolves `ddghr` → `দ্দঘ্র`, `dddhr` → `দ্দধ্র`, `ddbhr` → `দ্দভ্র`,
and `DDDhr` → `ড্ডঢ্র`. Try `'buddhi podmo udbhob udvob aDDa brrikkho'` → `বুদ্ধি পদ্ম উদ্ভব উদ্ভব আড্ডা বৃক্ষ`.
Copied Bangla resets context (`দ্দr` → `দ্দর`).
Exact `nd`/`ndh`/`nD`/`ND`/`NDh`/`nt`/`nth`/`nT`/`NT`/`nTh`/`NTh`
now produce ন্দ/ন্ধ/ন্ড/ণ্ড/ণ্ঢ/ন্ত/ন্থ/ন্ট/ণ্ট/ন্ঠ/ণ্ঠ, respectively.
Try `'nd ndh nD ND NDh nt nth nT NT nTh NTh'` → `ন্দ ন্ধ ন্ড ণ্ড ণ্ঢ ন্ত ন্থ ন্ট ণ্ট ন্ঠ ণ্ঠ`.
Each spelling supports vowel signs, ya, and following `r`, including the existing
`rri`/`rry`/`rrZ` exceptions (`ndrri` → `ন্দৃ`, `ntrry` → `ন্তরর‍্য`).
Longer `ndh`/`NDh`/`nth`/`nTh`/`NTh` win over their shorter forms.
Case and consumed keys matter: `nDh` → `ন্ডহ`, `NDh` → `ণ্ঢ`,
`ndd` → `ন্দদ`, `nddh` → `ন্দধ`, `ndhn` → `ন্ধন`, and `nndh` → `ন্নধ`.
Earlier `ShN`/`kShN`/`kxN` still consume `N` first (`kShNThr` → `ক্ষ্ণঠ্র`).
Basic `t`/`th`/`T`/`Th` now attach following `r`:
`'tr thr Tr Thr'` → `ত্র থ্র ট্র ঠ্র`, resolving `nntr` → `ন্নত্র`.
Try `'anondo bondhu grontho ghonTa ghoNTa konTho koNTho'` → `আনন্দ বন্ধু গ্রন্থ ঘন্টা ঘণ্টা কন্ঠ কণ্ঠ`.
Copied Bangla resets context (`ন্দr` → `ন্দর`).
Exact `tt`/`tth`/`tn`/`tm`/`TT`/`Tm` now produce ত্ত/ত্থ/ত্ন/ত্ম/ট্ট/ট্ম.
Try `'tt tth tn tm TT Tm uttor utthan rotno atma'` → `ত্ত ত্থ ত্ন ত্ম ট্ট ট্ম উত্তর উত্থান রত্ন আত্মা`.
Each supports vowel signs, ya, and following `r` with the existing exceptions
(`ttrri` → `ত্তৃ`, `tthrry` → `ত্থরর‍্য`, `Tmry` → `ট্ম্র্য`).
Longer `tth` wins before `tt`; `TTh` instead produces `ট্টহ`.
Consumed keys cannot start another match (`ttth` → `ত্তথ`, `ttn` → `ত্তন`,
`tng` → `ত্নগ`, `tmn` → `ত্মন`, `TTm` → `ট্টম`). Earlier nasal stops still win
(`ntt` → `ন্তত`, `ntth` → `ন্তথ`, `nTm` → `ন্টম`). Case-distinct `thn`/`thm`/`Tn`
stay separate (`থন`/`থম`/`টন`). Copied Bangla resets context (`ত্তr` → `ত্তর`).
Exact `kt`/`kT`/`kl` now produce ক্ত/ক্ট/ক্ল; `kn`/`km` stay separate (কন/কম).
Try `'kt kT kl kn km rokto shokti dokTor klanto klab'` → `ক্ত ক্ট ক্ল কন কম রক্ত শক্তি দক্টর ক্লান্ত ক্লাব`.
Vowels, ya, and following `r` use the checked exceptions (`ktrri` → `ক্তৃ`,
`kTrry` → `ক্টরর‍্য`, `klry` → `ক্ল্র্য`). There is no longer `kth`/`kTh`
match: `kth` → `ক্তহ`, `kTh` → `ক্টহ`. Consuming `t`/`T` prevents a new
conjunct starting at that key (`ktm` → `ক্তম`, `kTm` → `ক্টম`, `ktth` → `ক্তথ`).
Earlier k/nasal matches still win (`Ngkt` → `ঙ্কত`, `kkl` → `ক্কল`).
Copied Bangla resets context (`ক্তr` → `ক্তর`).
Exact `nk`/`nkh`/`lk`/`sk`/`skh`/`skl`/`Shk`/`ks` now produce
ঙ্ক/ঙ্খ/ল্ক/স্ক/স্খ/স্ক্ল/ষ্ক/ক্স. Longer `nkh`/`skh`/`skl` win before
`nk`/`sk`; longer `lkh`/`ksh` preserve separate consonants (`লখ`/`কশ`).
Try `'nk nkh lk lkh sk skl skh Shk ks ksh'` → `ঙ্ক ঙ্খ ল্ক লখ স্ক স্ক্ল স্খ ষ্ক ক্স কশ`.
Each supports vowels, ya, and following `r` with checked exceptions
(`nkrri` → `ঙ্কৃ`, `lkrry` → `ল্করর‍্য`, `skrrZ` → `স্করর‍্য`, `ksry` → `ক্স্র্য`).
Consumed keys cannot start another conjunct (`nktm` → `ঙ্কত্ম`, `lktm` → `ল্কত্ম`,
`sktm` → `স্কত্ম`, `Shktm` → `ষ্কত্ম`, `skkl` → `স্কক্ল`). `kshm` → `কশম`
and `lkhm` → `লখম` preserve the longer exceptions. Basic `s`/`sh`/`S` now
attach following `r`: `'sr shr Sr srri shrri Srri'` → `স্র শ্র শ্র সৃ শৃ শৃ`.
Try `'onko shonkho alko skhalon sklera shuShko baks shrom Srom'` → `অঙ্ক শঙ্খ আল্ক স্খালন স্ক্লেরা শুষ্ক বাক্স শ্রম শ্রম`.
Case-distinct `Nk`/`Sk`/`shk` stay separate (ণক/শক/শক); copied Bangla resets
context (`স্ক্লr` → `স্ক্লর`).
Exact `gl`/`ll`/`bl`/`pl`/`sl`/`shl`/`bhl`/`phl`/`vl`/`fl`/`Sl` now match:
`'gl ll bl pl sl shl bhl phl vl fl Sl'` → `গ্ল ল্ল ব্ল প্ল স্ল শ্ল ভ্ল ফ্ল ভ্ল ফ্ল শ্ল`.
`Shl` stays separate (ষল). Each supports vowels, ya, following `r`, and checked
exceptions (`glr` → `গ্ল্র`, `blrri` → `ব্লৃ`, `llrry` → `ল্লরর‍্য`, `shlry` → `শ্ল্র্য`).
Consumed `l` cannot start a second match: `'blk glk llk slk'` → `ব্লক গ্লক ল্লক স্লক`.
Earlier matches still win (`ggl` → `জ্ঞল`, `Nggl` → `ঙ্গল`, `dbhl` → `দ্ভল`,
`kshl` → `কশল`, `skll` → `স্ক্লল`); `mpl` → `মপ্ল` stays separate before `pl`.
Try `'glani ullas blok plabon sloth shlOk phlash flash SlOk'`
→ `গ্লানি উল্লাস ব্লক প্লাবন স্লথ শ্লোক ফ্লাশ ফ্লাশ শ্লোক`.
Basic `p`/`ph`/`f` exposed after each new form attach following `r`:
`'glpr glphr glfr'` → `গ্লপ্র গ্লফ্র গ্লফ্র`. Copied Bangla resets context (`গ্লr` → `গ্লর`).
Exact `mb`/`mbh`/`mv`/`mp`/`mph`/`mf` now produce ম্ব/ম্ভ/ম্ভ/ম্প/ম্ফ/ম্ফ.
Longer `mbh`/`mph` win before `mb`/`mp`; `mpl` preserves মপ্ল before `mp`
could join the first two consonants. Exact `lm`/`sm`/`shm`/`Sm` produce
ল্ম/স্ম/শ্ম/শ্ম and consume `m` before another conjunct can begin.
Try `'mb mbh mv mp mph mf mpl lm sm shm Sm'` → `ম্ব ম্ভ ম্ভ ম্প ম্ফ ম্ফ মপ্ল ল্ম স্ম শ্ম শ্ম`.
Each takes supported vowels and ya and attaches following `r`, preserving
`rri`/`rry`/`rrZ` exceptions (`mbrri` → `ম্বৃ`, `mbhrry` → `ম্ভরর‍্য`, `mpry` → `ম্প্র্য`).
Consumed keys cannot start a second match: `'mbl mbhl mvl mphl mfl'` → `ম্বল ম্ভল ম্ভল ম্ফল ম্ফল`.
`mplk` → `মপ্লক` and `mpll` → `মপ্লল` preserve the longer exception;
`'lmp smp shmp Smp'` → `ল্মপ স্মপ শ্মপ শ্মপ` preserve earlier matches.
Existing rules also keep priority (`Ngmb` → `ঙ্মব`, `kkhmp` → `ক্ষ্মপ`,
`sklm` → `স্ক্লম`, `kshm` → `কশম`); consumed keys can expose a complete new
form (`mbbl` → `ম্বব্ল`, `mppl` → `ম্পপ্ল`). Copied Bangla resets context (`ম্বr` → `ম্বর`).
Try `'kombol gombhir omvor sompod lompho domf almari smarok shmoshan'`
→ `কম্বল গম্ভির অম্ভর সম্পদ লম্ফ দম্ফ আল্মারি স্মারক শ্মশান`.
These are exact reference conversions, with no dictionary spelling correction.
Exact `lg`/`lb`/`lbh`/`lv`/`lp`/`lT`/`lD`/`ldh` now match, with longer
`lgh`/`lph` exceptions that keep separate consonants but consume the full spelling.
Try `'lg lgh lb lbh lv lp lph lT lD ldh'` → `ল্গ লঘ ল্ব ল্ভ ল্ভ ল্প লফ ল্ট ল্ড ল্ধ`.
Consumed keys cannot begin another conjunct: `'lgl lghl lbl lbhl lvl lpl lphl lTm lDh'`
→ `ল্গল লঘল ল্বল ল্ভল ল্ভল ল্পল লফল ল্টম ল্ডহ`.
`lf` remains separate; `lfl` → `লফ্ল`, while `lphl` → `লফল`.
Vowels and following `r` retain their checked exceptions (`lgrri` → `ল্গৃ`,
`lphrrZ` → `লফরর‍্য`, `lTry` → `ল্ট্র্য`). Earlier l-ending rules still win
(`blg` → `ব্লগ`, `mlg` → `ম্লগ`); copied Bangla resets context (`ল্গr` → `ল্গর`).
Try `'golpo olpo ulTa'` → `গল্প অল্প উল্টা`.
Exact `sp`/`sph`/`sf`/`spl` and `Shp`/`Shph`/`Shf` now match:
`'sp sph sf spl Shp Shph Shf'` → `স্প স্ফ স্ফ স্প্ল ষ্প ষ্ফ ষ্ফ`.
Longer `sph`/`spl` and `Shph` win over `sp`/`Shp`. Only `spl` consumes
the final `l`: `'spll sphll sfll Shpll Shphll Shfll'`
→ `স্প্লল স্ফল্ল স্ফল্ল ষ্পল্ল ষ্ফল্ল ষ্ফল্ল`.
The available `l` can start another rule (`splm` → `স্প্লম`, `sphlm` → `স্ফল্ম`,
`Shplg` → `ষ্পল্গ`). Vowels/ya and following `r` retain their checked
exceptions (`sprri` → `স্পৃ`, `sphrry` → `স্ফরর‍্য`, `Shfry` → `ষ্ফ্র্য`).
Earlier matches still win (`ksp` → `ক্সপ`, `kspl` → `ক্সপ্ল`, `kShp` → `ক্ষপ`,
`NgkShp` → `ঙ্ক্ষপ`); copied Bangla resets context (`স্পr` → `স্পর`).
`Sp`/`Sph`/`Sf` and `shp`/`shph`/`shf` stay separate consonants.
Try `'sphulingo sfUron puShpo niShphol niShfol'` → `স্ফুলিঙ্গ স্ফূরন পুষ্প নিষ্ফল নিষ্ফল`.
Exact `'bb pp pn sn shn Sn'` → `ব্ব প্প প্ন স্ন শ্ন শ্ন` now match.
The second `b`/`p` cannot start `bh`/`ph`/`bl`/`pl`:
`'bbh pph bbl ppl bbhl pphl'` → `ব্বহ প্পহ ব্বল প্পল ব্বহ্ল প্পহ্ল`.
Following `h`/`l` can start a fresh rule (`bbhm` → `ব্বহ্ম`, `pplg` → `প্পল্গ`).
The consumed `n` cannot start `nk`, whereas a fresh `n` can:
`'pnk snk shnk Snk pnnk snnk shnnk Snnk'` → `প্নক স্নক শ্নক শ্নক প্নঙ্ক স্নঙ্ক শ্নঙ্ক শ্নঙ্ক`.
Earlier forms keep their keys (`mbb` → `ম্বব`, `ppn` → `প্পন`, `ksn` → `ক্সন`,
`kshn` → `কশন`, `spn` → `স্পন`). Vowels and following `r` retain their checked
exceptions (`bbrri` → `ব্বৃ`, `pnrry` → `প্নরর‍্য`, `Snry` → `শ্ন্র্য`).
`shn` and `Sn` are exact spellings; `Shn` stays ষন. Copied Bangla resets
context (`প্নr` → `প্নর`). Try `'abbas dhappa shopno snan sneho proshno proSno'`
→ `আব্বাস ধাপ্পা শপ্ন স্নান স্নেহ প্রশ্ন প্রশ্ন`, without dictionary correction.
Exact `'ns nsh ps psh'` → `ন্স নশ প্স পশ` now match.
Longer `nsh`/`psh` consume `h` and produce separate consonants. Consumed `s`/`h`
cannot start another match: `'nsk nsl nsm nsp nsn nshl nshm nshn'`
→ `ন্সক ন্সল ন্সম ন্সপ ন্সন নশল নশম নশন`.
Fresh keys can start rules (`nssn` → `ন্সস্ন`, `nshhl` → `নশহ্ল`).
Earlier matches keep their keys (`pnsh` → `প্নশ`, `pps` → `প্পস`, `mps` → `ম্পস`).
Vowels and following `r` retain their exceptions (`nsrri` → `ন্সৃ`,
`pshrry` → `পশরর‍্য`). Copied Bangla resets context (`ন্সa` → `ন্সআ`).
Case-sensitive controls stay distinct: `'nS nSh pS pSh'` → `নশ নষ পশ পষ`.
Try `'mansik lipsa ipsito'` → `মান্সিক লিপ্সা ইপ্সিত`, without dictionary correction.
Exact `'st sth sT ShT ShTh sht St'` → `স্ত স্থ স্ট ষ্ট ষ্ঠ শ্ত শ্ত` now match.
Longer `'sTh shth Sth'` → `স্টহ শ্তহ শ্তহ` leave `h` available:
`'sThm shthm Sthm sthm ShThm'` → `স্টহ্ম শ্তহ্ম শ্তহ্ম স্থম ষ্ঠম`.
Consumed `t`/`T` cannot start another rule; fresh keys can:
`'stn stm sttn sttm ShTm ShTTm'` → `স্তন স্তম স্তত্ন স্তত্ম ষ্টম ষ্টট্ম`.
Earlier matches retain priority (`kst` → `ক্সত`, `ksht` → `কশত`, `kSt` → `কশ্ত`,
`nst` → `ন্সত`, `nsht` → `নশত`, `kShT` → `ক্ষট`).
Vowels and following `r` retain their checked exceptions (`strri` → `স্তৃ`,
`sthrry` → `স্থরর‍্য`, `ShThry` → `ষ্ঠ্র্য`). Copied Bangla resets context
(`স্থa` → `স্থআ`). Case controls stay separate:
`'shT shTh ST STh Shth'` → `শট শঠ শট শঠ ষথ`.
Try `'rasta astha sTeshon koShTo niShTha proshtho'`
→ `রাস্তা আস্থা স্টেশন কষ্ট নিষ্ঠা প্রশ্তহ`, without dictionary correction.
Uppercase aliases also participate in these matches (`mB` → `ম্ব`).
Lowercase `ng` produces ং (U+0982), as in `bangla` → `বাংলা`. Before the supported
single-key vowels and `OI`/`OU`, it produces ঙ্গ plus the appropriate vowel sign
(`nga` → `ঙ্গা`, `ngOI` → `ঙ্গৈ`, `ngo` → `ঙ্গ`). These forms consume their vowel
first: `ngee` → `ঙ্গেএ`, `ngoo` → `ঙ্গঅ`, rather than using ordinary `ee`/`oo`.
`ngh` and `nng` now match as described above; other case variants remain incomplete. The linked behavior section preserves the earlier partial outputs.
The prefixes `Ngr`/`NGr`/`ngr` consume their first `r` before the remainder is
matched. This gives `Ngrri` → `ঙররি`, `NGrri` → `ঞররি`, and `ngrri` → `ংররি`.
One more `r` leaves a separate `rri` vowel (`Ngrrri` → `ঙরৃ`). This rule is
specific to these three prefixes; `khrri` still produces `খৃ`.
Exact uppercase `Y` produces য় (U+09DF), with supported vowel signs (`Yi` → `য়ি`,
`Yrri` → `য়ৃ`). Ordinary adjacency stays separate (`kY` → `কয়`, `YY` → `য়য়`).
Lowercase `y` produces `ইয়` independently, `য়` after a supported vowel, and `্য`
after a supported consonant (`y ay ky` → `ইয় আয় ক্য`). All forms allow following
vowel signs (`ya` → `ইয়া`, `kya` → `ক্যা`) and repeated `y` (`yy` → `ইয়্য`).
Basic `r`/`rr` adds a zero-width joiner U+200D before `্য` (`ry` → `র‍্য`),
while nasal-plus-`r` prefixes do not (`Ngry` → `ঙর্য`). Copied Unicode and digits
reset this context. Uppercase `Y` stays distinct: `kY` → `কয়`, `ky` → `ক্য`.
Uppercase `Z` adds `্য` directly, even at the start or after a copied character,
and allows vowel signs (`kZa` → `ক্যা`). It shares the basic `r`/`rr` joiner rule
(`rZ` → `র‍্য`). Longer `aZ`/`oZ` matches produce `অ্যা`/`অ্য` before ordinary
vowel handling, even after a consonant (`kaZ` → `কঅ্যা`, `koZ` → `কঅ্য`).
Existing `ng`-vowel matches still consume their vowel first (`ngaZ` → `ঙ্গা্য`).
The longer forms establish consonant context (`aZa` → `অ্যাা`); the engine
preserves these exact reference sequences without spelling correction.
After `y`/`Z`, two `r` keys add ্রর and are consumed before the rest is matched:
`yrrri` → `ইয়্রররি`, `yrrrri` → `ইয়্ররৃ`. Existing `rri` and `rry`/`rrZ`
matches keep priority (`yrri` → `ইয়ৃ`, `yrry` → `ইয়রর‍্য`). Single `r` stays
separate (`yr` → `ইয়র`), except before `y`/`Z` after uppercase `Z`:
`Zry` → `্য্র্য` has no U+200D joiner, while `yry` → `ইয়র‍্য` retains it.
The same rules apply after `aZ`/`oZ` (`aZrr` → `অ্যা্রর`). Uppercase `Y`
also attaches a single following `r` (`Yr` → `য়্র`, `Yry` → `য়্র্য`),
without a U+200D joiner. Its longer runs share the two-`r` matching priority:
`Yrrri` → `য়্রররি`, `Yrrrri` → `য়্ররৃ`, while `Yrri` → `য়ৃ` and
`Yrry` → `য়রর‍্য` keep their existing behavior. `AZ` now aliases `aZ`. Reph also supports `rrY` → `র্য়`.
Lowercase `q` produces ক (U+0995) and establishes consonant context for vowels:
`qi` → `কি`, `qOI` → `কৈ`, `qrri` → `কৃ`. Raw sequences keep their own matching:
`qh` → `কহ`, `qq` → `কক`, and existing `kh` → `খ`. Uppercase `Q` aliases `q`;
following `qr` → `ক্র`, `qry` → `ক্র্য`, `qrr` → `ক্রর`, and `qrry` → `করর‍্য`
match, with U+200D only in the last example.
Lowercase `v` produces ভ (U+09AD), sharing basic vowel context with `bh`:
`vi` → `ভি`, `vU` → `ভূ`, `vrri` → `ভৃ`. Repeated `v` and following `h`
stay separate (`vv` → `ভভ`, `vh` → `ভহ`). Following `r` now attaches
(`vr` → `ভ্র`). Uppercase `V` aliases `v`.
Lowercase `w` produces ও independently and after vowels, or ্ব after supported
consonants (`kw` → `ক্ব`). At the start or after a copied character/digit, it
adds য় before a following supported single-key vowel (`wa` → `ওয়া`, `wee` → `ওয়ী`).
After a vowel it adds only ও (`awa` → `আওা`). Every form establishes consonant
context: `wrri` → `ওৃ`, `ww` → `ও্ব`, `wy` → `ও্য`. Longer `aZ`/`oZ`
still match separately (`waZ` → `ওয়অ্যা`, `awaZ` → `আওঅ্যা`). Following `r`
shares lowercase `y`'s two-key priority: `wrr` → `ও্রর`, `wrrri` → `ও্রররি`,
and `wrrrri` → `ও্ররৃ`, while `wrri` → `ওৃ` and `wrry` → `ওরর‍্য` keep
their exceptions. Single `wr` stays separate (`wr` → `ওর`, `wry` → `ওর‍্য`).
The same rule applies after consonants/vowels (`kwrr` → `ক্ব্রর`, `awrr` → `আও্রর`).
Uppercase `W` aliases `w`; reph `rrw` → `র্্ব` preserves both U+09CD viramas.
Lowercase `x` produces এক্স at the start or after a copied character/digit,
and ক্স after supported converted input (`ax` → `আক্স`, `qx` → `কক্স`).
Exact `kx` matches before single `k` and produces ক্ষ. Both establish consonant
context for vowels (`xi` → `এক্সি`, `kxrri` → `ক্ষৃ`) and other supported forms
(`xx` → `এক্সক্স`, `xy` → `এক্স্য`, `kxY` → `ক্ষয়`). Raw matching stays distinct:
`qx` → `কক্স`, `khx` → `খক্স`, and `kx` → `ক্ষ`. Following `r` shares the
`y`/`w` two-key priority:
`xrr` → `এক্স্রর`, `xrrri` → `এক্স্রররি`, `kxrr` → `ক্ষ্রর`, with `rri` and
`rry`/`rrZ` exceptions preserved (`xrri` → `এক্সৃ`, `xrry` → `এক্সরর‍্য`).
Single `xr`/`kxr` stays separate and retains the basic-`r` joiner before `y`/`Z`.
Exact `Ngx` matches before `Ng` and produces ঙ্ষ (U+0999 U+09CD U+09B7),
with the same vowel context and following-`r` priority as `kx`:
`Ngxi` → `ঙ্ষি`, `Ngxrri` → `ঙ্ষৃ`, `Ngxrr` → `ঙ্ষ্রর`. Existing case
distinctions remain (`NGx` → `ঞক্স`, `ngx` → `ংক্স`). Some earlier longer
matches still take priority (`gNgx` uses `gN` first and now yields `গ্ণগক্স`). `kkx` now converts as `ক্কক্স`.

## Build and check the Fcitx5 experiment

This builds an experimental addon and runs synthetic input through the installed
Fcitx5 dispatcher. It does not install the addon, connect a desktop frontend, or
change input-method settings. Limited user-reported desktop smoke observations
are recorded in the [specification](docs/v1.md#fcitx5-local-feasibility-result);
the full frontend matrix is still unverified.

In addition to Rust, use Fcitx5 Core/Utils development headers **5.1.23 or newer**,
a C++20 compiler, CMake **3.25 or newer**, and Ninja. These are build requirements;
only the versions in the [environment record](docs/v1.md#fcitx5-local-feasibility-result)
have been validated. Cargo remains offline and adds no registry dependencies.

```sh
cmake -S adapters/fcitx5 -B target/fcitx5 -G Ninja -DCMAKE_BUILD_TYPE=Debug
cmake --build target/fcitx5
ctest --test-dir target/fcitx5 --output-on-failure
```

The build produces `target/fcitx5/libjong-bangla.so` and
`target/fcitx5/jong-fcitx5-harness`. The harness loads that actual shared addon,
allows only it and a test passthrough engine, and uses temporary XDG configuration,
cache and runtime directories. Build-local metadata contains the absolute library
path and is unsuitable for installation. There is deliberately no install target.
Only synthetic strings are recorded by the harness; the addon has no input logger.

Ten suites check editing, boundaries/shortcuts, releases/modifiers/repeats, resolved
text, invisible buffers, switching/focus/reset, context isolation, fallback preedit
and explicit preedit clicks. They verify UTF-8 cursor offsets and ordered
preedit/commit/application-key effects, including framework-owned shortcuts and
both focus-out commit owners. A simulated frontend commit is not evidence that a
real toolkit honors that path. Actual AltGr/compose completion, document relocation,
Wayland/XWayland delivery and desktop UI still need systematic smoke tests.
Use the [remaining desktop checklist](docs/v1.md#remaining-desktop-smoke-checklist)
to report those observations and application runtime details.
The user has manually confirmed that the updated Escape behavior works. For
repeat tests, `ki` then Escape should keep কি, and the next `i` should start a
fresh ই. Restart the private test daemon
with the rebuilt addon; an already loaded process keeps its old implementation.
The precise
[boundary contract and limits](docs/v1.md#fcitx5-local-feasibility-result) are recorded
alongside the product specification.

For C++ formatting (validated with clang-format 23.1.4):

```sh
clang-format --dry-run --Werror adapters/fcitx5/addon.cpp adapters/fcitx5/jong.h adapters/fcitx5/tests/harness.cpp
```

The Rust checks above include the C-ABI crate through `--workspace`. To repeat
the C++ AddressSanitizer/UndefinedBehaviorSanitizer check as well:

```sh
cmake -S adapters/fcitx5 -B target/fcitx5-sanitized -G Ninja -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_CXX_FLAGS='-fsanitize=address,undefined -fno-omit-frame-pointer' \
  -DCMAKE_SHARED_LINKER_FLAGS='-fsanitize=address,undefined' \
  -DCMAKE_EXE_LINKER_FLAGS='-fsanitize=address,undefined'
cmake --build target/fcitx5-sanitized
ASAN_OPTIONS=detect_leaks=0 ctest --test-dir target/fcitx5-sanitized --output-on-failure
```

These sanitizers instrument the C++ addon and harness, not the Rust static library
or the system Fcitx5 libraries. The command above disables leak detection because
LeakSanitizer fails under a ptrace execution sandbox; it is not a leak check.
Rust boundary tests separately check invalid scalar arguments, UTF-8 spans and
recovery after a caught panic. On Linux, workspace tests also run five isolated
failure scenarios: ordinary heap-payload cleanup, a panicking payload destructor,
double panics during operation/cleanup, and a panicking runtime hook. Expected
aborts stay in child processes with core dumps disabled. The ignored child helper
is invoked by the parent test; it needs no separate manual run.

The boundary attempts to release a caught payload once. If its destructor panics,
the replacement payload is deliberately retained, and the session is cleared.
Double panics and panicking hooks still abort; this policy is not a guarantee of
leak freedom or recovery from arbitrary destructor code. See the
[failure-policy evidence](docs/v1.md#c-abi-caught-panic-policy-review).

For a leak check, use the same sanitized build in an ordinary terminal outside
ptrace/debugger tracing, with Python 3 available:

```sh
python3 tools/check-memory.py target/fcitx5-sanitized
```

This first requires LeakSanitizer to detect an intentionally lost Rust session,
then runs 20,000 C-ABI lifecycle cycles and the ten dispatcher suites with
ASan/UBSan/leak detection enabled. It fails if the detector is unavailable, if
the build lacks leak detection, or if a normal workload reports an error. The
normal lifecycle test is also included in CTest. No suppression file is used.
Rust heap allocations are observed through the system allocator, as verified by
the deliberate leak; this does not instrument Rust memory accesses. These
synthetic tests do not exercise every failure path or a live desktop daemon.
See the [results and panic-path limit](docs/v1.md#local-resource-measurement-and-leak-check).

## Measure core latency

```sh
cargo run --offline --release --example measure > target/core-latency.csv
```

The dependency-free runner measures seven short inputs and mixed buffers of
32, 128, 1,024 and 8,192 Unicode scalars. It includes one-shot conversion,
short-word typing/commit and insert/Backspace pairs at long buffers' ends.
It prints nanoseconds per action: minimum, median and 95th percentile of 31
batch means after adaptive calibration. Those percentiles describe batches,
not individual keystroke tail latency. Allocations/output destruction are timed;
long-buffer setup is excluded. The runner rejects debug builds. Run it without
concurrent builds/tests and record hardware, toolchain and load conditions.
The [local measurements](docs/v1.md#local-resource-measurement-and-leak-check)
record two core release runs. Separate
[adapter measurements](docs/v1.md#synthetic-adapter-resource-measurements)
compare dispatch time and memory with an explicit Fcitx baseline.

## Measure synthetic adapter resources

On Linux/glibc, with Python 3 and `nm` from binutils available:

```sh
cmake -S adapters/fcitx5 -B target/fcitx5-release -G Ninja \
  -DCMAKE_BUILD_TYPE=Release -DJONG_BUILD_RESOURCE_PROBE=ON
cmake --build target/fcitx5-release
ctest --test-dir target/fcitx5-release --output-on-failure
python3 tools/measure-adapter.py target/fcitx5-release --output target/adapter-resources
```

Run measurements after builds/checks finish. The optional probe uses the real
dispatcher and addon, isolated temporary XDG paths and no desktop frontend.
Three sequential rounds alternate baseline/Jong order. Each mode/workload class
starts in a fresh process. The baseline loads only a static passthrough engine;
Jong adds its shared addon and input-method entry. Both receive identical keys
through identical synthetic contexts, with no retained document or effect log.
The runner rejects non-Release and sanitizer-instrumented builds, and
`LD_PRELOAD`/`LD_AUDIT`; the probe is disabled in normal builds.

`timing.csv` records nanoseconds per key press for four typing/Space cycles and
insert/Backspace pairs at 32/128/1,024/8,192-scalar buffer ends. It uses 31 batch
means with adaptive >=5 ms calibration; setup is excluded. The Jong-minus-baseline
difference includes conversion, the C ABI, adapter and synchronous preedit work;
it does not isolate bridge cost or individual-event tail latency.

`memory.csv` records eight checkpoints: idle, short word, short cleared, long
word, long cleared, 1,000/2,000 repeated sessions and context destruction. The
parent reads `/proc/PID/smaps_rollup` while the probe waits, capturing RSS, PSS and
private pages. `os_hwm_kib` preserves the kernel's approximate `VmHWM` counter;
the maximum snapshot RSS is only an observed peak, not an exact transient peak.
glibc `mallinfo2` records allocator-accounted used/reserved bytes, including
allocator caches rather than exact Rust capacities. No allocator trimming is
forced. `metadata.json` records build/source hashes, versions and method.
These results measure an isolated dispatcher, not a running desktop daemon or
frontend. See the [recorded results and limits](docs/v1.md#synthetic-adapter-resource-measurements).

## Repeat the reference comparison

This optional development check uses Node.js (verified with the installed
v24.21.0 LTS-line runtime). Node is not needed to build or use the Rust library.
Our [initial fixtures](tests/fixtures/reference.tsv),
[`a`/`i` fixtures](tests/fixtures/vowels.tsv),
[`u`/`U` fixtures](tests/fixtures/vowels-u.tsv),
[`I` fixtures](tests/fixtures/vowels-i-long.tsv),
[`e`/`ee` fixtures](tests/fixtures/vowels-e.tsv),
[`o`/`O`/`oo` fixtures](tests/fixtures/vowels-o.tsv),
[`OI`/`OU` fixtures](tests/fixtures/vowels-oi-ou.tsv),
[`rri` fixtures](tests/fixtures/vowels-rri.tsv),
[`r`/`rr` fixtures](tests/fixtures/consonants-r.tsv),
[`p`/`ph`/`f` fixtures](tests/fixtures/consonants-p-ph-f.tsv),
[`c`/`ch`/`j`/`jh` fixtures](tests/fixtures/consonants-c-ch-j-jh.tsv),
[`d`/`dh`/`D`/`Dh` fixtures](tests/fixtures/consonants-d-dh.tsv),
[`l`/`h` fixtures](tests/fixtures/consonants-l-h.tsv),
[`s`/`sh`/`S`/`Sh` fixtures](tests/fixtures/consonants-s-sh.tsv),
[`R`/`Rh` fixtures](tests/fixtures/consonants-r-retroflex.tsv),
[`z` fixtures](tests/fixtures/consonant-z.tsv),
[`Ng`/`NG` fixtures](tests/fixtures/consonants-ng-nj.tsv),
[`ng` fixtures](tests/fixtures/sign-ng.tsv),
[following-`rri` fixtures](tests/fixtures/nasal-rri.tsv),
[`Y` fixtures](tests/fixtures/consonant-y-uppercase.tsv),
[`y` fixtures](tests/fixtures/consonant-y-lowercase.tsv),
[`Z` fixtures](tests/fixtures/consonant-z-uppercase.tsv),
[following-`r` fixtures](tests/fixtures/ya-following-r.tsv),
[`Y` following-`r` fixtures](tests/fixtures/uppercase-y-following-r.tsv),
[`q` fixtures](tests/fixtures/consonant-q.tsv),
[`v` fixtures](tests/fixtures/consonant-v.tsv),
[`w` fixtures](tests/fixtures/consonant-w.tsv),
[`w` following-`r` fixtures](tests/fixtures/w-following-r.tsv),
[`x` fixtures](tests/fixtures/consonant-x.tsv),
[`x`/`kx` following-`r` fixtures](tests/fixtures/x-following-r.tsv),
[`Ngx` fixtures](tests/fixtures/conjunct-ngx.tsv),
[`Ngk` fixtures](tests/fixtures/conjunct-ngk.tsv),
[`Ngkh` fixtures](tests/fixtures/conjunct-ngkh.tsv),
[`Ngkx` fixtures](tests/fixtures/conjunct-ngkx.tsv),
[`Ngkkh` fixtures](tests/fixtures/conjunct-ngkkh.tsv),
[`NgkSh` fixtures](tests/fixtures/conjunct-ngksh.tsv),
[`kkh` fixtures](tests/fixtures/conjunct-kkh.tsv),
[`kSh` fixtures](tests/fixtures/conjunct-ksh.tsv),
[`kkhN` fixtures](tests/fixtures/conjunct-kkhn.tsv),
[`kShN` fixtures](tests/fixtures/conjunct-kshn.tsv),
[`kxN` fixtures](tests/fixtures/conjunct-kxn.tsv),
[`kkhm` fixtures](tests/fixtures/conjunct-kkhm.tsv),
[`kShm` fixtures](tests/fixtures/conjunct-kshm.tsv),
[`kxm` fixtures](tests/fixtures/conjunct-kxm.tsv),
[`kk` fixtures](tests/fixtures/conjunct-kk.tsv),
[`ShN` fixtures](tests/fixtures/conjunct-shn.tsv),
[`Shm` fixtures](tests/fixtures/conjunct-shm.tsv),
[`k` following-`r` fixtures](tests/fixtures/k-following-r.tsv),
[`kh` following-`r` fixtures](tests/fixtures/kh-following-r.tsv),
[`Sh` following-`r` fixtures](tests/fixtures/sh-following-r.tsv),
[`g` following-`r` fixtures](tests/fixtures/g-following-r.tsv),
[`gh` following-`r` fixtures](tests/fixtures/gh-following-r.tsv),
[`Ngg` fixtures](tests/fixtures/conjunct-ngg.tsv),
[`Nggh` fixtures](tests/fixtures/conjunct-nggh.tsv),
[`gg` fixtures](tests/fixtures/conjunct-gg.tsv),
[`gN` fixtures](tests/fixtures/conjunct-g-n.tsv),
[`NN` fixtures](tests/fixtures/conjunct-nn.tsv),
[`hN` fixtures](tests/fixtures/conjunct-h-n.tsv),
[`gn` fixtures](tests/fixtures/conjunct-gn.tsv),
[`ghn` fixtures](tests/fixtures/conjunct-ghn.tsv),
[`hn` fixtures](tests/fixtures/conjunct-hn.tsv),
[`dg` fixtures](tests/fixtures/conjunct-dg.tsv),
[`dgh` fixtures](tests/fixtures/conjunct-dgh.tsv),
[`Rg` fixtures](tests/fixtures/conjunct-r-g.tsv),
[`h` fixtures](tests/fixtures/h-following-r.tsv),
[`n` fixtures](tests/fixtures/n-following-r.tsv),
[`N` fixtures](tests/fixtures/n-retroflex-following-r.tsv),
[`ngh` fixtures](tests/fixtures/conjunct-ngh.tsv),
[`nn` fixtures](tests/fixtures/conjunct-n-lower-pair.tsv),
[`Nn` fixtures](tests/fixtures/conjunct-n-mixed-pair.tsv),
[`mn` fixtures](tests/fixtures/conjunct-mn.tsv),
[`hm` fixtures](tests/fixtures/conjunct-hm.tsv),
[`hl` fixtures](tests/fixtures/conjunct-hl.tsv),
[`nm` fixtures](tests/fixtures/conjunct-nm.tsv),
[`Nm` fixtures](tests/fixtures/conjunct-n-retroflex-m.tsv),
[`mm` fixtures](tests/fixtures/conjunct-mm.tsv),
[`ml` fixtures](tests/fixtures/conjunct-ml.tsv),
[`Ngm` fixtures](tests/fixtures/conjunct-ng-m.tsv),
[`gm` fixtures](tests/fixtures/conjunct-gm.tsv),
[`m` following-`r` fixtures](tests/fixtures/m-following-r.tsv),
[`l` following-`r` fixtures](tests/fixtures/l-following-r.tsv),
[`ddh` fixtures](tests/fixtures/conjunct-ddh.tsv),
[`dd` fixtures](tests/fixtures/conjunct-dd.tsv),
[`dhn` fixtures](tests/fixtures/conjunct-dhn.tsv),
[`dhm` fixtures](tests/fixtures/conjunct-dhm.tsv),
[`dm` fixtures](tests/fixtures/conjunct-dm.tsv),
[`dbh` fixtures](tests/fixtures/conjunct-dbh.tsv),
[`dv` fixtures](tests/fixtures/conjunct-dv.tsv),
[`DD` fixtures](tests/fixtures/conjunct-d-retroflex-pair.tsv),
[`d` following-`r` fixtures](tests/fixtures/d-following-r.tsv),
[`dh` following-`r` fixtures](tests/fixtures/dh-following-r.tsv),
[`D` following-`r` fixtures](tests/fixtures/d-retroflex-following-r.tsv),
[`Dh` following-`r` fixtures](tests/fixtures/dh-retroflex-following-r.tsv),
[`b` following-`r` fixtures](tests/fixtures/b-following-r.tsv),
[`bh` following-`r` fixtures](tests/fixtures/bh-following-r.tsv),
[`v` following-`r` fixtures](tests/fixtures/v-following-r.tsv),
[`ndh` fixtures](tests/fixtures/conjunct-ndh.tsv),
[`nd` fixtures](tests/fixtures/conjunct-nd.tsv),
[`nD` fixtures](tests/fixtures/conjunct-n-d-retroflex.tsv),
[`NDh` fixtures](tests/fixtures/conjunct-n-retroflex-dh-retroflex.tsv),
[`ND` fixtures](tests/fixtures/conjunct-n-retroflex-d-retroflex.tsv),
[`nth` fixtures](tests/fixtures/conjunct-nth.tsv),
[`nt` fixtures](tests/fixtures/conjunct-nt.tsv),
[`nTh` fixtures](tests/fixtures/conjunct-n-th-retroflex.tsv),
[`nT` fixtures](tests/fixtures/conjunct-n-t-retroflex.tsv),
[`NTh` fixtures](tests/fixtures/conjunct-n-retroflex-th-retroflex.tsv),
[`NT` fixtures](tests/fixtures/conjunct-n-retroflex-t-retroflex.tsv),
[`t` following-`r` fixtures](tests/fixtures/t-following-r.tsv),
[`th` following-`r` fixtures](tests/fixtures/th-following-r.tsv),
[`T` following-`r` fixtures](tests/fixtures/t-retroflex-following-r.tsv),
[`Th` following-`r` fixtures](tests/fixtures/th-retroflex-following-r.tsv),
[`tth` fixtures](tests/fixtures/conjunct-tth.tsv),
[`tt` fixtures](tests/fixtures/conjunct-tt.tsv),
[`tn` fixtures](tests/fixtures/conjunct-tn.tsv),
[`tm` fixtures](tests/fixtures/conjunct-tm.tsv),
[`TT` fixtures](tests/fixtures/conjunct-t-retroflex-pair.tsv),
[`Tm` fixtures](tests/fixtures/conjunct-t-retroflex-m.tsv),
[`kt` fixtures](tests/fixtures/conjunct-kt.tsv),
[`kT` fixtures](tests/fixtures/conjunct-k-t-retroflex.tsv),
[`kl` fixtures](tests/fixtures/conjunct-kl.tsv), and
[k-family controls](tests/fixtures/k-family-controls.tsv),
[`nkh` fixtures](tests/fixtures/conjunct-nkh.tsv),
[`nk` fixtures](tests/fixtures/conjunct-nk.tsv),
[`lkh` fixtures](tests/fixtures/conjunct-lkh.tsv),
[`lk` fixtures](tests/fixtures/conjunct-lk.tsv),
[`skl` fixtures](tests/fixtures/conjunct-skl.tsv),
[`skh` fixtures](tests/fixtures/conjunct-skh.tsv),
[`sk` fixtures](tests/fixtures/conjunct-sk.tsv),
[`Shk` fixtures](tests/fixtures/conjunct-sh-k.tsv),
[`ksh` fixtures](tests/fixtures/conjunct-ksh-priority.tsv),
[`ks` fixtures](tests/fixtures/conjunct-ks.tsv),
[`s` following-`r` fixtures](tests/fixtures/s-following-r.tsv),
[`sh` following-`r` fixtures](tests/fixtures/sh-lowercase-following-r.tsv),
[`S` following-`r` fixtures](tests/fixtures/s-uppercase-following-r.tsv),
[consuming k-family controls](tests/fixtures/consuming-k-controls.tsv),
[`gl` fixtures](tests/fixtures/conjunct-gl.tsv),
[`ll` fixtures](tests/fixtures/conjunct-ll.tsv),
[`bl` fixtures](tests/fixtures/conjunct-bl.tsv),
[`pl` fixtures](tests/fixtures/conjunct-pl.tsv),
[`sl` fixtures](tests/fixtures/conjunct-sl.tsv),
[`shl` fixtures](tests/fixtures/conjunct-sh-lowercase-l.tsv),
[`bhl` fixtures](tests/fixtures/conjunct-bhl.tsv),
[`phl` fixtures](tests/fixtures/conjunct-phl.tsv),
[`vl` fixtures](tests/fixtures/conjunct-vl.tsv),
[`fl` fixtures](tests/fixtures/conjunct-fl.tsv),
[`Sl` fixtures](tests/fixtures/conjunct-s-uppercase-l.tsv),
[`p` following-`r` fixtures](tests/fixtures/p-following-r.tsv),
[`ph` following-`r` fixtures](tests/fixtures/ph-following-r.tsv),
[`f` following-`r` fixtures](tests/fixtures/f-following-r.tsv),
[l-ending controls](tests/fixtures/l-ending-controls.tsv),
[`mbh` fixtures](tests/fixtures/conjunct-mbh.tsv),
[`mb` fixtures](tests/fixtures/conjunct-mb.tsv),
[`mv` fixtures](tests/fixtures/conjunct-mv.tsv),
[`mph` fixtures](tests/fixtures/conjunct-mph.tsv),
[`mpl` fixtures](tests/fixtures/conjunct-mpl.tsv),
[`mp` fixtures](tests/fixtures/conjunct-mp.tsv),
[`mf` fixtures](tests/fixtures/conjunct-mf.tsv),
[`lm` fixtures](tests/fixtures/conjunct-lm.tsv),
[`sm` fixtures](tests/fixtures/conjunct-sm.tsv),
[`shm` fixtures](tests/fixtures/conjunct-sh-lowercase-m.tsv),
[`Sm` fixtures](tests/fixtures/conjunct-s-uppercase-m.tsv),
[m-prefix and m-ending controls](tests/fixtures/m-labial-controls.tsv),
[`lgh` fixtures](tests/fixtures/conjunct-lgh.tsv),
[`lg` fixtures](tests/fixtures/conjunct-lg.tsv),
[`lbh` fixtures](tests/fixtures/conjunct-lbh.tsv),
[`lb` fixtures](tests/fixtures/conjunct-lb.tsv),
[`lv` fixtures](tests/fixtures/conjunct-lv.tsv),
[`lph` fixtures](tests/fixtures/conjunct-lph.tsv),
[`lp` fixtures](tests/fixtures/conjunct-lp.tsv),
[`lT` fixtures](tests/fixtures/conjunct-l-t-retroflex.tsv),
[`lD` fixtures](tests/fixtures/conjunct-l-d-retroflex.tsv),
[`ldh` fixtures](tests/fixtures/conjunct-ldh.tsv),
[l-prefix controls](tests/fixtures/l-prefix-controls.tsv),
[`sph` fixtures](tests/fixtures/conjunct-sph.tsv),
[`spl` fixtures](tests/fixtures/conjunct-spl.tsv),
[`sp` fixtures](tests/fixtures/conjunct-sp.tsv),
[`sf` fixtures](tests/fixtures/conjunct-sf.tsv),
[`Shph` fixtures](tests/fixtures/conjunct-sh-ph.tsv),
[`Shp` fixtures](tests/fixtures/conjunct-sh-p.tsv),
[`Shf` fixtures](tests/fixtures/conjunct-sh-f.tsv),
[s/Sh labial controls](tests/fixtures/s-labial-controls.tsv),
[`bb` fixtures](tests/fixtures/conjunct-bb.tsv),
[`pp` fixtures](tests/fixtures/conjunct-pp.tsv),
[`pn` fixtures](tests/fixtures/conjunct-pn.tsv),
[`sn` fixtures](tests/fixtures/conjunct-sn.tsv),
[`shn` fixtures](tests/fixtures/conjunct-sh-lowercase-n.tsv),
[`Sn` fixtures](tests/fixtures/conjunct-s-uppercase-n.tsv),
[repetition and n-ending controls](tests/fixtures/repeat-n-controls.tsv),
[`ns` fixtures](tests/fixtures/conjunct-ns.tsv),
[`nsh` fixtures](tests/fixtures/conjunct-nsh.tsv),
[`ps` fixtures](tests/fixtures/conjunct-ps.tsv),
[`psh` fixtures](tests/fixtures/conjunct-psh.tsv),
[ns/ps controls](tests/fixtures/ns-ps-controls.tsv),
[`st` fixtures](tests/fixtures/conjunct-st.tsv),
[`sth` fixtures](tests/fixtures/conjunct-sth.tsv),
[`sT` fixtures](tests/fixtures/conjunct-s-t-retroflex.tsv),
[`ShT` fixtures](tests/fixtures/conjunct-sh-t-retroflex.tsv),
[`ShTh` fixtures](tests/fixtures/conjunct-sh-th-retroflex.tsv),
[`sht` fixtures](tests/fixtures/conjunct-sh-lowercase-t.tsv),
[`St` fixtures](tests/fixtures/conjunct-s-uppercase-t.tsv),
[s/Sh stop controls](tests/fixtures/s-stop-controls.tsv), the 30 fixture
files linked in the [remaining conjunct batch](docs/v1.md#remaining-exact-conjunct-batch),
the 16 files linked in the [case-alias batch](docs/v1.md#selective-ascii-case-alias-batch),
the 13 files linked in the [explicit G/J batch](docs/v1.md#explicit-gj-spelling-batch),
the 12 files linked in the [symbol batch](docs/v1.md#punctuation-and-symbol-batch),
the 19 files linked in the [vowel-escape batch](docs/v1.md#vowel-sign-escape-batch),
the three files linked in the [final-escape record](docs/v1.md#final-t-escape),
the five files linked in the [q/z/R/Rh following-r record](docs/v1.md#following-r-after-q-z-r-and-rh),
and the four files linked in the [reph record](docs/v1.md#reph-batch)
record the raw input,
expected UTF-8 output, and exact Unicode code points. The comparison calls
`OmicronLab.Avro.Phonetic.parse(input)` directly, without browser UI, dictionaries,
or autocorrection.

Download the upstream parser and its MPL-1.1 license to a temporary directory,
outside this project. These downloads require network access; subsequent checks
run locally. The checker verifies the parser's SHA-256 before evaluating it.

```sh
reference_dir="$(mktemp -d)"
curl --fail --location 'https://raw.githubusercontent.com/torifat/jsAvroPhonetic/d3790172ebff92d43f0294bc51fed1fca7c80039/src/avro-lib.js' --output "$reference_dir/avro-lib.js"
curl --fail --location 'https://raw.githubusercontent.com/torifat/jsAvroPhonetic/d3790172ebff92d43f0294bc51fed1fca7c80039/MPL-1.1.txt' --output "$reference_dir/MPL-1.1.txt"
node tools/check-reference.mjs "$reference_dir/avro-lib.js"
```

It checks the same 522,933 parity cases used by Rust tests, then separately checks
60 [Unicode policy cases](tests/fixtures/unicode-case-policy.tsv). These record
independent engine/reference outputs and exact code points: 51 deliberate
differences and nine agreements. Rust tests check the engine columns; this script
checks the reference columns. Policy cases do not increase the parity count.
For example, the engine preserves `İ`, `É`, and `Σ`; the reference returns `ই̇`,
`é`, and `σ`. Expansion can also change phonetic matches: `rrİ` stays `ররİ`
here but becomes `ঋ̇` in the reference. See the
[decision and limits](docs/v1.md#unicode-case-preservation-policy).
The original comparison and test tooling are MIT;
the upstream parser remains external and is not shipped with the library.

To repeat the broader generated context audit, using the same external parser:

```sh
node tools/audit-context.mjs "$reference_dir/avro-lib.js" /tmp/jong-context-audit.json
```

This builds and runs the `audit_batch` Rust example offline. Its binary UTF-8
transport preserves empty strings, newlines, NULs and other copied characters.
The command writes counts, hashes and every discrepancy with exact code points;
it exits unsuccessfully for any unexpected difference. The three known case
probes retain their original explicit outputs under the settled preservation
policy; the 60 focused policy cases above provide broader evidence. The
[recorded report](docs/evidence/mixed-context-audit.json) contains the validated
results and tool versions. Group hashes cover ordered JSON arrays of
`[input, engine output, reference output]`; group counts overlap. Compare hashes
and counts when reproducing results with different tool versions.

## License

Project code is licensed under [MIT](LICENSE).
