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
    for input in ["", "qXYZ", "বাংলা", "é🙂\u{301}", "\t\n !`"] {
        assert_eq!(transliterate(input), input);
    }
    assert_eq!(transliterate("🙂khবাংলা2é"), "🙂খবাংলা২é");
    // Only ASCII digits are converted; other scripts' digits are preserved.
    assert_eq!(transliterate("০১২৩৪৫৬৭৮৯ ١٢ １２"), "০১২৩৪৫৬৭৮৯ ١٢ １２");
}

#[test]
fn unsupported_rules_still_preserve_unmatched_characters() {
    for (input, expected) in [
        ("E", "E"),
        ("qo", "qঅ"),
        ("OI", "ওঈ"),
        ("OU", "ওঊ"),
        ("kkh", "কখ"),
        ("K", "K"),
        ("k`h", "ক`h"),
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
    ] {
        assert_eq!(transliterate(input), expected, "input: {input:?}");
    }
}

#[test]
fn long_mixed_input_preserves_character_boundaries() {
    let input = "🙂kh2a\u{301}kUuakIakeeeakoooakOa".repeat(4096);
    let expected = "🙂খ২আ\u{301}কূউয়াকীয়াকীএয়াকুঅয়াকোয়া".repeat(4096);
    assert_eq!(transliterate(&input), expected);
}

#[test]
fn vowel_context_resets_at_unmatched_characters_and_between_calls() {
    for boundary in [' ', ',', '-', '\n', '\r', '\t', '🙂', 'ক', '\u{301}'] {
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
        ] {
            let input = format!("k{boundary}{vowel} {vowel}{boundary}a");
            let expected = format!("ক{boundary}{independent} {independent}{boundary}আ");
            assert_eq!(transliterate(&input), expected, "input: {input:?}");
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
}
