use jong_bangla_core::transliterate;

#[test]
fn authored_reference_cases_match_exact_unicode() {
    for line in include_str!("fixtures/reference.tsv").lines() {
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
    }
}

#[test]
fn empty_input_and_unmatched_unicode_are_preserved() {
    for input in ["", "aXYZ", "বাংলা", "é🙂\u{301}", "\t\n !`"] {
        assert_eq!(transliterate(input), input);
    }
    assert_eq!(transliterate("🙂khবাংলা2é"), "🙂খবাংলা২é");
    // Only ASCII digits are converted; other scripts' digits are preserved.
    assert_eq!(transliterate("০১২৩৪৫৬৭৮৯ ١٢ １２"), "০১২৩৪৫৬৭৮৯ ١٢ １２");
}

#[test]
fn conversion_is_partial_and_has_no_context_or_escape_rules_yet() {
    for (input, expected) in [("ka", "কa"), ("kkh", "কখ"), ("K", "K"), ("k`h", "ক`h")] {
        assert_eq!(transliterate(input), expected, "input: {input:?}");
    }
}

#[test]
fn long_mixed_input_preserves_character_boundaries() {
    let input = "🙂kh2a\u{301}".repeat(4096);
    let expected = "🙂খ২a\u{301}".repeat(4096);
    assert_eq!(transliterate(&input), expected);
}
