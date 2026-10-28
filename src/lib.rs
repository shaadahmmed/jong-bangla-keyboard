//! A small, deterministic Bangla phonetic conversion slice.
//!
//! Supports ASCII digits, contextual lowercase `y`, lowercase `ng` with selected vowel forms,
//! the consonant sequences below, and
//! `a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo`/`OI`/`OU`/`rri`.
//! Unmatched Unicode characters pass through unchanged. This is not yet a
//! complete Avro-compatible engine: remaining consonants, other conjuncts, case
//! normalization, punctuation rules, and escapes are not implemented.
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
//! assert_eq!(transliterate("tumi kemon tOmake"), "তুমি কেমন তোমাকে");
//! assert_eq!(transliterate("OI kOI OU kOU nOUka"), "ঐ কৈ ঔ কৌ নৌকা");
//! assert_eq!(transliterate("rri krri krriti rriN"), "ঋ কৃ কৃতি ঋণ");
//! assert_eq!(transliterate("ram rani rrri rrrri"), "রাম রানি রররি ররৃ");
//! assert_eq!(transliterate("pani phOn fOn"), "পানি ফোন ফোন");
//! assert_eq!(transliterate("cabi chabi jani jhuri"), "চাবি ছাবি জানি ঝুরি");
//! assert_eq!(transliterate("din dhan Dan Dhaka"), "দিন ধান ডান ঢাকা");
//! assert_eq!(transliterate("lal holud bhalO"), "লাল হলুদ ভালো");
//! assert_eq!(transliterate("s sh S Sh"), "স শ শ ষ");
//! assert_eq!(transliterate("sokal shishu SiSu bhaSha"), "সকাল শিশু শিশু ভাষা");
//! assert_eq!(transliterate("baRi gaRi gaRh drriRh"), "বাড়ি গাড়ি গাঢ় দৃঢ়");
//! assert_eq!(transliterate("zai zabe zodi zokhon"), "যাই যাবে যদি যখন");
//! assert_eq!(transliterate("Ng NG aNgul raNga"), "ঙ ঞ আঙুল রাঙা");
//! assert_eq!(transliterate("bangla rong nga ngi"), "বাংলা রং ঙ্গা ঙ্গি");
//! assert_eq!(transliterate("Ngrri NGrrri ngrrrri"), "ঙররি ঞরৃ ংররররি");
//! assert_eq!(transliterate("zaY hoY hoYe maYa"), "যায় হয় হয়ে মায়া");
//! assert_eq!(transliterate("y ay ky kiya"), "ইয় আয় ক্য কিয়া");
//! ```

// Independently authored initial mappings; not an imported upstream rule table.
const CONSONANTS: [(&str, &str); 38] = [
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
    ("Ng", "ঙ"),
    ("NG", "ঞ"),
    ("b", "ব"),
    ("bh", "ভ"),
    ("m", "ম"),
    ("r", "র"),
    ("rr", "রর"),
    ("p", "প"),
    ("ph", "ফ"),
    ("f", "ফ"),
    ("c", "চ"),
    ("ch", "ছ"),
    ("j", "জ"),
    ("jh", "ঝ"),
    ("d", "দ"),
    ("dh", "ধ"),
    ("D", "ড"),
    ("Dh", "ঢ"),
    ("l", "ল"),
    ("h", "হ"),
    ("s", "স"),
    ("sh", "শ"),
    ("S", "শ"),
    ("Sh", "ষ"),
    ("R", "ড়"),
    ("Rh", "ঢ়"),
    ("z", "য"),
    ("Y", "য়"),
];

// Independently authored ng-vowel forms checked against the pinned reference.
// Consume only one e/o: ngee -> ঙ্গেএ and ngoo -> ঙ্গঅ, unlike ordinary ee/oo.
const NG_VOWELS: [(&str, &str, char); 10] = [
    ("OI", "ৈ", 'I'),
    ("OU", "ৌ", 'U'),
    ("a", "া", 'a'),
    ("i", "ি", 'i'),
    ("I", "ী", 'I'),
    ("u", "ু", 'u'),
    ("U", "ূ", 'U'),
    ("e", "ে", 'e'),
    ("o", "", 'o'),
    ("O", "ো", 'O'),
];

// Consume the first r with these nasal forms before matching the remaining rri.
// All three independently checked prefixes have the same ASCII length.
const NASAL_R_FORMS: [(&str, &str); 3] = [("Ngr", "ঙর"), ("NGr", "ঞর"), ("ngr", "ংর")];

