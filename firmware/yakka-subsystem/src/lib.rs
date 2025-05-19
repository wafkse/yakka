#![no_std]
#![forbid(
    missing_docs,
    unsafe_code,
    unused_unsafe,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
//! Yakka Subsystem.
//!
//! See [`Subsystem`] for more information.

use core::{
    fmt,
    ops::{Deref, DerefMut},
};

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};

/// A handle to an active [`Subsystem`].
#[repr(transparent)]
pub struct Handle<T, const N: usize = 1>(Channel<NoopRawMutex, T, N>);

impl<T, const N: usize> Deref for Handle<T, N> {
    type Target = Channel<NoopRawMutex, T, N>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_channel) = self;

        target_channel
    }
}

impl<T, const N: usize> DerefMut for Handle<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_channel) = self;

        target_channel
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for Handle<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Handle").finish_non_exhaustive()
    }
}

/// An asynchronous, cooperative subsystem that is to be executed from the context of a single executor.
///
/// Subsystems act as background tasks that expose a single [`handle`] instance.
///
/// Defer to [`Subsystem::start`] for further information.
///
/// [`handle`]: Subsystem::Handle
/// [`messages`]: Subsystem::Message
pub trait Subsystem {
    /// An unique handler to the subsystem.
    type Handle;

    /// The context gathered by the upstream driver.
    type Context;

    /// Start this subsystem with the default context.
    #[inline]
    fn subsystem(self) -> impl Future<Output = Self::Handle>
    where
        Self::Context: Default,
        Self: Sized,
    {
        Self::subsystem_with_context(self, Self::Context::default())
    }

    /// Start this subsystem with an explicitly attached context.
    ///
    /// This will start a background task, where the actual subsystem will be executed.
    fn subsystem_with_context(
        self,
        target_context: Self::Context,
    ) -> impl Future<Output = Self::Handle>
    where
        Self: Sized;
}
