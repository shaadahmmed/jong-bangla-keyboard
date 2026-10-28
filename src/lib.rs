//! A small, deterministic Bangla phonetic conversion slice.
//!
//! Supports ASCII digits, the consonant sequences below, and
//! `a`/`i`/`I`/`u`/`U`/`e`/`ee`.
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
//! assert_eq!(transliterate("tumi bhUmi kua"), "তুমি ভূমি কুয়া");
//! assert_eq!(transliterate("ki kI nIti"), "কি কী নীতি");
//! assert_eq!(transliterate("ami ke kee"), "আমি কে কী");
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
    // A following `a` inserts য় after i/I/u/U/e/ee.
    VowelWithYa,
    Other,
}

/// Convert the supported sequences and digits, preserving unmatched characters.
///
/// At each position, the longest supported sequence wins (`kh` before `k`).
/// The supported vowels become signs after a supported consonant and independent
/// vowels otherwise; `ee` is an alias for long `I`, matched before single `e`.
/// An `a` immediately after `i`/`I`/`u`/`U`/`e`/`ee` becomes `য়া`. Context
/// follows raw matched tokens, not preserved Bangla. Matching is case-sensitive.
/// Unsupported larger sequences can still contain supported parts: `kkh` becomes `কখ`, not a
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
        } else if matches!(ch, 'a' | 'i' | 'I' | 'u' | 'U' | 'e') {
            // Match the two ASCII keys as one long vowel before handling `e`.
            let (vowel, width) = if remaining.starts_with("ee") {
                ('I', 2)
            } else {
                (ch, ch.len_utf8())
            };
            output.push_str(match (vowel, context) {
                ('a', Context::Consonant) => "া",
                ('i', Context::Consonant) => "ি",
                ('I', Context::Consonant) => "ী",
                ('u', Context::Consonant) => "ু",
                ('U', Context::Consonant) => "ূ",
                ('e', Context::Consonant) => "ে",
                ('a', Context::VowelWithYa) => "য়া",
                ('a', _) => "আ",
                ('i', _) => "ই",
                ('I', _) => "ঈ",
                ('u', _) => "উ",
                ('e', _) => "এ",
                _ => "ঊ",
            });
            context = if matches!(vowel, 'i' | 'I' | 'u' | 'U' | 'e') {
                Context::VowelWithYa
            } else {
                Context::Other
            };
            remaining = &remaining[width..];
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
