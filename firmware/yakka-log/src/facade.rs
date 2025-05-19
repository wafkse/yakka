//! Asynchronous logging facade.

use crate::embed::Embed;

/// A trait that serves as an asynchronous logging interface.
pub trait Log {
    /// Log the target message into the logger.
    fn log<'a>(
        &self,
        target_content: &'static str,
        target_embed: &[Embed<'a>],
    ) -> impl Future + use<'a, Self>;
}
