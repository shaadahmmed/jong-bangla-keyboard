//! Conversion scanner and per-call context.
//!
//! Keep cross-family priority explicit here: consumed keys cannot begin a later
//! match. Rule tables and helpers must not reorder these branches.

mod following_r;
mod rules;
mod vowels;

use following_r::consume_following_r_prefix;
use rules::{
    CONSONANTS, CONSUMING_K_FORMS, CONSUMING_M_FORMS, D_CONJUNCTS, G_NASAL_CONJUNCTS, K_CONJUNCTS,
    L_CONJUNCTS, NASAL_H_CONJUNCTS, NASAL_R_FORMS, NASAL_STOP_CONJUNCTS, NG_VOWELS, T_CONJUNCTS,
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

/// Convert the supported sequences and digits, preserving unmatched characters.
///
/// At each position, verified sequence priority applies (`kh` before `k`,
/// `kkh` before `kk`, but `kk` consumes the first two keys of `kkkh`).
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
/// Exact Ngkx produces ঙ্ক্ষ before Ngk matching, sharing Ngx/kx context and
/// following-r priority: Ngkxr -> ঙ্ক্ষর, Ngkxrr -> ঙ্ক্ষ্রর, Ngkxrri -> ঙ্ক্ষৃ.
/// Exact Ngkkh/NgkSh also produce ঙ্ক্ষ before Ngk matching, sharing Ngkh's vowel
/// context and following-r attachment: Ngkkhr/NgkShr -> ঙ্ক্ষ্র,
/// Ngkkhrri/NgkShrri -> ঙ্ক্ষৃ. Case-distinct Ngksh still produces ঙ্কশ.
/// Exact kkh produces ক্ষ before basic k/kh, sharing Ngkh's consonant context
/// and following-r attachment: kkhr -> ক্ষ্র, kkhrri -> ক্ষৃ, kkhry -> ক্ষ্র্য.
/// Earlier Ngkkh still wins; NGkkh/ngkkh now convert as ঞক্ষ/ংক্ষ.
/// Exact kSh shares kkh's branch, vowel context, and following-r behavior:
/// kShr -> ক্ষ্র, kShrri -> ক্ষৃ, kShry -> ক্ষ্র্য. Earlier NgkSh still wins;
/// NGkSh/ngkSh convert as ঞক্ষ/ংক্ষ. Case-distinct ksh still produces কশ.
/// Exact kkhN/kShN/kxN produce ক্ষ্ণ before shorter kkh/kSh/kx, with shared
/// vowel context and single-r attachment: kxNr -> ক্ষ্ণ্র, kxNrri -> ক্ষ্ণৃ,
/// kxNry -> ক্ষ্ণ্র্য. Earlier Ngkkh/NgkSh/Ngkx retain priority. Consuming N
/// prevents a following Ng match: kShNgkSh -> ক্ষ্ণগক্ষ. A following uppercase
/// G remains unsupported (kShNG -> ক্ষ্ণG).
/// Exact kkhm/kShm/kxm produce ক্ষ্ম before shorter kkh/kSh/kx, with shared
/// vowel context and single-r attachment: kxmr -> ক্ষ্ম্র, kxmrri -> ক্ষ্মৃ,
/// kxmry -> ক্ষ্ম্র্য. Earlier Ngkkh/NgkSh/Ngkx still win (NgkShm -> ঙ্ক্ষম).
/// Consuming m prevents it joining a following nasal: kShmng -> ক্ষ্মং.
/// Exact kk produces ক্ক after longer kkh/kkhN/kkhm, with Ngk's vowel context
/// and following-r attachment: kkr -> ক্ক্র, kkrri -> ক্কৃ, kkry -> ক্ক্র্য.
/// Earlier Ngk still consumes k before kk: Ngkk -> ঙ্কক. Consuming kk prevents
/// a kkh/kSh/kx match starting at its second key: kkkh -> ক্কখ, kkx -> ক্কক্স.
/// Exact ShN/Shm produce ষ্ণ/ষ্ম before basic Sh, with vowel context and
/// single-r attachment: ShNr -> ষ্ণ্র, Shmrri -> ষ্মৃ, Shmry -> ষ্ম্র্য.
/// Consuming N prevents a following Ng match (ShNgx -> ষ্ণগক্স); consuming m
/// keeps a following nasal separate (Shmng -> ষ্মং). Earlier kShN/kShm and
/// NgkSh keep priority; kkShN/kkShm now produce ক্কষ্ণ/ক্কষ্ম.
/// Basic k/kh/Sh also attach following r: kr -> ক্র, khr -> খ্র, Shr -> ষ্র.
/// They retain rri/rry/rrZ exceptions and resolve kkkhr/kkShr/Ngkkr overlaps.
/// Basic g/gh share attachment and exceptions: gr -> গ্র, ghr -> ঘ্র, grri -> গৃ.
/// ShNgr/Shmgr/kShmgr attach the exposed g before r.
/// Exact Nggh/Ngg produce ঙ্ঘ/ঙ্গ before basic Ng and g/gh, using the same vowel
/// context and following-r helper: Ngghr -> ঙ্ঘ্র, Nggrri -> ঙ্গৃ. Nggh wins over
/// Ngg followed by h. Earlier ShN/kShN consume N first, leaving a separate gg.
/// Exact gg/gN/NN/hN/gn/ghn/hn/dg/dgh/Rg produce জ্ঞ/গ্ণ/ণ্ণ/হ্ণ/গ্ন/ঘ্ন/হ্ন/দ্গ/দ্ঘ/ড়্গ.
/// They share consonant context and following-r attachment, with dgh before dg
/// and ghn before basic gh. Consumed keys cannot start another match:
/// gNgg -> গ্ণজ্ঞ, dgn -> দ্গন, dggh -> দ্গঘ, kShNggh -> ক্ষ্ণজ্ঞহ.
/// Basic h/n/N also attach following r, including the exposed h in gghr -> জ্ঞহ্র.
/// Exact ngh/nn/Nn/mn/hm/hl/nm/Nm/mm/ml/Ngm/gm establish consonant context
/// and share following-r attachment, including l endings. Ngh wins before ng;
/// nn and mn consume n before a following g: nng -> ন্নগ, mng -> ম্নগ.
/// Earlier consuming matches still win: ShNgm -> ষ্ণগ্ম, Ngmm -> ঙ্মম.
/// Basic m/l also attach r, including the exposed m in NgkShmr -> ঙ্ক্ষম্র.
/// Exact dd/ddh/dhn/dhm/dm/dbh/dv/DD establish consonant context and attach r.
/// Longer ddh wins over dd, but dd consumes its d before dddh -> দ্দধ.
/// Basic d/dh/D/Dh/b/bh/v also attach following r, including the exposed
/// consonants in dddr -> দ্দদ্র, ddbhr -> দ্দভ্র, and DDDhr -> ড্ডঢ্র.
/// Exact nd/ndh/nD/ND/NDh/nt/nth/nT/NT/nTh/NTh establish consonant context
/// and attach r. Longer ndh/NDh/nth/nTh/NTh win over shorter forms; exact
/// nDh consumes nD first, leaving h. Earlier nasal matches retain priority:
/// nndh -> ন্নধ, ShNTh -> ষ্ণঠ. Consumed keys cannot start dd/dhn/dhm:
/// nddh -> ন্দধ, ndhn -> ন্ধন. Basic t/th/T/Th also attach following r,
/// including exposed stops in nntr -> ন্নত্র and kShNThr -> ক্ষ্ণঠ্র.
/// Exact tt/tth/tn/tm/TT/Tm establish consonant context and attach r.
/// Longer tth wins before tt; TT leaves a following h (TTh -> ট্টহ).
/// Consumed keys cannot start a second conjunct: ttn -> ত্তন, tng -> ত্নগ.
/// Earlier nasal stops keep priority: ntt -> ন্তত, nTm -> ন্টম.
/// Exact kt/kT/kl establish consonant context and attach r. kn/km stay separate.
/// Earlier Ngk/kk consume k first: Ngkt -> ঙ্কত, kkl -> ক্কল. kt/kT consume
/// t/T before t-family forms: ktm -> ক্তম, kTm -> ক্টম, kth -> ক্তহ.
/// Exact nk/nkh/lk/sk/skh/skl/Shk/ks establish consonant context and attach r.
/// Longer lkh/ksh preserve লখ/কশ before lk/ks could match. Earlier matches
/// retain priority: mnk -> ম্নক, NgkShk -> ঙ্ক্ষক. Consumed k cannot start
/// kt/kkh: nktm -> ঙ্কত্ম, nkkh -> ঙ্কখ; skl wins before sk: sklk -> স্ক্লক.
/// Basic s/sh/S attach following r, including exposed s in nksr -> ঙ্কস্র.
/// Exact gl/ll/bl/pl/sl/shl/bhl/phl/vl/fl/Sl establish consonant context and
/// attach r, retaining rri/rry/rrZ exceptions: glr -> গ্ল্র, blrri -> ব্লৃ.
/// Shl stays separate (ষল). Earlier consuming matches retain priority:
/// ggl -> জ্ঞল, Nggl -> ঙ্গল, dbhl -> দ্ভল, kshl -> কশল, skll -> স্ক্লল.
/// Consumed l cannot begin lk/ll/lT: blk -> ব্লক, gll -> গ্লল, blTm -> ব্লট্ম.
/// Basic p/ph/f also attach following r, including glpr/glphr/glfr.
/// Exact mb/mbh/mv/mp/mph/mf establish consonant context and attach r.
/// Longer mbh/mph win over mb/mp, while mpl preserves মপ্ল: mplk -> মপ্লক.
/// Earlier lm/sm/shm/Sm consume m before these forms: smp -> স্মপ,
/// shmpl -> শ্মপ্ল. They share the existing m-ending context and r exceptions.
/// Other earlier matches retain priority: Ngmb -> ঙ্মব, kkhmp -> ক্ষ্মপ,
/// sklm -> স্ক্লম, kshm -> কশম. Consumed keys cannot start a second match:
/// mbl -> ম্বল, mphl -> ম্ফল, mbbl -> ম্বব্ল, mppl -> ম্পপ্ল.
/// Earlier longer matches such as pn/sn/shn, ns/ps, labial conjuncts, bd, and reph remain gaps;
/// supported prefixes still convert inside them.
/// Backtick escapes remain unsupported.
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
