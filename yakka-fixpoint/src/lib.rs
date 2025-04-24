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
//! # Yakka Fixpoint
//!
//! This crate provides various utilities for working with fixed-point numbers.

use yakka_number::scalar::Scalar;

/// A fixed-point number type pinned to `N` bits of decimal precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Fixpoint<T, const N: usize = 4>(T)
where
    T: Scalar;

/// A type that represents the number of bytes allocated for the non-integer part of a fixed-point number.
pub enum Fractional<const N: usize> {}

mod private {
    /// A sealed trait to prevent external implementations of exported traits.
    pub trait Sealed {}
}

/// A trait whose only purpose is to signal whether a [`Fractional<N>`] type is supported for some `N`.
pub trait SupportedFraction<const N: usize>: private::Sealed {}

// TODO: Implement arithmetic operations for `Fixpoint`.
