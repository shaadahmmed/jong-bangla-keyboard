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

A dependency-free Rust library now converts Bangla digits, 34 consonant letters
through 37 basic sequences, lowercase `ng` with selected vowel forms, and
`a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo`/`OI`/`OU`/`rri`
with context, including `rri` after `Ng`/`NG`/`ng`. This is a partial engine with
2,150 reference-checked examples,
not a usable desktop keyboard or a full Avro compatibility claim. Remaining
letters, conjuncts, composition, and Fcitx5 integration are still missing.
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
[`rri` after `Ng`/`NG`/`ng`](docs/v1.md#following-rri-after-ng-ng-and-ng).

Start with [v1 requirements and development steps](docs/v1.md).
See [the progress log](docs/progress.md) for completed work, checks, and the next task.
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
Supported parts of an otherwise unsupported word still convert: `qa` currently
produces `qআ` because `q` is not implemented yet. Basic `r` produces `র`, and `rr`
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
It remains distinct from uppercase `Z`, whose reference form is ্য; uppercase
`Z`, related conjuncts, and escapes remain incomplete.
Exact `Ng`/`NG` produce ঙ/ঞ (U+0999/U+099E), matching before single `N` → ণ.
They support vowel signs (`Ngi` → `ঙি`, `NGU` → `ঞূ`). Conjuncts such as
`Ngk` → `ঙ্ক` and escapes remain incomplete.
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
[`ng` fixtures](tests/fixtures/sign-ng.tsv), and
[following-`rri` fixtures](tests/fixtures/nasal-rri.tsv) record the raw input,
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

It checks the same 2,150 cases used by Rust tests and prints reference outputs for
seven known coverage gaps. The original comparison and test tooling are MIT;
the upstream parser remains external and is not shipped with the library.

## License

Project code is licensed under [MIT](LICENSE).
