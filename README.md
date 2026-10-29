# Jong Bangla

A Bangla phonetic desktop keyboard, built from scratch with Rust.

The working project name is `jong-bangla-board`; **Jong Bangla** is the proposed
display name.

The first release targets **Linux + Fcitx5 on Wayland**. The first test environment
is **CachyOS + KDE Plasma**. IBus and Windows support are planned for later.

## First-release experience

- Avro-compatible phonetic typing, with an independently implemented engine.
- An editable current word; Backspace removes the last phonetic keystroke.
- Two choices only: converted Bangla, then the exact original Latin text.
- Bangla digits by default.
- Fast, lightweight, offline typing.
- Room for modular autocorrection later; no automatic spelling changes or
  predictive suggestions in the initial implementation.

## Current status

A dependency-free Rust library now converts Bangla digits, 35 consonant letters
through 40 basic sequences, lowercase `ng` with selected vowel forms, and
`a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo`/`OI`/`OU`/`rri`
with context, including `rri` after `Ng`/`NG`/`ng`, plus contextual lowercase `y`/`w`/`x`
and uppercase `Z` with selected `aZ`/`oZ` forms. Following-`r` matching after
`y`/`Z`/`Y`/`w`/`x`/`kx`/`kkh`/`kSh`/`Ngx`/`Ngk`/`Ngkh`/`Ngkx`/`Ngkkh`/`NgkSh`
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
This is a partial engine with 12,933 reference-checked examples,
not a usable desktop keyboard or a full Avro compatibility claim. Other
letter forms, conjuncts, composition, and Fcitx5 integration are still missing.
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
[exact `ShN`/`Shm`](docs/v1.md#exact-shn-and-shm-conjuncts).

Start with [v1 requirements and development steps](docs/v1.md).
See [the progress handoff](docs/progress.md) for current state, the latest completed
step and checks, and the next task. Detailed change history is in Git.
Rule coverage now proceeds in [small batches of related conjuncts](docs/v1.md#small-development-steps),
with checks, documentation, and a commit for each batch. The next planned batch is
following `r` after basic `k`/`kh`/`Sh`, checked independently for each spelling.
Developers and coding agents should also read [AGENTS.md](AGENTS.md).

## Build and try the conversion slice

Use Rust/Cargo **1.99.1 or newer**, with rustfmt and Clippy. This initial baseline
is the toolchain actually validated, not evidence that older Rust cannot work.
The library uses the Rust 2024 edition and has no external dependencies.

From the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --offline --all-targets -- -D warnings
cargo test --offline
cargo build --offline --release
cargo run --offline --example convert -- 'kh12T'
```

The last command prints `খ১২ট`. Try `'t T th Th n N'` for `ত ট থ ঠ ন ণ`, or
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
full context, case aliases, and longer consonant sequences are still incomplete.
Supported parts of an otherwise unsupported word still convert: `Qa` currently
produces `Qআ` because uppercase `Q` is not implemented yet. Basic `r` produces `র`, and `rr`
produces `রর`; `rri` is matched before either. This resolves `rrri` → `রররি` and
`rrrri` → `ররৃ`. Conjunct/reph forms remain incomplete: `kr` currently produces
`কর` instead of `ক্র`, and `rrk` produces `ররক` instead of `র্ক`.
Lowercase `p` produces `প`; `ph` and `f` both produce `ফ`. The longest supported
match handles `ph` before `p` (`phi` → `ফি`, matching `fi` → `ফি`). Uppercase
aliases and larger sequences such as `pph` remain incomplete.
Lowercase `c`/`ch`/`j`/`jh` produce `চ`/`ছ`/`জ`/`ঝ`; `ch` and `jh` match before
their single-key prefixes. Uppercase aliases, conjuncts such as `cch` → `চ্ছ`,
and escapes remain incomplete.
Exact `d`/`dh`/`D`/`Dh` produce `দ`/`ধ`/`ড`/`ঢ`; `dh` and `Dh` match before
their single-key prefixes. Case distinguishes `da` → `দা` from `Da` → `ডা`.
Uppercase-`H` aliases (`dH`/`DH`), conjuncts such as `ddh` → `দ্ধ`, and
escapes remain incomplete.
Lowercase `l`/`h` produce `ল`/`হ` and establish vowel context (`li` → `লি`,
`hu` → `হু`). Existing two-key matches still win: `kh` → `খ`, `khh` → `খহ`.
Uppercase `L`/`H`, conjuncts such as `ll` → `ল্ল`, and escapes remain incomplete.
Exact `s` → স, `sh`/`S` → শ, and `Sh` → ষ follow the confirmed mapping.
`sh` matches before `s`, and `Sh` before `S`; all four support vowel signs.
Conjuncts such as `sk` → `স্ক`, uppercase-`H` aliases, and escapes remain incomplete.
Exact `R`/`Rh` produce ড়/ঢ় (U+09DC/U+09DD), with `Rh` matched before `R`.
They support vowel signs, and `Rrri` → `ড়ৃ` preserves the existing `rri` vowel.
Conjuncts such as `Rri` → `ড়্রি`, uppercase-`H` aliases, and escapes remain incomplete.
Lowercase `z` produces য (U+09AF), with supported vowel signs (`zi` → `যি`).
It remains distinct from uppercase `Z` → ্য (U+09CD U+09AF).
Related conjuncts and escapes remain incomplete.
Exact `Ng`/`NG` produce ঙ/ঞ (U+0999/U+099E), matching before single `N` → ণ.
They support vowel signs (`Ngi` → `ঙি`, `NGU` → `ঞূ`). Exact `Ngk` → `ঙ্ক`
now matches before `Ng` and supports vowels and aliases (`Ngki` → `ঙ্কি`).
It attaches single following `r` without U+200D (`Ngkr` → `ঙ্ক্র`, `Ngkry` → `ঙ্ক্র্য`),
sharing the two-`r` priority and exceptions (`Ngkrri` → `ঙ্কৃ`,
`Ngkrry` → `ঙ্করর‍্য`). Earlier-prefix conflicts such as `gNgk`, reph,
uppercase aliases, and escapes remain gaps.
Exact `Ngkh` → `ঙ্খ` now matches before `Ngk` and shares its vowel context
and following-`r` priority (`Ngkhi` → `ঙ্খি`, `Ngkhr` → `ঙ্খ্র`,
`Ngkhrri` → `ঙ্খৃ`, `Ngkhrry` → `ঙ্খরর‍্য`). Earlier-prefix conflicts
such as `gNgkh`, reph, uppercase aliases, and escapes remain gaps.
Exact `Ngkx` → `ঙ্ক্ষ` also matches before `Ngk`, sharing `Ngx`/`kx`
vowel context and following-`r` priority (`Ngkxi` → `ঙ্ক্ষি`,
`Ngkxrri` → `ঙ্ক্ষৃ`, `Ngkxrr` → `ঙ্ক্ষ্রর`). Single `Ngkxr` → `ঙ্ক্ষর`
stays separate and adds U+200D before `y`/`Z` (`Ngkxry` → `ঙ্ক্ষর‍্য`).
Earlier-prefix conflicts such as `gNgkx` remain gaps.
Exact `Ngkkh` → `ঙ্ক্ষ` matches before `Ngk`, with vowel signs/aliases and
`Ngkh`'s following-`r` behavior (`Ngkkhr` → `ঙ্ক্ষ্র`, `Ngkkhrri` → `ঙ্ক্ষৃ`).
Its single following `r` attaches without a joiner (`Ngkkhry` → `ঙ্ক্ষ্র্য`),
while `Ngkxry` retains U+200D. Earlier-prefix conflicts, reph, uppercase aliases,
and escapes remain gaps.
Exact `NgkSh` shares `Ngkkh`'s output and vowel/following-`r` behavior
(`NgkShr` → `ঙ্ক্ষ্র`, `NgkShrri` → `ঙ্ক্ষৃ`, `NgkShry` → `ঙ্ক্ষ্র্য`).
Case-distinct `Ngksh` → `ঙ্কশ` stays separate. Earlier-prefix conflicts, reph,
uppercase aliases, and escapes remain gaps.
Exact `kkh` → `ক্ষ` matches before basic `k`/`kh`, sharing `Ngkh`'s vowel
context and following-`r` behavior (`kkhr` → `ক্ষ্র`, `kkhrri` → `ক্ষৃ`,
`kkhry` → `ক্ষ্র্য`). Earlier exact `Ngkkh` keeps priority;
`NGkkh` → `ঞক্ষ` and `ngkkh` → `ংক্ষ` now also match the reference.
Earlier `nk`/`lk`/`sk`/`Shk` matches (`nkkh`, `lkkh`, `skkh`,
`Shkkh`), other conjuncts, reph,
uppercase aliases, and punctuation/escapes remain gaps.
Exact `kSh` shares `kkh`'s output, vowel context, and following-`r` behavior
(`kShr` → `ক্ষ্র`, `kShrri` → `ক্ষৃ`, `kShry` → `ক্ষ্র্য`). Earlier exact
`NgkSh` keeps priority; `NGkSh` → `ঞক্ষ` and `ngkSh` → `ংক্ষ` now match.
Case-distinct `ksh` → `কশ` stays separate. Earlier-prefix conflicts,
reph, uppercase aliases, and punctuation/escapes remain gaps.
Exact `kkhN`/`kShN`/`kxN` → `ক্ষ্ণ` match before their shorter forms, with
supported vowel signs/aliases and single-`r` attachment (`kxNr` → `ক্ষ্ণ্র`,
`kxNry` → `ক্ষ্ণ্র্য`). Two `r` keys normally add ্রর, while `rri` and
`rry`/`rrZ` keep priority (`kxNrri` → `ক্ষ্ণৃ`, `kxNrry` → `ক্ষ্ণরর‍্য`).
Earlier `Ngkkh`/`NgkSh`/`Ngkx` still win (`NgkShN` → `ঙ্ক্ষণ`). Consuming
the final `N` prevents a following nasal match (`kShNgkSh` → `ক্ষ্ণগক্ষ`);
this also resolves `kxNgx`, `kxNgk`, `kxNgkh`, `kxNgkx`, `kxNgkkh`, and `kxNgkSh`.
Uppercase `G` remains unsupported after these forms (`kShNG` → `ক্ষ্ণG`,
reference `ক্ষ্ণগ`); other uppercase aliases, earlier-prefix conflicts,
unrelated conjuncts, reph, and punctuation/escapes remain gaps.
Exact `kkhm`/`kShm`/`kxm` → `ক্ষ্ম` also match before their shorter forms,
with supported vowel signs/aliases and single-`r` attachment (`kxmr` → `ক্ষ্ম্র`,
`kxmry` → `ক্ষ্ম্র্য`). The `rri` and `rry`/`rrZ` exceptions keep priority
(`kxmrri` → `ক্ষ্মৃ`, `kxmrry` → `ক্ষ্মরর‍্য`). Try
`'lokkhmI lokShmI lokxmI sUkShmo'` → `লক্ষ্মী লক্ষ্মী লক্ষ্মী সূক্ষ্ম`.
Earlier `Ngkkh`/`NgkSh`/`Ngkx` still win (`NgkShm` → `ঙ্ক্ষম`).
Consuming `m` prevents it joining a following nasal (`kShmng` → `ক্ষ্মং`,
`kShmn` → `ক্ষ্মন`). Basic `mr`/`mm`/`mn`, earlier-prefix conflicts,
uppercase aliases, reph, and punctuation/escapes remain incomplete.
Exact `kk` → `ক্ক` matches after longer `kkh`/`kkhN`/`kkhm` and before single `k`.
Its vowel context and following `r` share `Ngk`'s behavior: `kki` → `ক্কি`,
`kkr` → `ক্ক্র`, `kkrri` → `ক্কৃ`, `kkry` → `ক্ক্র্য`, and `kkrry` → `ক্করর‍্য`.
Try `'Dhakka dhakka pakka akkel'` → `ঢাক্কা ধাক্কা পাক্কা আক্কেল`.
Consuming both keys changes `kkkh` → `ক্কখ`, `kkSh` → `ক্কষ`, and
`kkx` → `ক্কক্স`; the second key cannot begin `kkh`/`kSh`/`kx`.
Earlier `Ngk` still wins (`Ngkk` → `ঙ্কক`). Repeated `kk` and neighboring
supported forms work (`kkkk` → `ক্কক্ক`, `kkhkk` → `ক্ষক্ক`).
Basic `kh`/`Sh`/`k` followed by `r`, earlier `nk`/`lk`/`sk`/`Shk`, uppercase
aliases, reph, and escapes remain incomplete (`Ngkkr` gives `ঙ্ককর`, reference `ঙ্কক্র`).
Exact `ShN` → `ষ্ণ` and `Shm` → `ষ্ম` match before basic `Sh`, with supported
vowel signs/aliases and following-`r` attachment: `ShNi` → `ষ্ণি`, `Shmr` → `ষ্ম্র`,
`ShNrri` → `ষ্ণৃ`, `Shmry` → `ষ্ম্র্য`, and `Shmrry` → `ষ্মরর‍্য`.
Try `'biShNu uShNo uShma'` → `বিষ্ণু উষ্ণ উষ্মা`.
Earlier `kShN`/`kShm` and `NgkSh` still win; `kkShN` → `ক্কষ্ণ` and
`kkShm` → `ক্কষ্ম` now match. Consuming `N` prevents a following `Ng` match:
`ShNgx` → `ষ্ণগক্স`, `ShNgkSh` → `ষ্ণগক্ষ`. Consuming `m` keeps a following
nasal separate: `Shmng` → `ষ্মং`, `Shmn` → `ষ্মন`. Uppercase `G` remains
unsupported after `ShN` (`ShNG` → `ষ্ণG`, reference `ষ্ণগ`). Basic
`gr`/`mr`/`mm`/`mn`, other case aliases, reph, and escapes remain incomplete.
Lowercase `ng` produces ং (U+0982), as in `bangla` → `বাংলা`. Before the supported
single-key vowels and `OI`/`OU`, it produces ঙ্গ plus the appropriate vowel sign
(`nga` → `ঙ্গা`, `ngOI` → `ঙ্গৈ`, `ngo` → `ঙ্গ`). These forms consume their vowel
first: `ngee` → `ঙ্গেএ`, `ngoo` → `ঙ্গঅ`, rather than using ordinary `ee`/`oo`.
Longer `ngh`/`nng`, other case variants, and escapes remain
incomplete; see the linked behavior section for exact partial outputs.
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
`Yrry` → `য়রর‍্য` keep their existing behavior. Uppercase aliases such as
`AZ`, reph such as `rrY` → `র্য়`,
and escapes remain incomplete.
Lowercase `q` produces ক (U+0995) and establishes consonant context for vowels:
`qi` → `কি`, `qOI` → `কৈ`, `qrri` → `কৃ`. Raw sequences keep their own matching:
`qh` → `কহ`, `qq` → `কক`, and existing `kh` → `খ`. Uppercase `Q`, conjuncts
such as `qr` → `ক্র`, and escapes remain incomplete.
Lowercase `v` produces ভ (U+09AD), sharing basic vowel context with `bh`:
`vi` → `ভি`, `vU` → `ভূ`, `vrri` → `ভৃ`. Repeated `v` and following `h`
stay separate (`vv` → `ভভ`, `vh` → `ভহ`). Uppercase `V`, conjuncts such as
`vr` → `ভ্র`, and escapes remain incomplete.
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
Uppercase `W`, reph (`rrw` → `র্্ব`), other conjuncts, and escapes remain incomplete.
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
matches still take priority in the reference (`gNgx` uses `gN` first) and remain
gaps here. `kkx` now converts as `ক্কক্স`; reph, uppercase aliases, and escapes
remain incomplete.

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
[`ShN` fixtures](tests/fixtures/conjunct-shn.tsv), and
[`Shm` fixtures](tests/fixtures/conjunct-shm.tsv) record the raw input,
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

It checks the same 12,933 cases used by Rust tests and prints reference outputs for
nine known coverage gaps. The original comparison and test tooling are MIT;
the upstream parser remains external and is not shipped with the library.

## License

Project code is licensed under [MIT](LICENSE).
