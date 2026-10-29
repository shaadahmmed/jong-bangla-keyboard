use jong_bangla_core::transliterate;

#[test]
fn ngg_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngg.tsv"));
}

#[test]
fn nggh_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-nggh.tsv"));
}

#[test]
fn ngg_nggh_context_resets_at_boundaries_and_between_calls() {
    for (raw, bangla, last) in [("Ngg", "ঙ্গ", "গ"), ("Nggh", "ঙ্ঘ", "ঘ")] {
        for boundary in [' ', '\n', '\r', '\t', '\0', '🙂', 'ঙ', '\u{301}'] {
            assert_eq!(
                transliterate(&format!("Ng{boundary}{}", &raw[2..])),
                format!("ঙ{boundary}{last}")
            );
            assert_eq!(
                transliterate(&format!("{raw}{boundary}a")),
                format!("{bangla}{boundary}আ")
            );
            assert_eq!(
                transliterate(&format!("{raw}{boundary}r")),
                format!("{bangla}{boundary}র")
            );
            assert_eq!(
                transliterate(&format!("{raw}r{boundary}y")),
                format!("{bangla}্র{boundary}ইয়")
            );
        }
        assert_eq!(transliterate(raw), bangla);
        assert_eq!(transliterate("a"), "আ");
        assert_eq!(transliterate("r"), "র");
        assert_eq!(transliterate("y"), "ইয়");
    }
    assert_eq!(
        transliterate(&"Ngg Nggh Nggr Ngghr Nggrri Ngghry 🙂 ".repeat(64)),
        "ঙ্গ ঙ্ঘ ঙ্গ্র ঙ্ঘ্র ঙ্গৃ ঙ্ঘ্র্য 🙂 ".repeat(64)
    );
}

#[test]
fn g_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/g-following-r.tsv"));
}

#[test]
fn gh_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/gh-following-r.tsv"));
}

#[test]
fn k_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/k-following-r.tsv"));
}

#[test]
fn kh_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/kh-following-r.tsv"));
}

#[test]
fn sh_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/sh-following-r.tsv"));
}

#[test]
fn basic_following_r_resets_at_boundaries_and_between_calls() {
    for (raw, bangla) in [
        ("k", "ক"),
        ("kh", "খ"),
        ("Sh", "ষ"),
        ("g", "গ"),
        ("gh", "ঘ"),
    ] {
        for boundary in [' ', '\n', '\r', '\t', '\0', '🙂', 'ক', '\u{301}'] {
            assert_eq!(
                transliterate(&format!("{raw}{boundary}r")),
                format!("{bangla}{boundary}র")
            );
            assert_eq!(
                transliterate(&format!("{raw}r{boundary}y")),
                format!("{bangla}্র{boundary}ইয়")
            );
            assert_eq!(
                transliterate(&format!("{raw}r{boundary}a")),
                format!("{bangla}্র{boundary}আ")
            );
        }
        assert_eq!(transliterate(&format!("{raw}r")), format!("{bangla}্র"));
        assert_eq!(transliterate("y"), "ইয়");
        assert_eq!(transliterate("a"), "আ");
        assert_eq!(transliterate("r"), "র");
    }
}

#[test]
fn authored_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/reference.tsv"));
}

#[test]
fn vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels.tsv"));
}

#[test]
fn u_vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels-u.tsv"));
}

#[test]
fn long_i_vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels-i-long.tsv"));
}

#[test]
fn e_vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels-e.tsv"));
}

#[test]
fn o_vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels-o.tsv"));
}

#[test]
fn oi_ou_vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels-oi-ou.tsv"));
}

#[test]
fn rri_vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels-rri.tsv"));
}

#[test]
fn r_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-r.tsv"));
}

#[test]
fn p_ph_f_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-p-ph-f.tsv"));
}

#[test]
fn c_ch_j_jh_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-c-ch-j-jh.tsv"));
}

#[test]
fn d_dh_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-d-dh.tsv"));
}

#[test]
fn l_h_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-l-h.tsv"));
}

#[test]
fn s_sh_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-s-sh.tsv"));
}

#[test]
fn retroflex_r_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-r-retroflex.tsv"));
}

#[test]
fn z_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-z.tsv"));
}

#[test]
fn ng_ng_consonant_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonants-ng-nj.tsv"));
}

#[test]
fn lowercase_ng_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/sign-ng.tsv"));
}

#[test]
fn nasal_rri_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/nasal-rri.tsv"));
}

#[test]
fn uppercase_y_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-y-uppercase.tsv"));
}

#[test]
fn lowercase_y_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-y-lowercase.tsv"));
}

#[test]
fn uppercase_z_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-z-uppercase.tsv"));
}

#[test]
fn ya_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/ya-following-r.tsv"));
}

#[test]
fn uppercase_y_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/uppercase-y-following-r.tsv"));
}

#[test]
fn lowercase_q_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-q.tsv"));
}

#[test]
fn lowercase_v_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-v.tsv"));
}

#[test]
fn lowercase_w_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-w.tsv"));
}

#[test]
fn w_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/w-following-r.tsv"));
}

#[test]
fn lowercase_x_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/consonant-x.tsv"));
}

#[test]
fn x_following_r_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/x-following-r.tsv"));
}

#[test]
fn ngx_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngx.tsv"));
}

#[test]
fn ngk_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngk.tsv"));
}

#[test]
fn ngkh_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngkh.tsv"));
}

#[test]
fn ngkx_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngkx.tsv"));
}

#[test]
fn ngkkh_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngkkh.tsv"));
}

#[test]
fn ngksh_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ngksh.tsv"));
}

#[test]
fn kkh_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kkh.tsv"));
}

#[test]
fn ksh_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-ksh.tsv"));
}

#[test]
fn kkhn_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kkhn.tsv"));
}

#[test]
fn kshn_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kshn.tsv"));
}

