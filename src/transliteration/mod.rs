//! Conversion scanner and per-call context.
//!
//! Keep cross-family priority explicit here: consumed keys cannot begin a later
//! match. Rule tables and helpers must not reorder these branches.

mod following_r;
mod rules;
mod vowels;

use following_r::consume_following_r_prefix;
use rules::{
    CONSONANTS, CONSUMING_K_FORMS, CONSUMING_L_FORMS, CONSUMING_M_FORMS, CONSUMING_S_FORMS,
    D_CONJUNCTS, G_NASAL_CONJUNCTS, K_CONJUNCTS, L_CONJUNCTS, NASAL_H_CONJUNCTS, NASAL_R_FORMS,
    NASAL_STOP_CONJUNCTS, NG_VOWELS, REPEAT_N_CONJUNCTS, S_LABIAL_CONJUNCTS, S_STOP_CONJUNCTS,
    T_CONJUNCTS,
};

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

/// Convert supported phonetic sequences and ASCII digits to Bangla.
///
/// Matching is case-sensitive and follows verified rule priority. Unmatched
/// Unicode characters pass through without normalization; supported parts of
/// an otherwise unsupported sequence may still convert.
///
/// Vowel context comes only from raw tokens matched within this call, not from
/// copied Bangla. The input is never modified or retained, and calls share no
/// state. See the [crate documentation](crate) for coverage and behavior notes.
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
        } else if let Some((raw, bangla, last_key)) = NASAL_H_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
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
        } else if let Some(after_nggh) = remaining.strip_prefix("Nggh") {
            output.push_str("ঙ্ঘ");
            remaining = consume_following_r_prefix(after_nggh, 'h', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngg) = remaining.strip_prefix("Ngg") {
            output.push_str("ঙ্গ");
            remaining = consume_following_r_prefix(after_ngg, 'g', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngx) = remaining.strip_prefix("Ngx") {
            output.push_str("ঙ্ষ");
            remaining = consume_following_r_prefix(after_ngx, 'x', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngkh) = remaining.strip_prefix("Ngkh") {
            output.push_str("ঙ্খ");
            remaining = consume_following_r_prefix(after_ngkh, 'h', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngkx) = remaining.strip_prefix("Ngkx") {
            output.push_str("ঙ্ক্ষ");
            remaining = consume_following_r_prefix(after_ngkx, 'x', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_conjunct) = remaining
            .strip_prefix("Ngkkh")
            .or_else(|| remaining.strip_prefix("NgkSh"))
        {
            output.push_str("ঙ্ক্ষ");
            remaining = consume_following_r_prefix(after_conjunct, 'h', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_ngk) = remaining.strip_prefix("Ngk") {
            output.push_str("ঙ্ক");
            remaining = consume_following_r_prefix(after_ngk, 'k', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_conjunct) = remaining
            .strip_prefix("kkhN")
            .or_else(|| remaining.strip_prefix("kShN"))
            .or_else(|| remaining.strip_prefix("kxN"))
        {
            output.push_str("ক্ষ্ণ");
            remaining = consume_following_r_prefix(after_conjunct, 'N', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_conjunct) = remaining
            .strip_prefix("kkhm")
            .or_else(|| remaining.strip_prefix("kShm"))
            .or_else(|| remaining.strip_prefix("kxm"))
        {
            output.push_str("ক্ষ্ম");
            remaining = consume_following_r_prefix(after_conjunct, 'm', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_kx) = remaining.strip_prefix("kx") {
            output.push_str("ক্ষ");
            remaining = consume_following_r_prefix(after_kx, 'x', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_conjunct) = remaining
            .strip_prefix("kkh")
            .or_else(|| remaining.strip_prefix("kSh"))
        {
            output.push_str("ক্ষ");
            remaining = consume_following_r_prefix(after_conjunct, 'h', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_kk) = remaining.strip_prefix("kk") {
            // Keep kkh/kkhN/kkhm above kk; do not match again at its second k.
            output.push_str("ক্ক");
            remaining = consume_following_r_prefix(after_kk, 'k', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = K_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = CONSUMING_K_FORMS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_shn) = remaining.strip_prefix("ShN") {
            output.push_str("ষ্ণ");
            remaining = consume_following_r_prefix(after_shn, 'N', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some(after_shm) = remaining.strip_prefix("Shm") {
            output.push_str("ষ্ম");
            remaining = consume_following_r_prefix(after_shm, 'm', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = G_NASAL_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = D_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = NASAL_STOP_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = T_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = CONSUMING_S_FORMS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = REPEAT_N_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = CONSUMING_L_FORMS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla)) = L_CONJUNCTS
            .iter()
            .find(|(raw, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], 'l', &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = CONSUMING_M_FORMS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = S_LABIAL_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla, last_key)) = S_STOP_CONJUNCTS
            .iter()
            .find(|(raw, _, _)| remaining.starts_with(raw))
        {
            output.push_str(bangla);
            remaining = consume_following_r_prefix(&remaining[raw.len()..], *last_key, &mut output);
            context = Context::Consonant { ya_joiner: false };
        } else if let Some((raw, bangla)) = matched {
            output.push_str(bangla);
            remaining = &remaining[raw.len()..];
            // Only these basic spellings have independently verified attachment.
            let following_r_key = match *raw {
                "Y" => Some('Y'),
                "k" => Some('k'),
                "g" => Some('g'),
                "kh" | "Sh" | "gh" | "h" | "dh" | "Dh" | "bh" | "th" | "Th" | "sh" | "ph" => {
                    Some('h')
                }
                "n" => Some('n'),
                "N" => Some('N'),
                "m" => Some('m'),
                "l" => Some('l'),
                "d" => Some('d'),
                "D" => Some('D'),
                "b" => Some('b'),
                "v" => Some('v'),
                "t" => Some('t'),
                "T" => Some('T'),
                "s" => Some('s'),
                "S" => Some('S'),
                "p" => Some('p'),
                "f" => Some('f'),
                _ => None,
            };
            if let Some(last_key) = following_r_key {
                remaining = consume_following_r_prefix(remaining, last_key, &mut output);
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
        } else if let Some(after_vowel) = vowels::consume(remaining, &mut context, &mut output) {
            remaining = after_vowel;
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
