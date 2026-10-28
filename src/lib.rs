//! A small, deterministic Bangla phonetic conversion slice.
//!
//! Supports ASCII digits, contextual lowercase `y`/`w`/`x`, exact `kx`/`Ngx`/`Ngk`/`Ngkh`,
//! uppercase `Z` and `aZ`/`oZ`,
//! lowercase `ng` with selected vowel forms, the consonant sequences below, and
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
//! assert_eq!(transliterate("kZa aZ oZ baZ"), "ক্যা অ্যা অ্য বঅ্যা");
//! assert_eq!(transliterate("yrr yrri yry Zry"), "ইয়্রর ইয়ৃ ইয়র\u{200D}্য ্য্র্য");
//! assert_eq!(transliterate("Yr Yrri Yry Yrry"), "য়্র য়ৃ য়্র্য য়রর\u{200D}্য");
//! assert_eq!(transliterate("q qa qi qOI qotha"), "ক কা কি কৈ কথা");
//! assert_eq!(transliterate("v vi vU valO vUmi"), "ভ ভি ভূ ভালো ভূমি");
//! assert_eq!(transliterate("w wa awa kw kwa"), "ও ওয়া আওা ক্ব ক্বা");
//! assert_eq!(transliterate("wr wrri wrrri wrry"), "ওর ওৃ ও্রররি ওরর\u{200D}্য");
//! assert_eq!(transliterate("x xa axa kx kxa"), "এক্স এক্সা আক্সা ক্ষ ক্ষা");
//! assert_eq!(transliterate("xr xrr xrri xrrri kxrr"), "এক্সর এক্স্রর এক্সৃ এক্স্রররি ক্ষ্রর");
//! assert_eq!(transliterate("Ngx Ngxa Ngxrri Ngxrr"), "ঙ্ষ ঙ্ষা ঙ্ষৃ ঙ্ষ্রর");
//! assert_eq!(transliterate("Ngk Ngka Ngkr Ngkrri"), "ঙ্ক ঙ্কা ঙ্ক্র ঙ্কৃ");
//! assert_eq!(transliterate("Ngkh Ngkha Ngkhr Ngkhrri"), "ঙ্খ ঙ্খা ঙ্খ্র ঙ্খৃ");
//! ```

