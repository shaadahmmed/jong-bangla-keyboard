//! Vowel-sign escapes and ordinary context-sensitive vowels.
//!
//! The scanner handles higher-priority rri, ng-vowel, and aZ/oZ forms first.

use super::Context;

// Independently authored from reference behavior. Match the final backtick with
// the vowel; every sign is explicit, including the empty escaped lowercase o.
// The scanner keeps earlier ng/nasal-r consumption ahead of these matches.
pub(super) fn consume_escape<'a>(remaining: &'a str, output: &mut String) -> Option<&'a str> {
    for (raw, sign) in [
        ("rri`", "ৃ"),
        ("OI`", "ৈ"),
        ("OU`", "ৌ"),
        ("ee`", "ী"),
        ("oo`", "ু"),
        ("a`", "া"),
        ("i`", "ি"),
        ("I`", "ী"),
        ("u`", "ু"),
        ("U`", "ূ"),
        ("e`", "ে"),
        ("o`", ""),
        ("O`", "ো"),
    ] {
        if let Some(after) = remaining.strip_prefix(raw) {
            output.push_str(sign);
            return Some(after);
        }
    }
    None
}

// Return the unconsumed suffix only when an ordinary vowel matches.
pub(super) fn consume<'a>(
    mut remaining: &'a str,
    context: &mut Context,
    output: &mut String,
) -> Option<&'a str> {
    let ch = remaining.chars().next()?;
    if remaining.starts_with("OI") || remaining.starts_with("OU") {
        // Consume both ASCII keys before single `O`; retain the final key.
        let (independent, sign, last_key) = if remaining.starts_with("OI") {
            ("ঐ", "ৈ", 'I')
        } else {
            ("ঔ", "ৌ", 'U')
        };
        output.push_str(if matches!(*context, Context::Consonant { .. }) {
            sign
        } else {
            independent
        });
        *context = Context::Vowel(last_key);
        remaining = &remaining[2..];
    } else if matches!(ch, 'a' | 'i' | 'I' | 'u' | 'U' | 'e' | 'o' | 'O') {
        // Match repeated-key aliases before their single-key forms.
        let (vowel, width) = if remaining.starts_with("ee") {
            ('I', 2)
        } else if remaining.starts_with("oo") {
            ('u', 2)
        } else {
            (ch, ch.len_utf8())
        };
        output.push_str(match (vowel, *context) {
            ('a', Context::Consonant { .. }) => "া",
            ('i', Context::Consonant { .. }) => "ি",
            ('I', Context::Consonant { .. }) => "ী",
            ('u', Context::Consonant { .. }) => "ু",
            ('U', Context::Consonant { .. }) => "ূ",
            ('e', Context::Consonant { .. }) => "ে",
            ('o', Context::Consonant { .. }) => "",
            ('O', Context::Consonant { .. }) => "ো",
            ('a', Context::Vowel('a')) => "আ",
            ('a', Context::Vowel(_)) => "য়া",
            ('o', Context::Vowel('o')) => "অ",
            ('o', Context::Vowel(_)) => "ও",
            ('a', _) => "আ",
            ('i', _) => "ই",
            ('I', _) => "ঈ",
            ('u', _) => "উ",
            ('e', _) => "এ",
            ('o', _) => "অ",
            ('O', _) => "ও",
            _ => "ঊ",
        });
        // Both aliases repeat one key, so `ch` is also their last raw key.
        *context = Context::Vowel(ch);
        remaining = &remaining[width..];
    } else {
        return None;
    }
    Some(remaining)
}
