//! Verified punctuation matches, including literal symbol escapes.
//!
//! These are independently authored from reference behavior. Backtick is an
//! empty-output token, not a general instruction to copy the next character.

pub(super) fn consume<'a>(remaining: &'a str, output: &mut String) -> Option<&'a str> {
    // An ellipsis consumes three periods before a later period can escape.
    // Keep two periods ahead of one; double comma emits virama then ZWNJ.
    for (raw, converted) in [
        ("...", "..."),
        (".`", "."),
        ("..", "।।"),
        (".", "।"),
        (":`", ":"),
        (":", "ঃ"),
        ("^`", "^"),
        ("^", "ঁ"),
        (",,", "\u{09CD}\u{200C}"),
        ("$", "৳"),
        ("`", ""),
    ] {
        if let Some(after) = remaining.strip_prefix(raw) {
            output.push_str(converted);
            return Some(after);
        }
    }
    None
}