#[derive(Clone, Copy)]
enum Context {
    Consonant {
        // Only basic r/rr adds ZWJ before lowercase y; nasal-r prefixes do not.
        ya_joiner: bool,
    },
    // Retain the raw vowel key: `ao` uses ও, but the last `o` in `ooo` uses অ.
    Vowel(char),
    Other,
}

/// Convert the supported sequences and digits, preserving unmatched characters.
///
/// At each position, the longest supported sequence wins (`kh` before `k`).
/// Most supported vowels become signs after a supported consonant; lowercase `o`
/// adds no character there. `ee` and `oo` alias `I` and `u`, matched before single
/// keys. Case-sensitive `OI`/`OU` match before single `O`, producing ঐ/ঔ or ৈ/ৌ.
/// Lowercase `rri` produces ঋ or ৃ before `rr`/`r` matching; `rr` produces রর.
/// Conjunct and reph forms involving `r` remain incomplete.
/// Lowercase `o` after a supported raw vowel other than `o` becomes ও.
/// An `a` after a supported vowel other than `a` becomes `য়া`. Context
/// follows raw matched tokens, not preserved Bangla. Matching is case-sensitive.
/// Lowercase `ng` produces ং, or ঙ্গ with a vowel sign in the selected ng-vowel
/// forms. These consume their vowel before ordinary ee/oo aliases are considered.
/// Ngr/NGr/ngr consume their first r before the rest is matched: Ngrri -> ঙররি,
/// Ngrrri -> ঙরৃ. Other consonants retain their existing rri matching.
/// Lowercase `y` produces ইয় independently, য় after a supported vowel, and ্য
/// after a supported consonant. Basic `r`/`rr` adds U+200D before ্য; nasal-r
/// prefixes do not. Every form allows following vowel signs and repeated y.
/// Unsupported larger sequences can still contain supported parts: `kkh` becomes `কখ`, not a
/// conjunct. Backtick escapes remain unsupported.
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

        let nasal_r = NASAL_R_FORMS
            .iter()
            .find(|(raw, _)| remaining.starts_with(raw));

        if let Some((raw, bangla)) = nasal_r {
            output.push_str(bangla);
            remaining = &remaining[raw.len()..];
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ng) = remaining.strip_prefix("ng") {
            let vowel = NG_VOWELS
                .iter()
                .filter(|(raw, _, _)| after_ng.starts_with(raw))
                .max_by_key(|(raw, _, _)| raw.len());
            if let Some((raw, sign, last_key)) = vowel {
                output.push_str("ঙ্গ");
                output.push_str(sign);
                context = Context::Vowel(*last_key);
                remaining = &after_ng[raw.len()..];
            } else {
                output.push('ং');
                // The raw match ends in g; ngr was handled before this branch.
                context = Context::Consonant { ya_joiner: false };
                remaining = after_ng;
            }
        } else if remaining.starts_with("rri") {
            // Match the three ASCII keys before individual consonants.
            output.push_str(if matches!(context, Context::Consonant { .. }) {
                "ৃ"
            } else {
                "ঋ"
            });
            context = Context::Vowel('i');
            remaining = &remaining[3..];
        } else if let Some((raw, bangla)) = matched {
            output.push_str(bangla);
            remaining = &remaining[raw.len()..];
            context = Context::Consonant {
                ya_joiner: matches!(*raw, "r" | "rr"),
            };
        } else if ch == 'y' {
            output.push_str(match context {
                Context::Consonant { ya_joiner: true } => "\u{200D}্য",
                Context::Consonant { ya_joiner: false } => "্য",
                Context::Vowel(_) => "য়",
                Context::Other => "ইয়",
            });
            remaining = &remaining[1..];
            context = Context::Consonant { ya_joiner: false };
        } else if remaining.starts_with("OI") || remaining.starts_with("OU") {
            // Consume both ASCII keys before single `O`; retain the final key.
            let (independent, sign, last_key) = if remaining.starts_with("OI") {
                ("ঐ", "ৈ", 'I')
            } else {
                ("ঔ", "ৌ", 'U')
            };
            output.push_str(if matches!(context, Context::Consonant { .. }) {
                sign
            } else {
                independent
            });
            context = Context::Vowel(last_key);
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
            output.push_str(match (vowel, context) {
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
            context = Context::Vowel(ch);
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
