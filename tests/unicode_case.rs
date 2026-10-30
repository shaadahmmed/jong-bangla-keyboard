use jong_bangla_core::Composition;
use jong_bangla_core::transliterate;

#[test]
fn unicode_case_policy_matches_exact_engine_outputs() {
    let fixture = include_str!("fixtures/unicode-case-policy.tsv");
    let mut count = 0;
    for line in fixture.lines().filter(|line| !line.starts_with('#')) {
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 5, "invalid policy fixture: {line:?}");
        let [input, expected, points, _, _] = columns.as_slice() else {
            unreachable!();
        };
        let actual = transliterate(input);
        assert_eq!(actual, *expected, "input: {input:?}");
        let actual_points = actual
            .chars()
            .map(|ch| format!("U+{:04X}", ch as u32))
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(actual_points, *points, "code points for input: {input:?}");
        count += 1;
    }
    assert!(count > 0, "policy fixture must contain cases");
}

#[test]
fn unicode_case_preservation_survives_editing_and_commit() {
    let mut word = Composition::new();
    for (key, expected) in [('k', "ক"), ('İ', "কİ"), ('a', "কİআ")] {
        assert!(word.insert(key));
        assert_eq!(word.preedit(), expected);
    }
    for expected in ["কİ", "ক", ""] {
        assert!(word.backspace());
        assert_eq!(word.preedit(), expected);
    }
    assert!(!word.is_active());
    // Preserve each unmatched scalar, including its case and combining mark.
    for key in "ÉΣK𐐀\u{0301}".chars() {
        assert!(word.insert(key));
    }
    assert_eq!(word.commit().as_deref(), Some("ÉΣK𐐀\u{0301}"));
    assert!(!word.is_active());
}