// Independently authored initial mappings; not an imported upstream rule table.
const CONSONANTS: [(&str, &str); 40] = [
    ("k", "ক"),
    ("q", "ক"),
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
    ("v", "ভ"),
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
        // Only basic r/rr adds ZWJ before y/Z; nasal-r prefixes do not.
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
/// Uppercase `Z` always adds ্য, with the same basic r/rr joiner exception.
/// Longer `aZ`/`oZ` produce অ্যা/অ্য before ordinary vowel handling; selected
/// ng-vowel forms still consume their vowel first. All establish consonant context.
/// After y/Z, two r keys produce ্রর before matching the rest, except that rri
/// and rry/rrZ keep priority. Zry/ZrZ add ্র without a joiner; yry/yrZ retain it.
/// Uppercase Y shares the two-r rule and exceptions; a single following r adds
/// ্র, including before y/Z, without setting the basic-r joiner flag.
/// Lowercase w produces ্ব after a consonant and ও otherwise. At a boundary,
/// it adds য় before a following supported single-key vowel (including aliases
/// beginning with that key). After a vowel it adds only ও. Every w establishes
/// consonant context for following signs; rri does not trigger the extra য়.
/// Following r keys share the y two-r rule and exceptions: wrr -> ও্রর,
/// wrri -> ওৃ, wrry -> ওরর‍্য. A single wr stays separate, including before y/Z.
/// Lowercase x produces এক্স at a boundary and ক্স after supported converted
/// input. Exact kx matches before k and produces ক্ষ. Both establish consonant
/// context without a joiner and share the y/w two-r rule and exceptions:
/// xrr -> এক্স্রর, xrri -> এক্সৃ, xrry -> এক্সরর‍্য. Single xr/kxr stay separate.
/// Exact Ngx produces ঙ্ষ before Ng matching, with the same consonant context
/// and following-r priority as kx. Exact Ngk produces ঙ্ক before Ng matching,
/// sharing Y's single-r attachment and two-r exceptions: Ngkr -> ঙ্ক্র,
/// Ngkrri -> ঙ্কৃ, Ngkry -> ঙ্ক্র্য, Ngkrry -> ঙ্করর‍্য.
/// Exact Ngkh produces ঙ্খ before Ngk matching, sharing Ngk's vowel context,
/// single-r attachment, and two-r exceptions: Ngkhr -> ঙ্খ্র, Ngkhrri -> ঙ্খৃ.
/// Earlier longer matches (such as gN in gNgx), Ngkx/Ngkkh/NgkSh,
/// kkx, and reph remain gaps; supported prefixes still convert inside them.
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
        } else if let Some(after_ngx) = remaining.strip_prefix("Ngx") {
            output.push_str("ঙ্ষ");
            remaining = consume_following_r_prefix(after_ngx, 'x', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngkh) = remaining.strip_prefix("Ngkh") {
            output.push_str("ঙ্খ");
            remaining = consume_following_r_prefix(after_ngkh, 'h', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngk) = remaining.strip_prefix("Ngk") {
            output.push_str("ঙ্ক");
            remaining = consume_following_r_prefix(after_ngk, 'k', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_kx) = remaining.strip_prefix("kx") {
            output.push_str("ক্ষ");
            remaining = consume_following_r_prefix(after_kx, 'x', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla)) = matched {
            output.push_str(bangla);
            remaining = &remaining[raw.len()..];
            if *raw == "Y" {
                remaining = consume_following_r_prefix(remaining, 'Y', &mut output);
            }
            context = Context::Consonant {
                ya_joiner: matches!(*raw, "r" | "rr"),
            };
        } else if matches!(ch, 'y' | 'Z') {
            output.push_str(match (ch, context) {
                (_, Context::Consonant { ya_joiner: true }) => "\u{200D}্য",
                ('Z', _) | (_, Context::Consonant { ya_joiner: false }) => "্য",
                (_, Context::Vowel(_)) => "য়",
                (_, Context::Other) => "ইয়",
            });
            remaining = &remaining[1..];
            remaining = consume_following_r_prefix(remaining, ch, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if ch == 'x' {
            output.push_str(if matches!(context, Context::Other) {
                "এক্স"
            } else {
                "ক্স"
            });
            remaining = &remaining[1..];
            remaining = consume_following_r_prefix(remaining, 'x', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if ch == 'w' {
            remaining = &remaining[1..];
            output.push_str(match context {
                Context::Consonant { .. } => "্ব",
                Context::Other
                    if remaining.starts_with(['a', 'i', 'I', 'u', 'U', 'e', 'o', 'O']) =>
                {
                    "ওয়"
                }
                _ => "ও",
            });
            remaining = consume_following_r_prefix(remaining, 'w', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if remaining.starts_with("aZ") || remaining.starts_with("oZ") {
            // Match both ASCII keys before contextual a/o, even after a consonant.
            output.push_str(if ch == 'a' {
                "অ্যা"
            } else {
                "অ্য"
            });
            remaining = &remaining[2..];
            remaining = consume_following_r_prefix(remaining, 'Z', &mut output);
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

// Independently checked following-r prefixes depend on the raw y/Z/Y/w/x key,
// including Z in aZ/oZ, x in kx/Ngx, k in Ngk, and h in Ngkh (not basic k/h).
// Do not infer them from copied Bangla.
fn consume_following_r_prefix<'a>(
    remaining: &'a str,
    raw_key: char,
    output: &mut String,
) -> &'a str {
    if remaining.starts_with("rr")
        && !["rri", "rry", "rrZ"]
            .iter()
            .any(|prefix| remaining.starts_with(prefix))
    {
        output.push_str("্রর");
        &remaining[2..]
    } else if (matches!(raw_key, 'Y' | 'k' | 'h')
        && remaining.starts_with('r')
        && !remaining.starts_with("rr"))
        || (raw_key == 'Z' && (remaining.starts_with("ry") || remaining.starts_with("rZ")))
    {
        // Y/Ngk/Ngkh attach a single r; Z only before y/Z. All omit the joiner.
        output.push_str("্র");
        &remaining[1..]
    } else {
        remaining
    }
}
