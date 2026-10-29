//! Deterministic Bangla phonetic transliteration with no external dependencies.
//!
//! Converts supported phonetic sequences and ASCII digits, preserving unmatched
//! Unicode characters. Compatibility with Avro is partial; composition controls
//! and platform integration are not implemented.
//!
//! See the repository's [coverage overview] and [behavior notes] for supported
//! rules, compatibility evidence, and limitations.
//!
//! [coverage overview]: https://github.com/shaadahmmed/jong-bangla-keyboard#current-status
//! [behavior notes]: https://github.com/shaadahmmed/jong-bangla-keyboard/blob/main/docs/v1.md#transliteration-api
//!
//! ```
//! use jong_bangla_core::transliterate;
//!
//! assert_eq!(transliterate("kh12T"), "খ১২ট");
//! assert_eq!(transliterate("ami khani kia"), "আমি খানি কিয়া");
//! assert_eq!(transliterate("s sh S Sh"), "স শ শ ষ");
//! ```

mod transliteration;

pub use transliteration::transliterate;
