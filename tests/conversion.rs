use jong_bangla_core::transliterate;

#[test]
fn authored_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/reference.tsv"));
}

#[test]
fn vowel_reference_cases_match_exact_unicode() {
    check_fixture(include_str!("fixtures/vowels.tsv"));
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
    for input in ["", "uXYZ", "বাংলা", "é🙂\u{301}", "\t\n !`"] {
        assert_eq!(transliterate(input), input);
    }
    assert_eq!(transliterate("🙂khবাংলা2é"), "🙂খবাংলা২é");
    // Only ASCII digits are converted; other scripts' digits are preserved.
    assert_eq!(transliterate("০১২৩৪৫৬৭৮৯ ١٢ １２"), "০১২৩৪৫৬৭৮৯ ١٢ １２");
}

#[test]
fn unsupported_rules_still_preserve_unmatched_characters() {
    for (input, expected) in [
        ("ua", "uআ"),
        ("kkh", "কখ"),
        ("K", "K"),
        ("k`h", "ক`h"),
        ("k`a", "ক`আ"),
        ("i`", "ই`"),
    ] {
        assert_eq!(transliterate(input), expected, "input: {input:?}");
    }
}

#[test]
fn long_mixed_input_preserves_character_boundaries() {
    let input = "🙂kh2a\u{301}".repeat(4096);
    let expected = "🙂খ২আ\u{301}".repeat(4096);
    assert_eq!(transliterate(&input), expected);
}

#[test]
fn vowel_context_resets_at_unmatched_characters_and_between_calls() {
    for boundary in [' ', ',', '-', '\n', '\r', '\t', '🙂', 'ক', '\u{301}'] {
        let input = format!("k{boundary}a i{boundary}a");
        let expected = format!("ক{boundary}আ ই{boundary}আ");
        assert_eq!(transliterate(&input), expected, "boundary: {boundary:?}");
    }
    assert_eq!(transliterate("k"), "ক");
    assert_eq!(transliterate("a"), "আ");
    assert_eq!(transliterate("i"), "ই");
    assert_eq!(transliterate("a"), "আ");
}