#[test]
fn kxn_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kxn.tsv"));
}

#[test]
fn kkhm_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kkhm.tsv"));
}

#[test]
fn kshm_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kshm.tsv"));
}

#[test]
fn kxm_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kxm.tsv"));
}

#[test]
fn kk_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-kk.tsv"));
}

#[test]
fn shn_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-shn.tsv"));
}

#[test]
fn shm_conjunct_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/conjunct-shm.tsv"));
}

#[test]
fn sh_nasal_context_resets_at_boundaries_and_between_calls() {
    for (raw, bangla, last) in [("ShN", "ষ্ণ", "ণ"), ("Shm", "ষ্ম", "ম")] {
        for boundary in [' ', '\n', '\r', '\t', '\0', '🙂', 'ক', '\u{301}'] {
            assert_eq!(
                transliterate(&format!("{raw}{boundary}a")),
                format!("{bangla}{boundary}আ")
            );
            assert_eq!(
                transliterate(&format!("Sh{boundary}{}", &raw[2..])),
                format!("ষ{boundary}{last}")
            );
            assert_eq!(
                transliterate(&format!("{raw}{boundary}r")),
                format!("{bangla}{boundary}র")
            );
            assert_eq!(
                transliterate(&format!("{raw}r{boundary}y")),
                format!("{bangla}্র{boundary}ইয়")
            );
        }
        assert_eq!(transliterate(raw), bangla);
        assert_eq!(transliterate("a"), "আ");
        assert_eq!(transliterate("r"), "র");
        assert_eq!(transliterate("y"), "ইয়");
    }
}

#[test]
fn kk_context_resets_at_boundaries_and_between_calls() {
    for boundary in [' ', '\n', '\r', '\t', '\0', '🙂', 'ক', '\u{301}'] {
        assert_eq!(
            transliterate(&format!("kk{boundary}a")),
            format!("ক্ক{boundary}আ")
        );
        assert_eq!(
            transliterate(&format!("k{boundary}k")),
            format!("ক{boundary}ক")
        );
        assert_eq!(
            transliterate(&format!("kk{boundary}r")),
            format!("ক্ক{boundary}র")
        );
    }
    assert_eq!(transliterate("kk"), "ক্ক");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("r"), "র");
    assert_eq!(transliterate("k"), "ক");
}

#[test]
fn m_conjunct_context_resets_at_boundaries_and_between_calls() {
    for raw in ["kkhm", "kShm", "kxm"] {
        for boundary in ['\0', '\n', '\r', '\t', '🙂', 'ক', '\u{301}'] {
            let shorter = &raw[..raw.len() - 1];
            assert_eq!(
                transliterate(&format!("{shorter}{boundary}m")),
                format!("ক্ষ{boundary}ম")
            );
            assert_eq!(
                transliterate(&format!("{raw}{boundary}a")),
                format!("ক্ষ্ম{boundary}আ")
            );
            assert_eq!(
                transliterate(&format!("{raw}r{boundary}y")),
                format!("ক্ষ্ম্র{boundary}ইয়")
            );
        }
        assert_eq!(transliterate(raw), "ক্ষ্ম");
        assert_eq!(transliterate("a"), "আ");
        assert_eq!(transliterate("r"), "র");
        assert_eq!(transliterate("y"), "ইয়");
        assert_eq!(transliterate("m"), "ম");
    }
}

#[test]
fn n_conjunct_context_resets_at_boundaries_and_between_calls() {
    for raw in ["kkhN", "kShN", "kxN"] {
        for boundary in ['\0', '\n', '\r', '\t', '🙂', 'ক', '\u{301}'] {
            let shorter = &raw[..raw.len() - 1];
            assert_eq!(
                transliterate(&format!("{shorter}{boundary}N")),
                format!("ক্ষ{boundary}ণ")
            );
            assert_eq!(
                transliterate(&format!("{raw}{boundary}a")),
                format!("ক্ষ্ণ{boundary}আ")
            );
            assert_eq!(
                transliterate(&format!("{raw}r{boundary}y")),
                format!("ক্ষ্ণ্র{boundary}ইয়")
            );
        }
        assert_eq!(transliterate(raw), "ক্ষ্ণ");
        assert_eq!(transliterate("a"), "আ");
        assert_eq!(transliterate("r"), "র");
        assert_eq!(transliterate("y"), "ইয়");
        assert_eq!(transliterate("N"), "ণ");
    }
}

fn check_fixture(fixture: &str) {
    let mut count = 0;
    for line in fixture.lines() {
        if line.starts_with('#') {
            continue;
        }
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 3, "invalid fixture: {line:?}");
        let [input, expected, points] = [columns[0], columns[1], columns[2]];
        assert_eq!(transliterate(input), expected, "input: {input:?}");
        let actual_points = expected
            .chars()
            .map(|ch| format!("U+{:04X}", ch as u32))
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(actual_points, points, "fixture code points: {input:?}");
        count += 1;
    }
    assert!(count > 0, "fixture must contain cases");
}

#[test]
fn empty_input_and_unmatched_unicode_are_preserved() {
    for input in ["", "QX", "বাংলা", "é🙂\u{301}", "\t\n !`"] {
        assert_eq!(transliterate(input), input);
    }
    assert_eq!(transliterate("qXZ"), "কX্য");
    assert_eq!(transliterate("qXYZ"), "কXয়্য");
    assert_eq!(transliterate("🙂khবাংলা2é"), "🙂খবাংলা২é");
    // Only ASCII digits are converted; other scripts' digits are preserved.
    assert_eq!(transliterate("০১২৩৪৫৬৭৮৯ ١٢ １２"), "০১২৩৪৫৬৭৮৯ ١٢ １２");
}

