# Developer and agent guide

## Project

Jong Bangla (`jong-bangla-board`) is a Bangla phonetic desktop input method,
implemented from scratch with a reusable Rust engine. The first release targets
Fcitx5 on Linux, with CachyOS + KDE Plasma on Wayland as the first test environment.
IBus and Windows integrations are future work. Project code is MIT licensed.

Read [README.md](README.md), [docs/v1.md](docs/v1.md), and
[docs/progress.md](docs/progress.md) before making changes.
`docs/v1.md` distinguishes confirmed requirements from proposals and open decisions.
Do not turn a proposal into a requirement without explaining the decision.
User instructions take precedence over this guide; update the docs when they change.

## Collaborate in small steps

- Explain the purpose and expected result of the next small step before editing.
- Complete one coherent, reviewable increment at a time. Avoid combining engine
  changes, platform integration, packaging, and unrelated cleanup in one change.
- Walk the user through the result in plain language: what changed, why it matters,
  how it was checked, and what comes next. Explain unfamiliar terms when introduced.
- Update `docs/progress.md` after each coherent step: record the date, result,
  actual checks and their outcomes, remaining limitations, and the next action.
  Keep planned, in-progress, and completed work distinct. Never claim a check or
  milestone is complete without evidence.
- Commit each completed step after its checks and progress-log update, using a
  descriptive message that explains the change. Include only that step's work.
- Continue within the requested scope without adding routine approval gates.
  Ask concise questions when an unresolved choice materially changes behavior.
- Keep documentation-only tasks to documentation. Do not scaffold an application
  or add dependencies just to make a documentation change look complete.
- Do not delegate to other agents unless the user explicitly requests it.

## Product behavior

- Aim for Avro-compatible phonetic behavior, using an independently implemented
  engine. Choose and document the compatibility reference before claiming parity.
- Keep the active word in preedit: text remains editable before it is committed
  to the application. Backspace removes a raw phonetic keystroke and recomputes it.
- Show only two choices: converted Bangla first, exact original Latin input second.
  The Latin choice is a literal fallback, not an English translation or prediction.
- Convert digits to Bangla by default. Do not consume digit keys as candidate
  selection shortcuts by default.
- Keep future autocorrection separate from deterministic transliteration. Do not
  silently change spelling, add dictionaries, or implement prediction for v1.
- Prioritize responsive typing and low resource use; verify performance claims.

## Architecture boundaries

- Keep transliteration independent of Fcitx5, display servers, desktop APIs, and UI.
- Keep composition state and edit/commit actions independent of the platform
  adapter so future IBus and Windows adapters can share behavior.
- Keep framework-specific key events, preedit updates, candidate display, and
  commits in the adapter. Reuse Fcitx5 UI facilities where suitable.
- Rust is the implementation language for engine and composition logic. A minimal
  C++ bridge is a proposal to validate against Fcitx5's addon API, not a settled
  requirement. Do not move transliteration rules into a bridge.
- Add concrete extension points when needed. Do not build a dynamic plugin system
  or speculative IBus/Windows adapters for the first release.
- Keep unsafe code at a documented interoperability boundary when unavoidable.
  Specify ownership, lifetimes, encoding, error handling, and panic containment.

## Correctness and validation

- Store source and fixtures as UTF-8. Handle Rust strings at valid character
  boundaries; deleting a raw keystroke must not delete a byte from Bangla output.
- Add meaningful regression cases for phonetic rules and composition behavior:
  contextual vowels, conjuncts, case-sensitive sequences, punctuation/escape
  sequences, digits, raw Latin selection, backspace, commit, cancel, and reset.
- Track expected Unicode sequences and the source of compatibility fixtures.
  Visually similar output alone does not establish compatibility.
- Keep platform smoke tests separate from engine tests. Record actual distro,
  Plasma, Fcitx5, toolkit/application versions, and whether the app uses Wayland
  or XWayland. Do not describe an untested environment as supported.
- Once Rust code exists, use the repository's documented format, lint, and test
  commands. Until then, do not claim Cargo checks ran or invent setup commands.
- Run checks appropriate to the changed behavior and report checks that could
  not run. Documentation-only edits need link and consistency checks, not tests.
- Do not log or persist the user's typed text by default.

## Dependencies and documentation

- Prefer the latest stable development tools where practical. Verify current
  releases against official sources and record the versions actually used.
- Prefer small, justified dependencies. Keep per-keystroke processing local;
  typing must not depend on network access or a separate service.
- Write original project code under MIT. Check provenance and license obligations
  before introducing third-party source, rule tables, fixtures, or assets.
- Record behavioral decisions in `docs/v1.md`; update usage/build instructions
  alongside code. Explain tradeoffs without creating unnecessary documents.
- Never overwrite unrelated work or change system input-method settings as a
  side effect of development. Document installation separately from source edits.
