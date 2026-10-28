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

Requirements and contributor guidance only. There is no Rust workspace, runnable
keyboard, installer, or validated compatibility claim yet.
The phonetic behavior reference is now pinned to jsAvroPhonetic v1.1.4; see the
[reference decision](docs/v1.md#phonetic-behavior-reference).

Start with [v1 requirements and development steps](docs/v1.md).
See [the progress log](docs/progress.md) for completed work, checks, and the next task.
Developers and coding agents should also read [AGENTS.md](AGENTS.md).

## License

Project code is licensed under [MIT](LICENSE).
