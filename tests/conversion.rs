use jong_bangla_core::transliterate;

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
    for input in ["", "qXZ", "বাংলা", "é🙂\u{301}", "\t\n !`"] {
        assert_eq!(transliterate(input), input);
    }
    assert_eq!(transliterate("qXYZ"), "qXয়Z");
    assert_eq!(transliterate("🙂khবাংলা2é"), "🙂খবাংলা২é");
    // Only ASCII digits are converted; other scripts' digits are preserved.
    assert_eq!(transliterate("০১২৩৪৫৬৭৮৯ ١٢ １２"), "০১২৩৪৫৬৭৮৯ ١٢ １２");
}

#[test]
fn unsupported_rules_still_preserve_unmatched_characters() {
    for (input, expected) in [
        ("E", "E"),
        ("qo", "qঅ"),
        ("kr", "কর"),
        ("rrk", "ররক"),
        ("rrr", "ররর"),
        ("Rri", "ড়রি"),
        ("Rr", "ড়র"),
        ("Rhr", "ঢ়র"),
        ("RH", "ড়H"),
        ("R`h", "ড়`হ"),
        ("Z", "Z"),
        ("kZ", "কZ"),
        ("zZ", "যZ"),
        ("zr", "যর"),
        ("z`h", "য`হ"),
        ("ngh", "ংহ"),
        ("nng", "নং"),
        ("nG", "নG"),
        ("ng`a", "ং`আ"),
        ("Ngk", "ঙক"),
        ("NGc", "ঞচ"),
        ("NGch", "ঞছ"),
        ("N`g", "ণ`গ"),
        ("yrr", "ইয়রর"),
        ("yrrri", "ইয়রররি"),
        ("yZ", "ইয়Z"),
        ("y`a", "ইয়`আ"),
        ("y`h", "ইয়`হ"),
        ("Yr", "য়র"),
        ("rrY", "ররয়"),
        ("YZ", "য়Z"),
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
        ("kkh", "কখ"),
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
    assert_eq!(transliterate(&input), expected);
}

#[test]
fn vowel_context_resets_at_unmatched_characters_and_between_calls() {
    for boundary in [' ', ',', '-', '\n', '\r', '\t', '🙂', 'ক', '\u{301}'] {
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
            ] {
                let input = format!("{raw}{boundary}{vowel} {vowel}{boundary}a");
                let expected = format!("{letter}{boundary}{independent} {independent}{boundary}আ");
                assert_eq!(transliterate(&input), expected, "input: {input:?}");
            }
        }
    }
    assert_eq!(transliterate("k"), "ক");
    assert_eq!(transliterate("a"), "আ");
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
}
