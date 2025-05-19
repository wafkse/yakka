//! Embeddings inside logging messages.
//!
//! Instead of regular costly formatting procedures, log facades only receive one `&str` and an array of [`Embed`]s.
//!
//! This way, logs can attach arbitrary data, without the formatting cost.

/// An embed inside a log instance.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Embed<'a> {
    /// An embed of a single 64-bit integer.
    Integer(i64),

    /// An embed of a [`str`] borrowed for `'a`.
    Str(&'a str),

    /// An embed of a key-value correspondence.
    KeyValue(&'a Self, &'a Self),
}
