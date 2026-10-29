//! Independently authored mappings; table order preserves verified priorities.

// Independently authored initial mappings; not an imported upstream rule table.
pub(super) const CONSONANTS: [(&str, &str); 40] = [
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