#[test]
fn unsupported_rules_still_preserve_unmatched_characters() {
    for (input, expected) in [
        ("E", "E"),
        ("X", "X"),
        ("Xa", "Xআ"),
        ("kX", "কX"),
        ("xA", "এক্সA"),
        ("NgX", "ঙX"),
        ("NgxA", "ঙ্ষA"),
        ("gNgx", "গঙ্ষ"),
        ("dgr", "দগ্র"),
        ("Rghr", "ড়ঘ্র"),
        ("ggr", "গগ্র"),
        ("gNgr", "গঙর"),
        ("gNgg", "গঙ্গ"),
        ("gNggh", "গঙ্ঘ"),
        ("NNgg", "ণঙ্গ"),
        ("hNgg", "হঙ্গ"),
        ("ShNgg", "ষ্ণগগ"),
        ("ShNggr", "ষ্ণগগ্র"),
        ("kkhNggr", "ক্ষ্ণগগ্র"),
        ("kShNgghr", "ক্ষ্ণগঘ্র"),
        ("kxNgghr", "ক্ষ্ণগঘ্র"),
        ("NNgx", "ণঙ্ষ"),
        ("hNgx", "হঙ্ষ"),
        ("rrNgx", "ররঙ্ষ"),
        ("Ngx`a", "ঙ্ষ`আ"),
        ("Ngx`h", "ঙ্ষ`হ"),
        ("rrx", "ররক্স"),
        ("rrkx", "ররক্ষ"),
        ("x`a", "এক্স`আ"),
        ("x`h", "এক্স`হ"),
        ("W", "W"),
        ("Wa", "Wআ"),
        ("wA", "ওA"),
        ("wE", "ওE"),
        ("rrw", "রর্ব"),
        ("w`a", "ও`আ"),
        ("w`h", "ও`হ"),
        ("Q", "Q"),
        ("Qa", "Qআ"),
        ("V", "V"),
        ("Va", "Vআ"),
        ("vr", "ভর"),
        ("vH", "ভH"),
        ("v`a", "ভ`আ"),
        ("v`h", "ভ`হ"),
        ("qr", "কর"),
        ("q`a", "ক`আ"),
        ("q`h", "ক`হ"),
        ("hr", "হর"),
        ("rrk", "ররক"),
        ("rrr", "ররর"),
        ("Rri", "ড়রি"),
        ("Rr", "ড়র"),
        ("Rhr", "ঢ়র"),
        ("RH", "ড়H"),
        ("R`h", "ড়`হ"),
        ("AZ", "A্য"),
        ("Z`a", "্য`আ"),
        ("Z`h", "্য`হ"),
        ("zr", "যর"),
        ("z`h", "য`হ"),
        ("ngh", "ংহ"),
        ("nng", "নং"),
        ("nG", "নG"),
        ("ng`a", "ং`আ"),
        ("NgK", "ঙK"),
        ("NgkH", "ঙ্কH"),
        ("NgkA", "ঙ্কA"),
        ("gNgk", "গঙ্ক"),
        ("NNgk", "ণঙ্ক"),
        ("hNgk", "হঙ্ক"),
        ("rrNgk", "ররঙ্ক"),
        ("Ngk`a", "ঙ্ক`আ"),
        ("Ngk`h", "ঙ্ক`হ"),
        ("gNgkh", "গঙ্খ"),
        ("NNgkh", "ণঙ্খ"),
        ("hNgkh", "হঙ্খ"),
        ("rrNgkh", "ররঙ্খ"),
        ("NgKh", "ঙKহ"),
        ("NgkhA", "ঙ্খA"),
        ("Ngkh`a", "ঙ্খ`আ"),
        ("Ngkh`h", "ঙ্খ`হ"),
        ("gNgkx", "গঙ্ক্ষ"),
        ("NNgkx", "ণঙ্ক্ষ"),
        ("hNgkx", "হঙ্ক্ষ"),
        ("rrNgkx", "ররঙ্ক্ষ"),
        ("NgkX", "ঙ্কX"),
        ("NgKx", "ঙKএক্স"),
        ("NgkxA", "ঙ্ক্ষA"),
        ("Ngkx`a", "ঙ্ক্ষ`আ"),
        ("Ngkx`h", "ঙ্ক্ষ`হ"),
        ("gNgkkh", "গঙ্ক্ষ"),
        ("NNgkkh", "ণঙ্ক্ষ"),
        ("hNgkkh", "হঙ্ক্ষ"),
        ("rrNgkkh", "ররঙ্ক্ষ"),
        ("NgKkh", "ঙKখ"),
        ("NgkkH", "ঙ্ককH"),
        ("NgkkhA", "ঙ্ক্ষA"),
        ("Ngkkh`a", "ঙ্ক্ষ`আ"),
        ("Ngkkh`h", "ঙ্ক্ষ`হ"),
        ("gNgkSh", "গঙ্ক্ষ"),
        ("NNgkSh", "ণঙ্ক্ষ"),
        ("hNgkSh", "হঙ্ক্ষ"),
        ("rrNgkSh", "ররঙ্ক্ষ"),
        ("NgKSh", "ঙKষ"),
        ("NgkSH", "ঙ্কশH"),
        ("NgkShA", "ঙ্ক্ষA"),
        ("NgkSh`a", "ঙ্ক্ষ`আ"),
        ("NgkSh`h", "ঙ্ক্ষ`হ"),
        ("nkSh", "নক্ষ"),
        ("lkSh", "লক্ষ"),
        ("skSh", "সক্ষ"),
        ("ShkSh", "ষক্ষ"),
        ("kShNG", "ক্ষ্ণG"),
        ("rrkShN", "ররক্ষ্ণ"),
        ("kSHN", "কশHণ"),
        ("kShN`a", "ক্ষ্ণ`আ"),
        ("kSH", "কশH"),
        ("KSh", "Kষ"),
        ("kShA", "ক্ষA"),
        ("rrkSh", "ররক্ষ"),
        ("kSh`a", "ক্ষ`আ"),
        ("kSh`h", "ক্ষ`হ"),
        ("k`Sh", "ক`ষ"),
        ("kSh.", "ক্ষ."),
        ("NGc", "ঞচ"),
        ("NGch", "ঞছ"),
        ("N`g", "ণ`গ"),
        ("y`a", "ইয়`আ"),
        ("y`h", "ইয়`হ"),
        ("rrY", "ররয়"),
        ("Y`a", "য়`আ"),
        ("Y`h", "য়`হ"),
        ("pr", "পর"),
        ("pp", "পপ"),
        ("pph", "পফ"),
        ("P", "P"),
        ("Ph", "Pহ"),
        ("F", "F"),
        ("p`h", "প`হ"),
        ("cc", "চচ"),
        ("cch", "চছ"),
        ("jjh", "জঝ"),
        ("cr", "চর"),
        ("C", "C"),
        ("J", "J"),
        ("Jh", "Jহ"),
        ("c`h", "চ`হ"),
        ("ddh", "দধ"),
        ("DDh", "ডঢ"),
        ("dr", "দর"),
        ("dH", "দH"),
        ("DH", "ডH"),
        ("d`h", "দ`হ"),
        ("ll", "লল"),
        ("hl", "হল"),
        ("L", "L"),
        ("H", "H"),
        ("l`h", "ল`হ"),
        ("h`", "হ`"),
        ("sk", "সক"),
        ("st", "সত"),
        ("ShT", "ষট"),
        ("sH", "সH"),
        ("SH", "শH"),
        ("s`h", "স`হ"),
        ("nkkh", "নক্ষ"),
        ("lkkh", "লক্ষ"),
        ("skkh", "সক্ষ"),
        ("Shkkh", "ষক্ষ"),
        ("kkhNG", "ক্ষ্ণG"),
        ("kxNG", "ক্ষ্ণG"),
        ("kkH", "ক্কH"),
        ("kkA", "ক্কA"),
        ("kkE", "ক্কE"),
        ("kkX", "ক্কX"),
        ("kK", "কK"),
        ("Kk", "Kক"),
        ("KK", "KK"),
        ("ShNG", "ষ্ণG"),
        ("Shmmm", "ষ্মমম"),
        ("ShNmng", "ষ্ণমং"),
        ("ShM", "ষM"),
        ("SHN", "শHণ"),
        ("SHm", "শHম"),
        ("rrShN", "ররষ্ণ"),
        ("rrShm", "ররষ্ম"),
        ("Shm`a", "ষ্ম`আ"),
        ("ShN`a", "ষ্ণ`আ"),
        ("ShNmr", "ষ্ণমর"),
        ("Shmmr", "ষ্মমর"),
        ("shm", "শম"),
        ("shn", "শন"),
        ("Sm", "শম"),
        ("nkk", "নক্ক"),
        ("lkk", "লক্ক"),
        ("skk", "সক্ক"),
        ("Shkk", "ষক্ক"),
        ("rrkk", "ররক্ক"),
        ("kk`a", "ক্ক`আ"),
        ("Kkh", "Kখ"),
        ("kKh", "কKহ"),
        ("kkhA", "ক্ষA"),
        ("rrkkh", "ররক্ষ"),
        ("kkh`a", "ক্ষ`আ"),
        ("kkh`h", "ক্ষ`হ"),
        ("k`kh", "ক`খ"),
        ("kkh.", "ক্ষ."),
        ("K", "K"),
        ("k`h", "ক`হ"),
        ("k`a", "ক`আ"),
        ("i`", "ই`"),
        ("I`", "ঈ`"),
        ("u`", "উ`"),
        ("U`", "ঊ`"),
        ("e`", "এ`"),
        ("ee`", "ঈ`"),
        ("o`", "অ`"),
        ("O`", "ও`"),
        ("oo`", "উ`"),
        ("OI`", "ঐ`"),
        ("OU`", "ঔ`"),
        ("rri`", "ঋ`"),
        ("r`", "র`"),
        ("rr`", "রর`"),
    ] {
        assert_eq!(transliterate(input), expected, "input: {input:?}");
    }
    // Earlier n/l/s/Sh + k rules and standalone uppercase G remain gaps.
    for raw in ["kkhN", "kShN", "kxN"] {
        for (prefix, letter) in [("n", "ন"), ("l", "ল"), ("s", "স"), ("Sh", "ষ")] {
            assert_eq!(
                transliterate(&format!("{prefix}{raw}")),
                format!("{letter}ক্ষ্ণ")
            );
        }
        assert_eq!(transliterate(&format!("{raw}G")), "ক্ষ্ণG");
    }
    // The new forms do not implement remaining earlier prefixes, basic m conjuncts,
    // uppercase aliases, reph or escapes.
    for raw in ["kkhm", "kShm", "kxm"] {
        for (prefix, letter) in [("n", "ন"), ("l", "ল"), ("s", "স"), ("Sh", "ষ")] {
            assert_eq!(
                transliterate(&format!("{prefix}{raw}")),
                format!("{letter}ক্ষ্ম")
            );
        }
        assert_eq!(transliterate(&format!("Ng{raw}r")), "ঙ্ক্ষমর");
        assert_eq!(transliterate(&format!("{raw}mm")), "ক্ষ্মমম");
        assert_eq!(transliterate(&format!("{raw}mr")), "ক্ষ্মমর");
        assert_eq!(transliterate(&format!("rr{raw}")), "ররক্ষ্ম");
        assert_eq!(transliterate(&format!("{}M", &raw[..raw.len() - 1])), "ক্ষM");
        assert_eq!(transliterate(&format!("{raw}`a")), "ক্ষ্ম`আ");
    }
    assert_eq!(transliterate("mr mm mn mng"), "মর মম মন মং");
}

