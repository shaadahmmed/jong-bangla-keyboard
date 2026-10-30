# Contributing to Jong Bangla

Jong Bangla is an experimental Bangla phonetic input method. Start with the
[README](README.md), the [current handoff](docs/progress.md#current-position),
and the relevant [requirements and behavior decisions](docs/v1.md).
[AGENTS.md](AGENTS.md) describes the detailed development workflow.

## Report a bug or suggest a change

Use the repository's issue tracker. Before opening an issue, check existing
reports and the [known desktop limitations](docs/v1.md#remaining-desktop-smoke-checklist).

For conversion bugs, provide a minimal synthetic phonetic input, expected and
actual Bangla output, and Unicode code points when visually similar text differs.
State the reference and revision used for the expected behavior. For desktop
bugs, include reproduction steps, distro, desktop, Fcitx5 and application/toolkit
versions, and whether the application uses Wayland or XWayland. Distinguish an
active editable word from text already committed to the application.

Use disposable sample text. Do not include passwords, private typed text or
unrelated application data in reports. Feature requests should explain the
desired typing behavior and whether they change a confirmed v1 decision.

## Prepare a change

Keep each change focused and independently reviewable. Follow the
[development guide](docs/development.md) for prerequisites and commands; run them
from the repository root. No network service is needed for ordinary Rust checks.

- Keep transliteration and composition independent of Fcitx5 and desktop APIs.
- Add meaningful regression cases for behavior changes. For phonetic rules,
  verify exact Unicode, context and overlapping matches against the
  [pinned reference](docs/reference-checks.md).
- Keep fixtures independently authored and record their provenance. Do not copy
  upstream source or rule tables without a separate license review.
- Keep typed text local and avoid input logging.

## Validate and submit

For Rust changes, run the documented format, Clippy, workspace test and release
build commands. Run the Fcitx5 checks when changing the adapter; keep synthetic
harness results separate from actual desktop observations. Use reference and
resource checks when relevant to the behavior or claims being changed. Report
any check you could not run.

For documentation-only changes, check relative links, heading anchors, command
consistency and `git diff --check`; application tests are unnecessary.

Update usage instructions with code changes, behavioral decisions in
[docs/v1.md](docs/v1.md), and the three current handoff sections in
[docs/progress.md](docs/progress.md). Add notable user-facing changes under
`Unreleased` in [CHANGELOG.md](CHANGELOG.md); routine internal steps belong in Git
and the handoff. Keep the changelog distinct from detailed validation records.

Commit each completed step after its checks and documentation updates. Pull
requests should explain the problem, resulting behavior, validation and material
limitations. Project code is [MIT licensed](LICENSE); retain applicable notices
and document any third-party obligations.
