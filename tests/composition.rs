use jong_bangla_core::{Composition, CompositionKey};

fn insert(composition: &mut Composition, input: &str) {
    for ch in input.chars() {
        assert!(composition.insert(ch));
    }
}

fn assert_empty(composition: &Composition) {
    assert!(!composition.is_active());
    assert_eq!(composition.preedit(), "");
}

#[test]
fn empty_actions_leave_document_editing_to_the_application() {
    for mut composition in [Composition::new(), Composition::default()] {
        assert_empty(&composition);
        assert!(!composition.backspace());
        assert!(!composition.delete_word());
        assert!(!composition.cancel());
        assert_eq!(composition.commit(), None);
        composition.reset();
        assert_empty(&composition);
    }
}

#[test]
fn typing_recomputes_contextual_vowels_and_longer_matches() {
    let mut composition = Composition::new();
    for (key, expected) in [
        ('k', "ক"),
        ('h', "খ"),
        ('a', "খা"),
        ('n', "খান"),
        ('i', "খানি"),
    ] {
        assert!(composition.insert(key));
        assert!(composition.is_active());
        assert_eq!(composition.preedit(), expected);
    }
    assert_eq!(composition.commit().as_deref(), Some("খানি"));
    assert_empty(&composition);
}

#[test]
fn backspace_rebuilds_conjuncts_from_raw_keys() {
    let mut composition = Composition::new();
    insert(&mut composition, "kkh");
    assert_eq!(composition.preedit(), "ক\u{09cd}ষ");
    for expected in ["ক\u{09cd}ক", "ক", ""] {
        assert!(composition.backspace());
        assert_eq!(composition.preedit(), expected);
    }
    assert_empty(&composition);
    assert!(!composition.backspace());
}

#[test]
fn backspace_restores_vowel_and_reph_context() {
    let mut composition = Composition::new();
    insert(&mut composition, "ki");
    assert_eq!(composition.preedit(), "কি");
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "ক");
    composition.reset();
    insert(&mut composition, "rrq");
    assert_eq!(composition.preedit(), "র\u{09cd}ক");
    for expected in ["রর", "র", ""] {
        assert!(composition.backspace());
        assert_eq!(composition.preedit(), expected);
    }
}

#[test]
fn digits_are_inserted_and_committed_as_bangla() {
    let mut composition = Composition::new();
    insert(&mut composition, "0123456789");
    assert_eq!(composition.preedit(), "০১২৩৪৫৬৭৮৯");
    assert!(composition.backspace());
    assert_eq!(composition.commit().as_deref(), Some("০১২৩৪৫৬৭৮"));
    assert_empty(&composition);
}

#[test]
fn backspace_removes_whole_unicode_scalars_including_copied_input() {
    let mut composition = Composition::new();
    insert(&mut composition, "ক🙂é\u{0301}");
    assert_eq!(composition.preedit(), "ক🙂é\u{0301}");
    for expected in ["ক🙂é", "ক🙂", "ক", ""] {
        assert!(composition.backspace());
        assert_eq!(composition.preedit(), expected);
    }
    assert_empty(&composition);
}

#[test]
fn punctuation_and_symbol_escapes_remain_editable() {
    let mut composition = Composition::new();
    for expected in ["\u{0964}", "\u{0964}\u{0964}", "..."] {
        assert!(composition.insert('.'));
        assert_eq!(composition.preedit(), expected);
    }
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "\u{0964}\u{0964}");
    composition.reset();
    assert!(composition.insert(':'));
    assert_eq!(composition.preedit(), "\u{0983}");
    assert!(composition.insert('`'));
    assert_eq!(composition.preedit(), ":");
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "\u{0983}");
}

#[test]
fn vowel_and_final_t_escapes_recompute_after_deletion() {
    let mut composition = Composition::new();
    insert(&mut composition, "a`");
    assert_eq!(composition.preedit(), "\u{09be}");
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "আ");
    composition.reset();
    insert(&mut composition, "t``");
    assert_eq!(composition.preedit(), "\u{09ce}");
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "ত");
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "ত");
}

#[test]
fn invisible_input_is_active_and_backspace_restores_its_visible_prefix() {
    let mut composition = Composition::new();
    insert(&mut composition, "o`");
    assert!(composition.is_active());
    assert_eq!(composition.preedit(), "");
    assert!(composition.backspace());
    assert_eq!(composition.preedit(), "অ");
    assert!(composition.backspace());
    assert_empty(&composition);
}

#[test]
fn commit_distinguishes_invisible_input_from_no_input() {
    let mut composition = Composition::new();
    for input in ["`", "o`"] {
        insert(&mut composition, input);
        assert_eq!(composition.commit().as_deref(), Some(""));
        assert_empty(&composition);
        assert_eq!(composition.commit(), None);
    }
}

#[test]
fn word_deletion_clears_only_the_active_buffer() {
    let mut composition = Composition::new();
    insert(&mut composition, "ami");
    assert_eq!(composition.commit().as_deref(), Some("আমি"));
    for input in ["khani12", "...", "o`"] {
        insert(&mut composition, input);
        assert!(composition.delete_word());
        assert_empty(&composition);
        assert_eq!(composition.commit(), None);
        assert!(!composition.delete_word());
    }
}

