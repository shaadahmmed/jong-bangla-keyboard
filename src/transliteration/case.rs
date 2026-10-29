//! Selective ASCII case aliases, independently checked against the pinned parser.

use std::borrow::Cow;

// Apply aliases before matching, so a capital inside a longer rule or exposed
// after consumed keys behaves the same way. G/J and the other capitals retain
// their explicit, case-sensitive meanings. Never lowercase arbitrary Unicode.
pub(super) fn normalize(input: &str) -> Cow<'_, str> {
    const ALIASES: &str = "ABCEFHKLMPQVWX";
    if input.bytes().any(|byte| ALIASES.as_bytes().contains(&byte)) {
        Cow::Owned(
            input
                .chars()
                .map(|ch| {
                    if ch.is_ascii() && ALIASES.contains(ch) {
                        ch.to_ascii_lowercase()
                    } else {
                        ch
                    }
                })
                .collect(),
        )
    } else {
        Cow::Borrowed(input)
    }
}
