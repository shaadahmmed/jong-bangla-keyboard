//! Editable input state; platform adapters choose which events invoke each action.

use crate::transliterate;

/// Editing keys with settled composition behavior, independent of platform events.
///
/// The adapter normalizes framework keys and modifiers into these values. Other
/// shortcuts, Escape, navigation, and lifecycle events need separate policies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionKey {
    Backspace,
    CtrlBackspace,
    Space,
    Enter,
}

/// Ordered instructions for the adapter after handling an editing key.
///
/// Publish the controller's current preedit, commit `committed` if nonempty,
/// then forward the original key exactly once when `consumed` is false.
/// This type omits Debug formatting because committed text is user input.
#[must_use]
pub struct KeyEffect {
    pub consumed: bool,
    pub committed: Option<String>,
}

/// One uncommitted raw-input buffer and its converted Bangla preedit.
///
/// Activity depends on raw input, because escapes can produce an empty preedit.
/// No document history, cursor, platform events, or candidate state is stored.
/// Debug formatting is deliberately omitted to avoid accidental input logging.
///
/// ```
/// use jong_bangla_core::Composition;
///
/// let mut word = Composition::new();
/// for key in "kh12T".chars() {
///     assert!(word.insert(key));
/// }
/// assert_eq!(word.preedit(), "খ১২ট");
/// assert!(word.delete_word());
/// assert!(!word.delete_word()); // The application can handle further word deletion.
/// assert_eq!(word.commit(), None);
/// ```
#[derive(Default)]
pub struct Composition {
    raw: String,
    preedit: String,
}

impl Composition {
    /// Creates an empty composition.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether any uncommitted raw input remains, including invisible escapes.
    pub fn is_active(&self) -> bool {
        !self.raw.is_empty()
    }

    /// The current converted output, which may be empty even while active.
    pub fn preedit(&self) -> &str {
        &self.preedit
    }

    /// Appends one Unicode scalar and recomputes the entire preedit.
    ///
    /// Returns false without changing state for whitespace or control characters.
    /// Adapters must route boundaries and shortcuts separately, finalizing active
    /// composition before forwarding a boundary to the application.
    pub fn insert(&mut self, input: char) -> bool {
        if input.is_whitespace() || input.is_control() {
            return false;
        }
        self.raw.push(input);
        self.recompute();
        true
    }

    /// Handles a normalized editing key using the confirmed v1 policy.
    ///
    /// Backspace and Ctrl+Backspace are consumed only with active raw input.
    /// Space and Enter finalize composition and are always forwarded, so the
    /// application inserts the space or performs its usual Enter action.
    pub fn handle_key(&mut self, key: CompositionKey) -> KeyEffect {
        match key {
            CompositionKey::Backspace => KeyEffect {
                consumed: self.backspace(),
                committed: None,
            },
            CompositionKey::CtrlBackspace => KeyEffect {
                consumed: self.delete_word(),
                committed: None,
            },
            CompositionKey::Space | CompositionKey::Enter => KeyEffect {
                consumed: false,
                committed: self.commit(),
            },
        }
    }

    /// Removes one raw Unicode scalar and recomputes, rather than erasing output.
    ///
    /// Returns false only when no raw input exists, allowing normal application
    /// Backspace handling. Each accepted scalar is one editable input unit.
    pub fn backspace(&mut self) -> bool {
        if self.raw.pop().is_none() {
            return false;
        }
        self.recompute();
        true
    }

    /// Deletes the whole active buffer without committing or touching document text.
    ///
    /// Returns whether raw input was removed. An adapter can forward a word-delete
    /// shortcut to the application only when this returns false.
    pub fn delete_word(&mut self) -> bool {
        self.cancel()
    }

    /// Takes the converted output and clears all composition state.
    ///
    /// Returns `None` when inactive. `Some("")` represents active input that produced
    /// no visible output; adapters should skip an empty framework commit while
    /// still treating that composition as finalized.
    pub fn commit(&mut self) -> Option<String> {
        if !self.is_active() {
            return None;
        }
        self.raw.clear();
        Some(std::mem::take(&mut self.preedit))
    }

    /// Discards active input without committing; returns whether input existed.
    pub fn cancel(&mut self) -> bool {
        let was_active = self.is_active();
        self.reset();
        was_active
    }

    /// Clears state unconditionally, without producing committed text.
    ///
    /// Use a separate instance per input context. An adapter must explicitly
    /// commit first if its lifecycle policy requires preserving pending text.
    pub fn reset(&mut self) {
        self.raw.clear();
        self.preedit.clear();
    }

    fn recompute(&mut self) {
        self.preedit = transliterate(&self.raw);
    }
}
