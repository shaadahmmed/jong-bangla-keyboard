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
`g`/`gh`/`k`/`kh`/`Sh`/`y`/`Z`/`Y`/`w`/`x`/`kx`/`kkh`/`kSh`/`Ngx`/`Ngk`/`Ngkh`/`Ngkx`/`Ngkkh`/`NgkSh`/`Ngg`/`Nggh`
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
This is a partial engine with 99,177 reference-checked examples,
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
[the consuming k-family batch](docs/v1.md#consuming-k-family-batch).

Start with [v1 requirements and development steps](docs/v1.md).
See [the progress handoff](docs/progress.md) for current state, the latest completed
step and checks, and the next task. Detailed change history is in Git.
Rule coverage favors the [largest coherent related batches](docs/v1.md#small-development-steps)
that remain reliable to validate and review. Look for 3–6 related spellings
initially and expand when practical; there is no fixed upper cap. Each spelling
needs independent reference evidence, with one full validation, documentation
update, and commit per completed batch. The next planned batch investigates
l-ending spellings around `gl`/`ll`/`bl`/`pl`/`sl`/`shl`, expanding to
`bhl`/`phl`/`vl` when their independent checks fit a coherent review. Check
longer priorities and exposed basic attachments before choosing the exact scope.
This is planned work.
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
full context, case aliases, and longer consonant sequences are still incomplete.
Supported parts of an otherwise unsupported word still convert: `Qa` currently
produces `Qআ` because uppercase `Q` is not implemented yet. Basic `r` produces `র`, and `rr`
produces `রর`; `rri` is matched before either. This resolves `rrri` → `রররি` and
`rrrri` → `ররৃ`. Basic `kr` now produces `ক্র`; reph remains incomplete
(`rrk` produces `ররক` instead of `র্ক`).
Lowercase `p` produces `প`; `ph` and `f` both produce `ফ`. The longest supported
match handles `ph` before `p` (`phi` → `ফি`, matching `fi` → `ফি`). Uppercase
aliases and larger sequences such as `pph` remain incomplete.
Lowercase `c`/`ch`/`j`/`jh` produce `চ`/`ছ`/`জ`/`ঝ`; `ch` and `jh` match before
their single-key prefixes. Uppercase aliases, conjuncts such as `cch` → `চ্ছ`,
and escapes remain incomplete.
Exact `d`/`dh`/`D`/`Dh` produce `দ`/`ধ`/`ড`/`ঢ`; `dh` and `Dh` match before
their single-key prefixes. Case distinguishes `da` → `দা` from `Da` → `ডা`.
`ddh` → `দ্ধ` now matches, as described below. Uppercase-`H` aliases
(`dH`/`DH`) and escapes remain incomplete.
Lowercase `l`/`h` produce `ল`/`হ` and establish vowel context (`li` → `লি`,
`hu` → `হু`). Existing two-key matches still win: `kh` → `খ`, `khh` → `খহ`.
Uppercase `L`/`H`, conjuncts such as `ll` → `ল্ল`, and escapes remain incomplete.
Exact `s` → স, `sh`/`S` → শ, and `Sh` → ষ follow the confirmed mapping.
`sh` matches before `s`, and `Sh` before `S`; all four support vowel signs.
Other conjuncts, uppercase-`H` aliases, and escapes remain incomplete.
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
`Ngkrry` → `ঙ্করর‍্য`). Earlier `gN` now wins in `gNgk` → `গ্ণগক`; reph,
uppercase aliases, and escapes remain gaps.
Exact `Ngkh` → `ঙ্খ` now matches before `Ngk` and shares its vowel context
and following-`r` priority (`Ngkhi` → `ঙ্খি`, `Ngkhr` → `ঙ্খ্র`,
`Ngkhrri` → `ঙ্খৃ`, `Ngkhrry` → `ঙ্খরর‍্য`). Earlier `gN` now wins in
`gNgkh` → `গ্ণগখ`; reph, uppercase aliases, and escapes remain gaps.
Exact `Ngkx` → `ঙ্ক্ষ` also matches before `Ngk`, sharing `Ngx`/`kx`
vowel context and following-`r` priority (`Ngkxi` → `ঙ্ক্ষি`,
`Ngkxrri` → `ঙ্ক্ষৃ`, `Ngkxrr` → `ঙ্ক্ষ্রর`). Single `Ngkxr` → `ঙ্ক্ষর`
stays separate and adds U+200D before `y`/`Z` (`Ngkxry` → `ঙ্ক্ষর‍্য`).
Earlier `gN` now wins in `gNgkx` → `গ্ণগক্ষ`.
Exact `Ngkkh` → `ঙ্ক্ষ` matches before `Ngk`, with vowel signs/aliases and
`Ngkh`'s following-`r` behavior (`Ngkkhr` → `ঙ্ক্ষ্র`, `Ngkkhrri` → `ঙ্ক্ষৃ`).
Its single following `r` attaches without a joiner (`Ngkkhry` → `ঙ্ক্ষ্র্য`),
while `Ngkxry` retains U+200D. Earlier `gN`/`NN`/`hN` overlaps now match;
reph, uppercase aliases, and escapes remain gaps.
Exact `NgkSh` shares `Ngkkh`'s output and vowel/following-`r` behavior
(`NgkShr` → `ঙ্ক্ষ্র`, `NgkShrri` → `ঙ্ক্ষৃ`, `NgkShry` → `ঙ্ক্ষ্র্য`).
Case-distinct `Ngksh` → `ঙ্কশ` stays separate. Earlier `gN`/`NN`/`hN` overlaps
now match; reph, uppercase aliases, and escapes remain gaps.
Exact `kkh` → `ক্ষ` matches before basic `k`/`kh`, sharing `Ngkh`'s vowel
context and following-`r` behavior (`kkhr` → `ক্ষ্র`, `kkhrri` → `ক্ষৃ`,
`kkhry` → `ক্ষ্র্য`). Earlier exact `Ngkkh` keeps priority;
`NGkkh` → `ঞক্ষ` and `ngkkh` → `ংক্ষ` now also match the reference.
Earlier `nk`/`lk`/`sk`/`Shk` now consume `k` first (`nkkh` → `ঙ্কখ`,
`lkkh` → `ল্কখ`, `skkh` → `স্কখ`, `Shkkh` → `ষ্কখ`). Other conjuncts,
reph, uppercase aliases, and punctuation/escapes remain gaps.
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
`kShmn` → `ক্ষ্মন`). Basic `mr`/`mm`/`mn` are now supported by the nasal/h
batch below. Earlier-prefix conflicts, uppercase aliases, reph, and
punctuation/escapes remain incomplete.
Exact `kk` → `ক্ক` matches after longer `kkh`/`kkhN`/`kkhm` and before single `k`.
Its vowel context and following `r` share `Ngk`'s behavior: `kki` → `ক্কি`,
`kkr` → `ক্ক্র`, `kkrri` → `ক্কৃ`, `kkry` → `ক্ক্র্য`, and `kkrry` → `ক্করর‍্য`.
Try `'Dhakka dhakka pakka akkel'` → `ঢাক্কা ধাক্কা পাক্কা আক্কেল`.
Consuming both keys changes `kkkh` → `ক্কখ`, `kkSh` → `ক্কষ`, and
`kkx` → `ক্কক্স`; the second key cannot begin `kkh`/`kSh`/`kx`.
Earlier `Ngk` still wins (`Ngkk` → `ঙ্কক`). Repeated `kk` and neighboring
supported forms work (`kkkk` → `ক্কক্ক`, `kkhkk` → `ক্ষক্ক`).
Earlier `nk`/`lk`/`sk`/`Shk`, uppercase aliases, reph, and escapes remain incomplete.
Exact `ShN` → `ষ্ণ` and `Shm` → `ষ্ম` match before basic `Sh`, with supported
vowel signs/aliases and following-`r` attachment: `ShNi` → `ষ্ণি`, `Shmr` → `ষ্ম্র`,
`ShNrri` → `ষ্ণৃ`, `Shmry` → `ষ্ম্র্য`, and `Shmrry` → `ষ্মরর‍্য`.
Try `'biShNu uShNo uShma'` → `বিষ্ণু উষ্ণ উষ্মা`.
Earlier `kShN`/`kShm` and `NgkSh` still win; `kkShN` → `ক্কষ্ণ` and
`kkShm` → `ক্কষ্ম` now match. Consuming `N` prevents a following `Ng` match:
`ShNgx` → `ষ্ণগক্স`, `ShNgkSh` → `ষ্ণগক্ষ`. Consuming `m` keeps a following
nasal separate: `Shmng` → `ষ্মং`, `Shmn` → `ষ্মন`. Uppercase `G` remains
unsupported after `ShN` (`ShNG` → `ষ্ণG`, reference `ষ্ণগ`). Basic
`mr`/`mm`/`mn` now match as described below; other case aliases, reph, and escapes
remain incomplete.
Basic `k`/`kh`/`Sh` now attach following `r`: try `'kr khr Shr'` → `ক্র খ্র ষ্র`.
Vowels and ya forms work (`kri` → `ক্রি`, `khry` → `খ্র্য`, `ShrZ` → `ষ্র্য`).
The `rri` and `rry`/`rrZ` exceptions remain (`krri` → `কৃ`, `krry` → `করর‍্য`);
`krr` → `ক্রর`. Existing longer matches retain priority, while exposed basic
spellings now attach: `kkkhr` → `ক্কখ্র`, `kkShr` → `ক্কষ্র`, `Ngkkr` → `ঙ্কক্র`.
Copied Bangla does not supply raw context (`কr` → `কর`); raw `qr` still gives
`কর`. Other basic following-`r` forms, uppercase aliases, reph, and escapes
remain incomplete.
Basic `g`/`gh` now attach following `r`: `'gr ghr'` → `গ্র ঘ্র`.
Vowels and ya forms keep consonant context (`grI` → `গ্রী`, `ghry` → `ঘ্র্য`).
The exceptions remain (`grri` → `গৃ`, `ghrry` → `ঘরর‍্য`), while `grr` → `গ্রর`.
Try `'grIShmo ghraN ghrriNa'` → `গ্রীষ্ম ঘ্রাণ ঘৃণা`.
Existing conjuncts retain priority; exposed `g` now attaches in `ShNgr` → `ষ্ণগ্র`,
`Shmgr` → `ষ্মগ্র`, and `kShmgr` → `ক্ষ্মগ্র`. Earlier `dg`/`Rg`/`gg`/`gN`
matches now have their own rules; other basic following-`r` forms, aliases,
reph, and escapes remain incomplete. Copied Unicode resets context (`গr` → `গর`, `ঘr` → `ঘর`).
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
context (`স্ক্লr` → `স্ক্লর`). Earlier `pn`/`sn`/`shn`, `bl`/`gl`/`ll`/`sl`,
and `ns`/`ps` remain gaps: `pnk`/`blk`/`nsk` give পঙ্ক/বল্ক/নস্ক, against
reference প্নক/ব্লক/ন্সক. `lTm` still gives লট্ম, against reference ল্টম.
Earlier b-prefixed matches (`bd`), `shn`/`shm`/`bhl`/`phl`/`shl`/`vl`,
other conjuncts and following-`r` forms, uppercase aliases, reph, and escapes
remain incomplete.
Lowercase `ng` produces ং (U+0982), as in `bangla` → `বাংলা`. Before the supported
single-key vowels and `OI`/`OU`, it produces ঙ্গ plus the appropriate vowel sign
(`nga` → `ঙ্গা`, `ngOI` → `ঙ্গৈ`, `ngo` → `ঙ্গ`). These forms consume their vowel
first: `ngee` → `ঙ্গেএ`, `ngoo` → `ঙ্গঅ`, rather than using ordinary `ee`/`oo`.
`ngh` and `nng` now match as described above; other case variants and escapes
remain incomplete. The linked behavior section preserves the earlier partial outputs.
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
stay separate (`vv` → `ভভ`, `vh` → `ভহ`). Following `r` now attaches
(`vr` → `ভ্র`). Uppercase `V` and escapes remain incomplete.
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
matches still take priority (`gNgx` uses `gN` first and now yields `গ্ণগক্স`). `kkx` now converts as `ক্কক্স`; reph, uppercase aliases, and escapes
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
[`S` following-`r` fixtures](tests/fixtures/s-uppercase-following-r.tsv), and
[consuming k-family controls](tests/fixtures/consuming-k-controls.tsv) record the raw input,
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

It checks the same 99,177 cases used by Rust tests and prints reference outputs for
eighteen known coverage gaps. The original comparison and test tooling are MIT;
the upstream parser remains external and is not shipped with the library.

## License

Project code is licensed under [MIT](LICENSE).
