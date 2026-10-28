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

A dependency-free Rust library now converts Bangla digits, 13 consonant
sequences, and `a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo` with context.
This is a partial engine with 473 reference-checked examples, not a usable desktop
keyboard or a full Avro compatibility claim. Other vowels, conjuncts, composition,
and Fcitx5 integration are still missing.
See the [reference decision](docs/v1.md#phonetic-behavior-reference) and
[initial slice](docs/v1.md#first-conversion-slice) plus the
[first vowel group](docs/v1.md#first-vowel-group) and
[`u`/`U` group](docs/v1.md#second-vowel-group), followed by
[uppercase `I`](docs/v1.md#third-vowel-group) and
[`e`/`ee`](docs/v1.md#fourth-vowel-group) and
[`o`/`O`/`oo`](docs/v1.md#fifth-vowel-group).

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
Supported parts of an otherwise unsupported word still convert: `qa` currently
produces `qআ` because `q` is not implemented yet. The two-key vowels `OI`/`OU`
remain unimplemented.

## Repeat the reference comparison

This optional development check uses Node.js (verified with the installed
v24.21.0 LTS-line runtime). Node is not needed to build or use the Rust library.
Our [initial fixtures](tests/fixtures/reference.tsv),
[`a`/`i` fixtures](tests/fixtures/vowels.tsv),
[`u`/`U` fixtures](tests/fixtures/vowels-u.tsv),
[`I` fixtures](tests/fixtures/vowels-i-long.tsv),
[`e`/`ee` fixtures](tests/fixtures/vowels-e.tsv), and
[`o`/`O`/`oo` fixtures](tests/fixtures/vowels-o.tsv) record the raw input, expected UTF-8
output, and exact Unicode code points. The comparison calls
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

It checks the same 473 cases used by Rust tests and prints reference outputs for
five known coverage gaps. The original comparison and test tooling are MIT;
the upstream parser remains external and is not shipped with the library.

## License

Project code is licensed under [MIT](LICENSE).
