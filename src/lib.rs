//! A small, deterministic Bangla phonetic conversion slice.
//!
//! Supports ASCII digits and only the consonant sequences listed below.
//! Unmatched Unicode characters pass through unchanged. This is not yet a
//! complete Avro-compatible engine: vowels, conjuncts, case normalization,
//! punctuation rules, and escapes are not implemented.
//!
//! ```
//! use jong_bangla_core::transliterate;
//!
//! assert_eq!(transliterate("kh12T"), "খ১২ট");
//! assert_eq!(transliterate("t T th Th n N"), "ত ট থ ঠ ন ণ");
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

/// Convert the supported sequences and digits, preserving unmatched characters.
///
/// At each position, the longest supported sequence wins (`kh` before `k`).
/// Matching is case-sensitive. Unsupported larger sequences can still contain
/// supported parts: `ka` becomes `কa`, and `kkh` becomes `কখ`, not a conjunct.
/// The input is borrowed and never modified or retained.
pub fn transliterate(input: &str) -> String {
    let mut remaining = input;
    let mut output = String::with_capacity(input.len());

    while let Some(ch) = remaining.chars().next() {
        let matched = CONSONANTS
            .iter()
            .filter(|(raw, _)| remaining.starts_with(raw))
            .max_by_key(|(raw, _)| raw.len());

        if let Some((raw, bangla)) = matched {
            output.push_str(bangla);
            remaining = &remaining[raw.len()..];
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
        }
    }

    output
}
