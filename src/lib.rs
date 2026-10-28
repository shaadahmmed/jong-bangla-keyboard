//! A small, deterministic Bangla phonetic conversion slice.
//!
//! Supports ASCII digits, the consonant sequences below, and lowercase `a`/`i`.
//! Unmatched Unicode characters pass through unchanged. This is not yet a
//! complete Avro-compatible engine: other vowels, conjuncts, case normalization,
//! punctuation rules, and escapes are not implemented.
//!
//! ```
//! use jong_bangla_core::transliterate;
//!
//! assert_eq!(transliterate("kh12T"), "খ১২ট");
//! assert_eq!(transliterate("t T th Th n N"), "ত ট থ ঠ ন ণ");
//! assert_eq!(transliterate("ami khani kia"), "আমি খানি কিয়া");
//! ```

// Independently authored initial mappings; not an imported upstream rule table.
const CONSONANTS: [(&str, &str); 13] = [
    ("k", "ক"),
    ("kh", "খ"),
    ("g", "গ"),
    ("gh", "ঘ"),
    ("t", "ত"),
    ("th", "থ"),
    ("T", "ট"),
    ("Th", "ঠ"),
    ("n", "ন"),
    ("N", "ণ"),
    ("b", "ব"),
    ("bh", "ভ"),
    ("m", "ম"),
];

#[derive(Clone, Copy)]
enum Context {
    Consonant,
    VowelI,
    Other,
}

/// Convert the supported sequences and digits, preserving unmatched characters.
///
/// At each position, the longest supported sequence wins (`kh` before `k`).
/// Lowercase `a`/`i` become vowel signs after a supported consonant and independent
/// vowels otherwise; `a` immediately after `i` becomes `য়া`. Context follows raw
/// matched tokens, not preserved Bangla. Matching is case-sensitive. Unsupported
/// larger sequences can still contain supported parts: `kkh` becomes `কখ`, not a
/// conjunct. Other vowels and backtick escapes remain unsupported.
/// The input is borrowed and never modified or retained.
pub fn transliterate(input: &str) -> String {
    let mut remaining = input;
    let mut output = String::with_capacity(input.len());
    let mut context = Context::Other;

    while let Some(ch) = remaining.chars().next() {
        let matched = CONSONANTS
            .iter()
            .filter(|(raw, _)| remaining.starts_with(raw))
            .max_by_key(|(raw, _)| raw.len());

        if let Some((raw, bangla)) = matched {
            output.push_str(bangla);
            remaining = &remaining[raw.len()..];
            context = Context::Consonant;
        } else if matches!(ch, 'a' | 'i') {
            output.push_str(match (ch, context) {
                ('a', Context::Consonant) => "া",
                ('i', Context::Consonant) => "ি",
                ('a', Context::VowelI) => "য়া",
                ('a', _) => "আ",
                _ => "ই",
            });
            context = if ch == 'i' {
                Context::VowelI
            } else {
                Context::Other
            };
            remaining = &remaining[ch.len_utf8()..];
        } else {
            output.push(match ch {
                '0' => '০',
                '1' => '১',
                '2' => '২',
                '3' => '৩',
                '4' => '৪',
                '5' => '৫',
                '6' => '৬',
                '7' => '৭',
                '8' => '৮',
                '9' => '৯',
                _ => ch,
            });
            // Advance by a whole UTF-8 character, including for preserved text.
            remaining = &remaining[ch.len_utf8()..];
            context = Context::Other;
        }
    }

    output
}
