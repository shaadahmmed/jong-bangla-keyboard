//! Reph depends on normalized raw keys, never the rendered Bangla shape.

/// Consume only the leading rr; leave the following spelling to the scanner.
pub(super) fn consume<'a>(
    remaining: &'a str,
    preceded_by_r: bool,
    output: &mut String,
) -> Option<&'a str> {
    if preceded_by_r {
        return None;
    }
    let after_rr = remaining.strip_prefix("rr")?;
    // Lowercase y and uppercase Z retain the basic rr/ya joiner. Vowels,
    // symbols and copied Unicode do not attach. Aliases are already normalized.
    let attaches = after_rr.starts_with([
        'b', 'c', 'd', 'D', 'f', 'g', 'G', 'h', 'j', 'J', 'k', 'l', 'm', 'n', 'N', 'p', 'q', 'R',
        's', 'S', 't', 'T', 'v', 'w', 'x', 'Y', 'z',
    ]);
    // Exactly terminal rrr is distinct from rrra, rrr! and longer r runs.
    if attaches || after_rr == "r" {
        output.push_str("র্");
        Some(after_rr)
    } else {
        None
    }
}