#[test]
fn typing_after_word_deletion_starts_fresh_context() {
    let mut composition = Composition::new();
    insert(&mut composition, "k");
    assert!(composition.delete_word());
    insert(&mut composition, "i");
    assert_eq!(composition.preedit(), "ই");
    assert_eq!(composition.commit().as_deref(), Some("ই"));
}

#[test]
fn cancellation_and_reset_discard_visible_and_invisible_input() {
    let mut composition = Composition::new();
    for input in ["k", "o`"] {
        insert(&mut composition, input);
        assert!(composition.cancel());
        assert_empty(&composition);
        assert_eq!(composition.commit(), None);
        insert(&mut composition, input);
        composition.reset();
        assert_empty(&composition);
        assert_eq!(composition.commit(), None);
    }
    insert(&mut composition, "i");
    assert_eq!(composition.preedit(), "ই");
}

#[test]
fn commits_start_fresh_context_and_cannot_be_recovered() {
    let mut composition = Composition::new();
    insert(&mut composition, "k");
    assert_eq!(composition.commit().as_deref(), Some("ক"));
    assert!(!composition.backspace());
    insert(&mut composition, "i");
    assert_eq!(composition.commit().as_deref(), Some("ই"));
    assert_empty(&composition);
}

#[test]
fn whitespace_and_control_input_is_rejected_without_mutating_state() {
    let mut composition = Composition::new();
    for initial in ["", "ki", "o`"] {
        insert(&mut composition, initial);
        let expected = composition.preedit().to_owned();
        for ch in [
            ' ', '\t', '\n', '\r', '\0', '\u{007f}', '\u{0085}', '\u{00a0}', '\u{2003}', '\u{2028}',
        ] {
            assert!(!composition.insert(ch));
            assert_eq!(composition.is_active(), !initial.is_empty());
            assert_eq!(composition.preedit(), expected);
        }
        composition.reset();
    }
}

#[test]
fn input_contexts_keep_independent_state() {
    let mut first = Composition::new();
    let mut second = Composition::new();
    insert(&mut first, "k");
    insert(&mut second, "i");
    assert_eq!(first.preedit(), "ক");
    assert_eq!(second.preedit(), "ই");
    first.reset();
    assert_eq!(second.commit().as_deref(), Some("ই"));
    assert_empty(&first);
    assert_empty(&second);
}

#[test]
fn ctrl_backspace_consumes_only_active_word_deletion_including_invisible_input() {
    let mut composition = Composition::new();
    for input in ["khani12", "...", "o`", "`"] {
        insert(&mut composition, input);
        let deleted = composition.handle_key(CompositionKey::CtrlBackspace);
        assert!(deleted.consumed);
        assert_eq!(deleted.committed, None);
        assert_empty(&composition);
        let forwarded = composition.handle_key(CompositionKey::CtrlBackspace);
        assert!(!forwarded.consumed);
        assert_eq!(forwarded.committed, None);
    }
}

#[test]
fn normalized_backspace_recomputes_then_forwards_only_after_raw_input_is_empty() {
    let mut composition = Composition::new();
    insert(&mut composition, "o`");
    for expected in ["অ", ""] {
        let effect = composition.handle_key(CompositionKey::Backspace);
        assert!(effect.consumed);
        assert_eq!(effect.committed, None);
        assert_eq!(composition.preedit(), expected);
    }
    let forwarded = composition.handle_key(CompositionKey::Backspace);
    assert!(!forwarded.consumed);
    assert_eq!(forwarded.committed, None);
}

#[test]
fn space_and_enter_commit_once_and_forward_the_same_key() {
    let mut composition = Composition::new();
    for key in [CompositionKey::Space, CompositionKey::Enter] {
        for input in ["khani12", "...", "o`", ""] {
            insert(&mut composition, input);
            let expected = match input {
                "khani12" => Some("খানি১২"),
                "..." => Some("..."),
                "o`" => Some(""),
                "" => None,
                _ => unreachable!(),
            };
            let effect = composition.handle_key(key);
            assert!(!effect.consumed);
            assert_eq!(effect.committed.as_deref(), expected);
            assert_empty(&composition);
            let repeated = composition.handle_key(key);
            assert!(!repeated.consumed);
            assert_eq!(repeated.committed, None);
        }
    }
}

#[test]
fn ctrl_backspace_after_a_boundary_leaves_committed_word_deletion_to_the_app() {
    let mut composition = Composition::new();
    insert(&mut composition, "ami");
    let boundary = composition.handle_key(CompositionKey::Space);
    assert_eq!(boundary.committed.as_deref(), Some("আমি"));
    assert!(!boundary.consumed);
    let deletion = composition.handle_key(CompositionKey::CtrlBackspace);
    assert!(!deletion.consumed);
    assert_eq!(deletion.committed, None);
    insert(&mut composition, "i");
    assert_eq!(composition.preedit(), "ই");
}
