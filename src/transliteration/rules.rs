//! Independently authored mappings; table order preserves verified priorities.

// Longer nsh/psh consume h without a conjunct. Their shorter ns/ps forms must
// not leave s available to later s-family matches. Final keys govern following r.
pub(super) const CONSUMING_S_FORMS: [(&str, &str, char); 4] = [
    ("nsh", "নশ", 'h'),
    ("ns", "ন্স", 's'),
    ("psh", "পশ", 'h'),
    ("ps", "প্স", 's'),
];

// Independently authored initial mappings; not an imported upstream rule table.
pub(super) const CONSONANTS: [(&str, &str); 43] = [
    ("k", "ক"),
    ("q", "ক"),
    ("kh", "খ"),
    ("g", "গ"),
    ("gh", "ঘ"),
    ("G", "গ"),
    ("Gh", "ঘ"),
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
    ("J", "জ"),
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

// Explicit uppercase-G matches, independently checked rather than case folded.
// Fresh G can begin these matches; consumed G in NG/cNG/jNG cannot. J has only
// a basic match: Jh/JJ stay separate consonants, unlike lowercase jh/jj.
pub(super) const EXPLICIT_G_CONJUNCTS: [(&str, &str, char); 9] = [
    ("Ghn", "ঘ্ন", 'n'),
    ("Gdh", "গ্ধ", 'h'),
    ("GN", "গ্ণ", 'N'),
    ("Gn", "গ্ন", 'n'),
    ("Gm", "গ্ম", 'm'),
    ("Gl", "গ্ল", 'l'),
    ("GG", "জ্ঞ", 'G'),
    ("Gg", "জ্ঞ", 'g'),
    ("gG", "জ্ঞ", 'G'),
];

// Independently authored ng-vowel forms checked against the pinned reference.
// Consume only one e/o: ngee -> ঙ্গেএ and ngoo -> ঙ্গঅ, unlike ordinary ee/oo.
pub(super) const NG_VOWELS: [(&str, &str, char); 10] = [
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
pub(super) const NASAL_R_FORMS: [(&str, &str); 3] = [("Ngr", "ঙর"), ("NGr", "ঞর"), ("ngr", "ংর")];

// Original exact spellings, independently checked for context and overlaps.
// The final raw key controls following-r attachment. Longer dgh precedes dg;
// earlier Ng/k/Sh matches still consume their keys before reaching this table.
pub(super) const G_NASAL_CONJUNCTS: [(&str, &str, char); 10] = [
    ("gg", "জ্ঞ", 'g'),
    ("gN", "গ্ণ", 'N'),
    ("NN", "ণ্ণ", 'N'),
    ("hN", "হ্ণ", 'N'),
    ("gn", "গ্ন", 'n'),
    ("ghn", "ঘ্ন", 'n'),
    ("hn", "হ্ন", 'n'),
    ("dgh", "দ্ঘ", 'h'),
    ("dg", "দ্গ", 'g'),
    ("Rg", "ড়্গ", 'g'),
];

// Original exact spellings, independently checked for each context and overlap.
// In particular ngh must consume all three keys before the shorter ng sign.
pub(super) const NASAL_H_CONJUNCTS: [(&str, &str, char); 12] = [
    ("ngh", "ঙ্ঘ", 'h'),
    ("nn", "ন্ন", 'n'),
    ("Nn", "ণ্ন", 'n'),
    ("mn", "ম্ন", 'n'),
    ("hm", "হ্ম", 'm'),
    ("hl", "হ্ল", 'l'),
    ("nm", "ন্ম", 'm'),
    ("Nm", "ণ্ম", 'm'),
    ("mm", "ম্ম", 'm'),
    ("ml", "ম্ল", 'l'),
    ("Ngm", "ঙ্ম", 'm'),
    ("gm", "গ্ম", 'm'),
];

// Original exact spellings with independent context and priority evidence.
// Longer ddh wins over dd; the final raw key governs following-r attachment.
pub(super) const D_CONJUNCTS: [(&str, &str, char); 8] = [
    ("ddh", "দ্ধ", 'h'),
    ("dd", "দ্দ", 'd'),
    ("dhn", "ধ্ন", 'n'),
    ("dhm", "ধ্ম", 'm'),
    ("dm", "দ্ম", 'm'),
    ("dbh", "দ্ভ", 'h'),
    ("dv", "দ্ভ", 'v'),
    ("DD", "ড্ড", 'D'),
];

// Original nasal-stop spellings, independently verified for Unicode and context.
// Longer ndh/NDh/nth/nTh/NTh precede their shorter forms. Exact nDh is absent:
// nD consumes D first, leaving h. Earlier nasal matches retain priority.
pub(super) const NASAL_STOP_CONJUNCTS: [(&str, &str, char); 11] = [
    ("ndh", "ন্ধ", 'h'),
    ("nd", "ন্দ", 'd'),
    ("nD", "ন্ড", 'D'),
    ("NDh", "ণ্ঢ", 'h'),
    ("ND", "ণ্ড", 'D'),
    ("nth", "ন্থ", 'h'),
    ("nt", "ন্ত", 't'),
    ("nTh", "ন্ঠ", 'h'),
    ("nT", "ন্ট", 'T'),
    ("NTh", "ণ্ঠ", 'h'),
    ("NT", "ণ্ট", 'T'),
];

// Original t-family spellings with independent context and priority evidence.
// Longer tth precedes tt. There is no TTh match: TT leaves h for a later rule.
// Earlier nasal-stop matches consume their t/T before reaching this table.
pub(super) const T_CONJUNCTS: [(&str, &str, char); 6] = [
    ("tth", "ত্থ", 'h'),
    ("tt", "ত্ত", 't'),
    ("tn", "ত্ন", 'n'),
    ("tm", "ত্ম", 'm'),
    ("TT", "ট্ট", 'T'),
    ("Tm", "ট্ম", 'm'),
];

// Original k-family spellings, independently checked for context and priority.
// Earlier Ngk/kk consume k first; kt/kT consume t/T before t-family matches.
// kn and km stay separate basic consonants; no exact kth or kTh rule exists.
pub(super) const K_CONJUNCTS: [(&str, &str, char); 3] =
    [("kt", "ক্ত", 't'), ("kT", "ক্ট", 'T'), ("kl", "ক্ল", 'l')];

// Original spellings verified separately for context and consuming priority.
// Longer nkh/skh/skl precede nk/sk. lkh/ksh preserve separate consonants
// before shorter lk/ks could insert a hasanta. Earlier nasal matches still win.
pub(super) const CONSUMING_K_FORMS: [(&str, &str, char); 10] = [
    ("nkh", "ঙ্খ", 'h'),
    ("nk", "ঙ্ক", 'k'),
    ("lkh", "লখ", 'h'),
    ("lk", "ল্ক", 'k'),
    ("skl", "স্ক্ল", 'l'),
    ("skh", "স্খ", 'h'),
    ("sk", "স্ক", 'k'),
    ("Shk", "ষ্ক", 'k'),
    ("ksh", "কশ", 'h'),
    ("ks", "ক্স", 's'),
];

// Original l-prefix forms, independently checked for context and priority.
// Longer lgh/lph preserve separate consonants before lg/lp could join them;
// both consume h (lghl -> লঘল, lphl -> লফল). lbh precedes lb.
// Earlier l-ending forms consume l first; these consume their final key before
// later g/l/labial/d/t-family matches can reuse it (lgl -> ল্গল, lTm -> ল্টম).
pub(super) const CONSUMING_L_FORMS: [(&str, &str, char); 10] = [
    ("lgh", "লঘ", 'h'),
    ("lg", "ল্গ", 'g'),
    ("lbh", "ল্ভ", 'h'),
    ("lb", "ল্ব", 'b'),
    ("lv", "ল্ভ", 'v'),
    ("lph", "লফ", 'h'),
    ("lp", "ল্প", 'p'),
    ("lT", "ল্ট", 'T'),
    ("lD", "ল্ড", 'D'),
    ("ldh", "ল্ধ", 'h'),
];

// Original exact l-ending spellings, each checked for context and priorities.
// Earlier gg/Ng/dg/dbh/dv/ks/ksh/skl matches consume shared keys first.
// Shl is separate consonants; Sl is an exact spelling, not case normalization.
pub(super) const L_CONJUNCTS: [(&str, &str); 11] = [
    ("gl", "গ্ল"),
    ("ll", "ল্ল"),
    ("bl", "ব্ল"),
    ("pl", "প্ল"),
    ("sl", "স্ল"),
    ("shl", "শ্ল"),
    ("bhl", "ভ্ল"),
    ("phl", "ফ্ল"),
    ("vl", "ভ্ল"),
    ("fl", "ফ্ল"),
    ("Sl", "শ্ল"),
];

// Original exact spellings, independently checked for Unicode and context.
// Longer mbh/mph precede mb/mp; mpl preserves মপ্ল before mp could join m+p.
// Earlier m-ending forms consume m first, including lm/sm/shm/Sm below.
pub(super) const CONSUMING_M_FORMS: [(&str, &str, char); 11] = [
    ("mbh", "ম্ভ", 'h'),
    ("mb", "ম্ব", 'b'),
    ("mv", "ম্ভ", 'v'),
    ("mph", "ম্ফ", 'h'),
    ("mpl", "মপ্ল", 'l'),
    ("mp", "ম্প", 'p'),
    ("mf", "ম্ফ", 'f'),
    ("lm", "ল্ম", 'm'),
    ("sm", "স্ম", 'm'),
    ("shm", "শ্ম", 'm'),
    ("Sm", "শ্ম", 'm'),
];

// Original s/Sh labial spellings, independently checked for context and priority.
// Longer sph/spl and Shph precede sp/Shp. Only spl consumes the final l;
// sphl/sfl/Shpl/Shphl/Shfl leave l available for a fresh l-prefix match.
// Earlier ks/kSh/NgkSh consume shared s/Sh before reaching these forms.
pub(super) const S_LABIAL_CONJUNCTS: [(&str, &str, char); 7] = [
    ("sph", "স্ফ", 'h'),
    ("spl", "স্প্ল", 'l'),
    ("sp", "স্প", 'p'),
    ("sf", "স্ফ", 'f'),
    ("Shph", "ষ্ফ", 'h'),
    ("Shp", "ষ্প", 'p'),
    ("Shf", "ষ্ফ", 'f'),
];

// Original s/Sh stop spellings, independently checked for context and overlaps.
// Longer sth/ShTh consume h before st/ShT. In contrast, sTh/shth/Sth leave h
// available after sT/sht/St. The consumed t/T cannot begin a later t/T rule.
pub(super) const S_STOP_CONJUNCTS: [(&str, &str, char); 7] = [
    ("sth", "স্থ", 'h'),
    ("st", "স্ত", 't'),
    ("sT", "স্ট", 'T'),
    ("ShTh", "ষ্ঠ", 'h'),
    ("ShT", "ষ্ট", 'T'),
    ("sht", "শ্ত", 't'),
    ("St", "শ্ত", 't'),
];

// Original repetition and n-ending spellings, each independently verified.
// bb/pp consume both keys before bh/ph/bl/pl can reuse the second one;
// following h/l remains available. pn/sn/shn/Sn consume n before nk/nn/etc.
// Exact Shn stays separate; equal shn/Sn output does not imply case folding.
pub(super) const REPEAT_N_CONJUNCTS: [(&str, &str, char); 6] = [
    ("bb", "ব্ব", 'b'),
    ("pp", "প্প", 'p'),
    ("pn", "প্ন", 'n'),
    ("sn", "স্ন", 'n'),
    ("shn", "শ্ন", 'n'),
    ("Sn", "শ্ন", 'n'),
];

// Independently authored remaining exact conjunct spellings. Longer bdh/cch/
// jjh/NGch/NGjh/nch/njh/shch/Sch consume h before their shorter counterparts.
// pt/pT/bj/cNG/jNG do not consume a following h. Earlier complete matches keep
// their shared keys; cNG/jNG end in raw G, not a standalone uppercase alias.
pub(super) const COMPLETION_CONJUNCTS: [(&str, &str, char); 25] = [
    ("pt", "প্ত", 't'),
    ("pT", "প্ট", 'T'),
    ("bdh", "ব্ধ", 'h'),
    ("bd", "ব্দ", 'd'),
    ("bj", "ব্জ", 'j'),
    ("cNG", "চ্ঞ", 'G'),
    ("cch", "চ্ছ", 'h'),
    ("cc", "চ্চ", 'c'),
    ("gdh", "গ্ধ", 'h'),
    ("jjh", "জ্ঝ", 'h'),
    ("jNG", "জ্ঞ", 'G'),
    ("jj", "জ্জ", 'j'),
    ("mth", "ম্থ", 'h'),
    ("NGch", "ঞ্ছ", 'h'),
    ("NGjh", "ঞ্ঝ", 'h'),
    ("NGc", "ঞ্চ", 'c'),
    ("NGj", "ঞ্জ", 'j'),
    ("nch", "ঞ্ছ", 'h'),
    ("njh", "ঞ্ঝ", 'h'),
    ("nj", "ঞ্জ", 'j'),
    ("nc", "ঞ্চ", 'c'),
    ("shch", "শ্ছ", 'h'),
    ("Sch", "শ্ছ", 'h'),
    ("shc", "শ্চ", 'c'),
    ("Sc", "শ্চ", 'c'),
];