#[test]
fn long_mixed_input_preserves_character_boundaries() {
    let input = "🙂kh2a\u{301}kUuakIakeeeakoooakOakOIakOUokrria".repeat(4096)
        + &"rrrirrrri".repeat(4096)
        + &"phifipO".repeat(4096);
    let input =
        input + &"cichijijhi".repeat(4096) + &"didhIDuDhO".repeat(4096) + &"lihU".repeat(4096);
    let input = input + &"sishI SuShO".repeat(4096) + &"RiRhORrri".repeat(4096);
    let input = input + &"zizU".repeat(4096) + &"NgiNGU".repeat(4096);
    let input = input + &"ngang1ngoo".repeat(4096);
    let input = input + &"Ngrri🙂NGrrri1ngrrrri".repeat(4096);
    let input = input + &"YiaYrriY1a".repeat(4096);
    let input = input + &"🙂y🙂ay🙂kya1ryyNgry".repeat(4096);
    let input = input + &"🙂Zi1rZyNgrZa1aZ🙂oZ".repeat(4096);
    let input = input + &"🙂yrr1yrri2yrry3Zry4aZrr".repeat(4096);
    let input = input + &"🙂Yr1Yrri2Yry3Yrry4Yrrrri".repeat(4096);
    let input = input + &"🙂qa1qia2qU3qOI4qrri5qy".repeat(4096);
    let input = input + &"🙂va1via2vU3vOI4vrri5vy".repeat(4096);
    let input = input + &"🙂wa1awa2kwa3wwi4wrri5wy".repeat(4096);
    let input = input + &"🙂wrr1wrri2wrrri3wrry4kwrr5awrr".repeat(4096);
    let input = input + &"🙂xa1axa2kxi3xxU4xrri5xy".repeat(4096);
    let input = input + &"🙂xrr1xrri2xrrri3xrry4kxrr5axrr".repeat(4096);
    let input = input + &"🙂Ngxa1Ngxrri2Ngxrr3Ngxrry4NgxNgx".repeat(4096);
    let input = input + &"🙂Ngka1Ngkr2Ngkrri3Ngkry4Ngkrry5NgkNgk".repeat(4096);
    let input = input + &"🙂Ngkha1Ngkhr2Ngkhrri3Ngkhry4Ngkhrry5NgkhNgkh".repeat(4096);
    let input = input + &"🙂Ngkxa1Ngkxr2Ngkxrri3Ngkxry4Ngkxrr5NgkxNgkx".repeat(4096);
    let input = input + &"🙂Ngkkha1Ngkkhr2Ngkkhrri3Ngkkhry4Ngkkhrry5NgkkhNgkkh".repeat(4096);
    let input = input + &"🙂NgkSha1NgkShr2NgkShrri3NgkShry4NgkShrry5NgkShNgkkh".repeat(4096);
    let input = input + &"🙂kkha1kkhr2kkhrri3kkhry4kkhrry5kkhkkh".repeat(4096);
    let input = input + &"🙂kSha1kShr2kShrri3kShry4kShrry5kShkkh".repeat(4096);
    let input = input + &"🙂kkhNa1kShNr2kxNrri3kxNry4kkhNrry5kShNgkSh".repeat(4096);
    let input = input + &"🙂kkhma1kShmr2kxmrri3kxmry4kkhmrry5kShmng6Ngkxm".repeat(4096);
    let input = input + &"🙂kka1kkr2kkrri3kkry4kkrry5kkkh6kkx7kkhkk".repeat(4096);
    let input = input + &"🙂ShNa1Shmr2ShNrri3Shmry4ShNrry5ShNgx6kkShm7Shmng".repeat(4096);
    let input = input + &"🙂kr1khr2Shr3kry4khrry5ShrrZ6kkkhr7kkShr8Ngkkr".repeat(4096);
    let input = input + &"🙂gr1ghr2gry3ghrry4grri5ShNgr6Shmgr7kShmgr".repeat(4096);
    let expected = "🙂খ২আ\u{301}কূউয়াকীয়াকীএয়াকুঅয়াকোয়াকৈয়াকৌওকৃয়া".repeat(4096)
        + &"রররিররৃ".repeat(4096)
        + &"ফিফিপো".repeat(4096);
    let expected =
        expected + &"চিছিজিঝি".repeat(4096) + &"দিধীডুঢো".repeat(4096) + &"লিহূ".repeat(4096);
    let expected = expected + &"সিশী শুষো".repeat(4096) + &"ড়িঢ়োড়ৃ".repeat(4096);
    let expected = expected + &"যিযূ".repeat(4096) + &"ঙিঞূ".repeat(4096);
    let expected = expected + &"ঙ্গাং১ঙ্গঅ".repeat(4096);
    let expected = expected + &"ঙররি🙂ঞরৃ১ংররররি".repeat(4096);
    let expected = expected + &"য়িয়ায়ৃয়১আ".repeat(4096);
    let expected = expected + &"🙂ইয়🙂আয়🙂ক্যা১র\u{200D}্য্যঙর্য".repeat(4096);
    let expected = expected + &"🙂্যি১র\u{200D}্য্যঙর্যা১অ্যা🙂অ্য".repeat(4096);
    let expected = expected + &"🙂ইয়্রর১ইয়ৃ২ইয়রর\u{200D}্য৩্য্র্য৪অ্যা্রর".repeat(4096);
    let expected = expected + &"🙂য়্র১য়ৃ২য়্র্য৩য়রর\u{200D}্য৪য়্ররৃ".repeat(4096);
    let expected = expected + &"🙂কা১কিয়া২কূ৩কৈ৪কৃ৫ক্য".repeat(4096);
    let expected = expected + &"🙂ভা১ভিয়া২ভূ৩ভৈ৪ভৃ৫ভ্য".repeat(4096);
    let expected = expected + &"🙂ওয়া১আওা২ক্বা৩ও্বি৪ওৃ৫ও্য".repeat(4096);
    let expected = expected + &"🙂ও্রর১ওৃ২ও্রররি৩ওরর\u{200D}্য৪ক্ব্রর৫আও্রর".repeat(4096);
    let expected = expected + &"🙂এক্সা১আক্সা২ক্ষি৩এক্সক্সূ৪এক্সৃ৫এক্স্য".repeat(4096);
    let expected = expected + &"🙂এক্স্রর১এক্সৃ২এক্স্রররি৩এক্সরর\u{200D}্য৪ক্ষ্রর৫আক্স্রর".repeat(4096);
    let expected = expected + &"🙂ঙ্ষা১ঙ্ষৃ২ঙ্ষ্রর৩ঙ্ষরর\u{200D}্য৪ঙ্ষঙ্ষ".repeat(4096);
    let expected = expected + &"🙂ঙ্কা১ঙ্ক্র২ঙ্কৃ৩ঙ্ক্র্য৪ঙ্করর\u{200D}্য৫ঙ্কঙ্ক".repeat(4096);
    let expected = expected + &"🙂ঙ্খা১ঙ্খ্র২ঙ্খৃ৩ঙ্খ্র্য৪ঙ্খরর\u{200D}্য৫ঙ্খঙ্খ".repeat(4096);
    let expected = expected + &"🙂ঙ্ক্ষা১ঙ্ক্ষর২ঙ্ক্ষৃ৩ঙ্ক্ষর\u{200D}্য৪ঙ্ক্ষ্রর৫ঙ্ক্ষঙ্ক্ষ".repeat(4096);
    let expected = expected + &"🙂ঙ্ক্ষা১ঙ্ক্ষ্র২ঙ্ক্ষৃ৩ঙ্ক্ষ্র্য৪ঙ্ক্ষরর\u{200D}্য৫ঙ্ক্ষঙ্ক্ষ".repeat(4096);
    let expected = expected + &"🙂ঙ্ক্ষা১ঙ্ক্ষ্র২ঙ্ক্ষৃ৩ঙ্ক্ষ্র্য৪ঙ্ক্ষরর\u{200D}্য৫ঙ্ক্ষঙ্ক্ষ".repeat(4096);
    let expected = expected + &"🙂ক্ষা১ক্ষ্র২ক্ষৃ৩ক্ষ্র্য৪ক্ষরর\u{200D}্য৫ক্ষক্ষ".repeat(4096);
    let expected = expected + &"🙂ক্ষা১ক্ষ্র২ক্ষৃ৩ক্ষ্র্য৪ক্ষরর\u{200D}্য৫ক্ষক্ষ".repeat(4096);
    let expected = expected + &"🙂ক্ষ্ণা১ক্ষ্ণ্র২ক্ষ্ণৃ৩ক্ষ্ণ্র্য৪ক্ষ্ণরর\u{200D}্য৫ক্ষ্ণগক্ষ".repeat(4096);
    let expected = expected + &"🙂ক্ষ্মা১ক্ষ্ম্র২ক্ষ্মৃ৩ক্ষ্ম্র্য৪ক্ষ্মরর\u{200D}্য৫ক্ষ্মং৬ঙ্ক্ষম".repeat(4096);
    let expected = expected + &"🙂ক্কা১ক্ক্র২ক্কৃ৩ক্ক্র্য৪ক্করর\u{200D}্য৫ক্কখ৬ক্কক্স৭ক্ষক্ক".repeat(4096);
    let expected = expected + &"🙂ষ্ণা১ষ্ম্র২ষ্ণৃ৩ষ্ম্র্য৪ষ্ণরর\u{200D}্য৫ষ্ণগক্স৬ক্কষ্ম৭ষ্মং".repeat(4096);
    let expected =
        expected + &"🙂ক্র১খ্র২ষ্র৩ক্র্য৪খরর\u{200D}্য৫ষরর\u{200D}্য৬ক্কখ্র৭ক্কষ্র৮ঙ্কক্র".repeat(4096);
    let expected = expected + &"🙂গ্র১ঘ্র২গ্র্য৩ঘরর\u{200D}্য৪গৃ৫ষ্ণগ্র৬ষ্মগ্র৭ক্ষ্মগ্র".repeat(4096);
    assert_eq!(transliterate(&input), expected);
}

#[test]
fn vowel_context_resets_at_unmatched_characters_and_between_calls() {
    for boundary in [' ', ',', '-', '\n', '\r', '\t', '🙂', 'ক', '\u{301}'] {
        assert_eq!(
            transliterate(&format!("Y{boundary}r")),
            format!("য়{boundary}র")
        );
        assert_eq!(
            transliterate(&format!("Y{boundary}ry")),
            format!("য়{boundary}র\u{200D}্য")
        );
        assert_eq!(
            transliterate(&format!("r{boundary}Z")),
            format!("র{boundary}্য")
        );
        assert_eq!(
            transliterate(&format!("k{boundary}wa")),
            format!("ক{boundary}ওয়া")
        );
        assert_eq!(
            transliterate(&format!("w{boundary}rr")),
            format!("ও{boundary}রর")
        );
        assert_eq!(
            transliterate(&format!("wrr{boundary}ry")),
            format!("ও্রর{boundary}র\u{200D}্য")
        );
        assert_eq!(
            transliterate(&format!("k{boundary}xa")),
            format!("ক{boundary}এক্সা")
        );
        for (raw, letter) in [
            ("x", "এক্স"),
            ("kx", "ক্ষ"),
            ("Ngx", "ঙ্ষ"),
            ("Ngk", "ঙ্ক"),
            ("Ngkh", "ঙ্খ"),
            ("Ngkx", "ঙ্ক্ষ"),
            ("Ngkkh", "ঙ্ক্ষ"),
            ("NgkSh", "ঙ্ক্ষ"),
            ("kkh", "ক্ষ"),
            ("kSh", "ক্ষ"),
        ] {
            assert_eq!(
                transliterate(&format!("{raw}{boundary}rr")),
                format!("{letter}{boundary}রর")
            );
            assert_eq!(
                transliterate(&format!("{raw}rr{boundary}ry")),
                format!("{letter}্রর{boundary}র\u{200D}্য")
            );
        }
        assert_eq!(
            transliterate(&format!("Ng{boundary}x")),
            format!("ঙ{boundary}এক্স")
        );
        assert_eq!(
            transliterate(&format!("Ng{boundary}k")),
            format!("ঙ{boundary}ক")
        );
        assert_eq!(
            transliterate(&format!("Ngkr{boundary}y")),
            format!("ঙ্ক্র{boundary}ইয়")
        );
        assert_eq!(
            transliterate(&format!("Ngk{boundary}h")),
            format!("ঙ্ক{boundary}হ")
        );
        assert_eq!(
            transliterate(&format!("Ngkh{boundary}r")),
            format!("ঙ্খ{boundary}র")
        );
        assert_eq!(
            transliterate(&format!("Ngkhr{boundary}y")),
            format!("ঙ্খ্র{boundary}ইয়")
        );
        assert_eq!(
            transliterate(&format!("Ngk{boundary}x")),
            format!("ঙ্ক{boundary}এক্স")
        );
        assert_eq!(
            transliterate(&format!("Ngkx{boundary}ry")),
            format!("ঙ্ক্ষ{boundary}র\u{200D}্য")
        );
        assert_eq!(
            transliterate(&format!("Ngkxr{boundary}y")),
            format!("ঙ্ক্ষর{boundary}ইয়")
        );
        assert_eq!(
            transliterate(&format!("Ngk{boundary}kh")),
            format!("ঙ্ক{boundary}খ")
        );
        assert_eq!(
            transliterate(&format!("Ngkkh{boundary}r")),
            format!("ঙ্ক্ষ{boundary}র")
        );
        assert_eq!(
            transliterate(&format!("Ngkkhr{boundary}y")),
            format!("ঙ্ক্ষ্র{boundary}ইয়")
        );
        assert_eq!(
            transliterate(&format!("Ngk{boundary}Sh")),
            format!("ঙ্ক{boundary}ষ")
        );
        assert_eq!(
            transliterate(&format!("NgkSh{boundary}r")),
            format!("ঙ্ক্ষ{boundary}র")
        );
        assert_eq!(
            transliterate(&format!("NgkShr{boundary}y")),
            format!("ঙ্ক্ষ্র{boundary}ইয়")
        );
        assert_eq!(
            transliterate(&format!("k{boundary}kh")),
            format!("ক{boundary}খ")
        );
        assert_eq!(
            transliterate(&format!("kkhr{boundary}y")),
            format!("ক্ষ্র{boundary}ইয়")
        );
        assert_eq!(
            transliterate(&format!("k{boundary}Sh")),
            format!("ক{boundary}ষ")
        );
        assert_eq!(
            transliterate(&format!("kShr{boundary}y")),
            format!("ক্ষ্র{boundary}ইয়")
        );
        for (raw, expected) in [("r", "র"), ("ay", "আয়"), ("ky", "ক্য")] {
            assert_eq!(
                transliterate(&format!("{raw}{boundary}y")),
                format!("{expected}{boundary}ইয়")
            );
        }
        for (vowel, independent) in [
            ("a", "আ"),
            ("i", "ই"),
            ("I", "ঈ"),
            ("u", "উ"),
            ("U", "ঊ"),
            ("e", "এ"),
            ("ee", "ঈ"),
            ("o", "অ"),
            ("O", "ও"),
            ("oo", "উ"),
            ("OI", "ঐ"),
            ("OU", "ঔ"),
            ("rri", "ঋ"),
        ] {
            for (raw, letter) in [
                ("k", "ক"),
                ("q", "ক"),
                ("v", "ভ"),
                ("x", "এক্স"),
                ("kx", "ক্ষ"),
                ("xrr", "এক্স্রর"),
                ("kxrr", "ক্ষ্রর"),
                ("Ngx", "ঙ্ষ"),
                ("Ngxrr", "ঙ্ষ্রর"),
                ("Ngk", "ঙ্ক"),
                ("Ngkr", "ঙ্ক্র"),
                ("Ngkrr", "ঙ্ক্রর"),
                ("Ngkh", "ঙ্খ"),
                ("Ngkhr", "ঙ্খ্র"),
                ("Ngkhrr", "ঙ্খ্রর"),
                ("Ngkx", "ঙ্ক্ষ"),
                ("Ngkxr", "ঙ্ক্ষর"),
                ("Ngkxrr", "ঙ্ক্ষ্রর"),
                ("Ngkkh", "ঙ্ক্ষ"),
                ("Ngkkhr", "ঙ্ক্ষ্র"),
                ("Ngkkhrr", "ঙ্ক্ষ্রর"),
                ("NgkSh", "ঙ্ক্ষ"),
                ("NgkShr", "ঙ্ক্ষ্র"),
                ("NgkShrr", "ঙ্ক্ষ্রর"),
                ("kkh", "ক্ষ"),
                ("kkhr", "ক্ষ্র"),
                ("kkhrr", "ক্ষ্রর"),
                ("kSh", "ক্ষ"),
                ("kShr", "ক্ষ্র"),
                ("kShrr", "ক্ষ্রর"),
                ("w", "ও"),
                ("kw", "ক্ব"),
                ("wrr", "ও্রর"),
                ("kwrr", "ক্ব্রর"),
                ("l", "ল"),
                ("h", "হ"),
                ("s", "স"),
                ("sh", "শ"),
                ("S", "শ"),
                ("Sh", "ষ"),
                ("R", "ড়"),
                ("Rh", "ঢ়"),
                ("z", "য"),
                ("Ng", "ঙ"),
                ("NG", "ঞ"),
                ("ng", "ং"),
                ("Y", "য়"),
                ("y", "ইয়"),
                ("Z", "্য"),
                ("yr", "ইয়র"),
                ("yrr", "ইয়্রর"),
                ("Zry", "্য্র্য"),
                ("Yr", "য়্র"),
                ("Yrr", "য়্রর"),
                ("Yry", "য়্র্য"),
            ] {
                let input = format!("{raw}{boundary}{vowel} {vowel}{boundary}a");
                let expected = format!("{letter}{boundary}{independent} {independent}{boundary}আ");
                assert_eq!(transliterate(&input), expected, "input: {input:?}");
            }
        }
    }
    assert_eq!(transliterate("k"), "ক");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("q"), "ক");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("v"), "ভ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("w"), "ও");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("k"), "ক");
    assert_eq!(transliterate("wa"), "ওয়া");
    assert_eq!(transliterate("i"), "ই");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("I"), "ঈ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("u"), "উ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("U"), "ঊ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("e"), "এ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("ee"), "ঈ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("o"), "অ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("O"), "ও");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("oo"), "উ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("OI"), "ঐ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("OU"), "ঔ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("rri"), "ঋ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("r"), "র");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("rr"), "রর");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("p"), "প");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("ph"), "ফ");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("f"), "ফ");
    assert_eq!(transliterate("a"), "আ");
    for (raw, expected) in [("c", "চ"), ("ch", "ছ"), ("j", "জ"), ("jh", "ঝ")] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("a"), "আ");
    }
    for (raw, expected) in [("d", "দ"), ("dh", "ধ"), ("D", "ড"), ("Dh", "ঢ")] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("a"), "আ");
    }
    for (raw, expected) in [("l", "ল"), ("h", "হ")] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("a"), "আ");
    }
    for (raw, expected) in [("s", "স"), ("sh", "শ"), ("S", "শ"), ("Sh", "ষ")] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("a"), "আ");
    }
    for (raw, expected) in [("R", "ড়"), ("Rh", "ঢ়")] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("a"), "আ");
    }
    assert_eq!(transliterate("z"), "য");
    assert_eq!(transliterate("a"), "আ");
    for (raw, expected) in [("Ng", "ঙ"), ("NG", "ঞ")] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("a"), "আ");
        assert_eq!(transliterate("rri"), "ঋ");
    }
    assert_eq!(transliterate("ng"), "ং");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("rri"), "ঋ");
    assert_eq!(transliterate("Y"), "য়");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("r"), "র");
    assert_eq!(transliterate("y"), "ইয়");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("y"), "ইয়");
    assert_eq!(transliterate("Z"), "্য");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("y"), "ইয়");
    for (raw, expected) in [
        ("yrr", "ইয়্রর"),
        ("Zry", "্য্র্য"),
        ("aZrr", "অ্যা্রর"),
        ("Yr", "য়্র"),
        ("Yrr", "য়্রর"),
        ("Yry", "য়্র্য"),
        ("wrr", "ও্রর"),
        ("kwrr", "ক্ব্রর"),
        ("wrrwrr", "ও্রর্ব্রর"),
        ("x", "এক্স"),
        ("kx", "ক্ষ"),
        ("xrr", "এক্স্রর"),
        ("kxrr", "ক্ষ্রর"),
        ("xrrxrr", "এক্স্ররক্স্রর"),
        ("Ngx", "ঙ্ষ"),
        ("Ngxrr", "ঙ্ষ্রর"),
        ("NgxNgx", "ঙ্ষঙ্ষ"),
        ("Ngk", "ঙ্ক"),
        ("Ngkr", "ঙ্ক্র"),
        ("Ngkrr", "ঙ্ক্রর"),
        ("Ngkry", "ঙ্ক্র্য"),
        ("NgkNgk", "ঙ্কঙ্ক"),
        ("Ngkh", "ঙ্খ"),
        ("Ngkhr", "ঙ্খ্র"),
        ("Ngkhrr", "ঙ্খ্রর"),
        ("Ngkhry", "ঙ্খ্র্য"),
        ("NgkhNgkh", "ঙ্খঙ্খ"),
        ("Ngkx", "ঙ্ক্ষ"),
        ("Ngkxr", "ঙ্ক্ষর"),
        ("Ngkxrr", "ঙ্ক্ষ্রর"),
        ("Ngkxry", "ঙ্ক্ষর\u{200D}্য"),
        ("NgkxNgkx", "ঙ্ক্ষঙ্ক্ষ"),
        ("Ngkkh", "ঙ্ক্ষ"),
        ("Ngkkhr", "ঙ্ক্ষ্র"),
        ("Ngkkhrr", "ঙ্ক্ষ্রর"),
        ("Ngkkhry", "ঙ্ক্ষ্র্য"),
        ("NgkkhNgkkh", "ঙ্ক্ষঙ্ক্ষ"),
        ("NgkSh", "ঙ্ক্ষ"),
        ("NgkShr", "ঙ্ক্ষ্র"),
        ("NgkShrr", "ঙ্ক্ষ্রর"),
        ("NgkShry", "ঙ্ক্ষ্র্য"),
        ("NgkShNgkkh", "ঙ্ক্ষঙ্ক্ষ"),
        ("kkh", "ক্ষ"),
        ("kkhr", "ক্ষ্র"),
        ("kkhrr", "ক্ষ্রর"),
        ("kkhry", "ক্ষ্র্য"),
        ("kkhkkh", "ক্ষক্ষ"),
        ("kSh", "ক্ষ"),
        ("kShr", "ক্ষ্র"),
        ("kShrr", "ক্ষ্রর"),
        ("kShry", "ক্ষ্র্য"),
        ("kShkkh", "ক্ষক্ষ"),
    ] {
        assert_eq!(transliterate(raw), expected);
        assert_eq!(transliterate("r"), "র");
        assert_eq!(transliterate("rr"), "রর");
        assert_eq!(transliterate("y"), "ইয়");
        assert_eq!(transliterate("a"), "আ");
    }
}
