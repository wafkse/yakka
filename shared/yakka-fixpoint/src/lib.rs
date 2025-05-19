#![cfg_attr(not(test), no_std)]
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

use core::{
    marker,
    ops::{Add, Div, Mul, Rem, Shl, Shr, Sub},
};

use yakka_number::scalar::Scalar;

mod private {
    /// A sealed trait to prevent external implementations of exported traits.
    pub trait Sealed {}
}

/// A type that represents the number of bits allocated for the non-integer part of a fixed-point
/// number.
pub struct Fractional<T, const N: usize>(marker::PhantomData<T>);

/// A trait whose only purpose is to signal whether a [`Fractional<N>`] type is supported for some
/// fraction `N`.
#[diagnostic::on_unimplemented(message = "{Self} is not a valid fraction")]
pub trait Fraction<const N: usize>: private::Sealed {}

/// A structure encompasing the integer and fractional part of a fixed-point number.
///
/// # Layout
///
/// The integer part is stored as-is, however, the fractional part is stored MSB-first, i.e, the most significant bit represents biggest fractional value possible.
///
/// # Conversion mechanism
///
/// For a [`Fixpoint<T, N>`], the implied driver type `T` is used to extract
///  - The *integer* part: extracted as-is using bitwise operations.
///  - The *fractional* part: extracted as-is, but then left-shifted towards MSB as per the defined fractional bits to reach a standard-ish floating-point representation.
#[derive(Eq, PartialEq, PartialOrd)]
pub struct Partwise {
    /// The integer part of the number.
    integer: u64,

    /// The fractional part of the number.
    fractional: u64,
}

/// Trait for integer types that can be used to drive a fixed-point number.
pub trait Fixed: Scalar + private::Sealed {
    /// Convert the integer into a [`Partwise`] decomposition as per the target scale `N`.
    fn partwise<const N: usize>(self) -> Partwise;

    /// Interpret the target [`Partwise`] as a fixed-point number as per the target scale `N`.
    fn interpret<const N: usize>(target_partwise: Partwise) -> Self;
}

/// A trait representing an IEEE floating-point number of any kind.
pub trait Floating {
    /// The total size of the floating-point number.
    const BITS: u32;

    /// The size of the mantissa of the floating-point number.
    const MANTISSA_BITS: u32;

    /// The bits allocated to the exponent part of this floating-point number.
    const EXPONENT_BITS: u32;

    /// Convert a [`Partwise`] decomposition into this floating-point number.
    fn partwise(target_value: Partwise) -> Self;

    /// Turn this floating-point number into its [`Partwise`] decomposition.
    fn raw(self) -> Partwise;
}

/// A fixed-point number type pinned to `N` bits of decimal precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Fixpoint<T, const N: usize>(T)
where
    T: Fixed,
    Fractional<T, N>: Fraction<N>;

impl<T, const N: usize> Fixpoint<T, N>
where
    T: Fixed,
    Fractional<T, N>: Fraction<N>,
{
    /// Instantiate a new [`Fixpoint`] literal from its raw value.
    #[inline]
    pub const fn raw(target_value: T) -> Self {
        Self(target_value)
    }

    /// Unwrap the internal fixed-point value from this new-type.
    #[inline]
    pub const fn value(self) -> T {
        let Self(target_value) = self;

        target_value
    }

    /// Convert to a [`Fixpoint`] literal from a floating-point number `F`.
    ///
    /// The [`Fixpoint`] literal will adopt the absolute value of `F`.
    #[inline]
    pub fn convert<F>(target_value: F) -> Self
    where
        F: Floating,
    {
        let target_partwise = target_value.raw();

        Self(T::interpret::<N>(target_partwise))
    }

    /// Convert a [`Fixpoint`] number into a regular floating-point number `F`.
    ///
    /// The yielded float `F` is guaranteed to be non-negative.
    #[inline]
    pub fn float<F>(self) -> F
    where
        F: Floating,
    {
        let Self(target_value) = self;

        let target_partwise = target_value.partwise::<N>();

        F::partwise(target_partwise)
    }

    /// Convert a scalar value `T` into a [`Fixpoint`] number.
    ///
    /// This will strip any non-fitting bits from the scalar value.
    #[inline]
    pub fn scalar(target_scalar: T) -> Self {
        /* interpret as if it had zero fractional bits */
        let target_partwise = target_scalar.partwise::<0>();

        /* reinterpret it back with N fractional bits, effectively scaling it to our desired range */
        Self(T::interpret::<N>(target_partwise))
    }
}

macro_rules! floating {
    () => {};
    (
        $(
            $target_type:ty
        ),+

        $(,)?
    ) => {
        $(
            impl Floating for $target_type {
                const BITS: u32 = core::mem::size_of::<Self>() as u32 * u8::BITS;

                const MANTISSA_BITS: u32 = Self::MANTISSA_DIGITS;

                const EXPONENT_BITS: u32 = Self::BITS - Self::MANTISSA_BITS - 1;

                #[inline]
                fn partwise(Partwise { integer, fractional, .. }: Partwise) -> Self {
                    let integer_part = integer as Self;

                    let fractional_bits = fractional >> (u64::BITS - Self::MANTISSA_BITS);

                    let fractional_part = (fractional_bits as Self) / (1u64 << Self::MANTISSA_BITS) as Self;

                    let fractional_part = fractional_part.abs();

                    integer_part + fractional_part
                }

                #[inline]
                fn raw(self) -> Partwise {
                    let absolute_value = self.abs();

                    let truncated_value = absolute_value as u64;
                    let fractional_value = absolute_value - (truncated_value as Self);

                    let fractional_value = (fractional_value * u64::MAX as Self).abs();

                    let fractional_value = fractional_value as u64;

                    let (integer, fractional) = (truncated_value, fractional_value);

                    Partwise { integer, fractional }
                }
            }
        )+
    };
}

floating!(f32, f64);

macro_rules! operator {
    () => {};
    (
        $($target_trait:ident),+ $(,)?
    ) => {
        permafrost::embed!(
            $(
                impl<T, const N: usize> $target_trait for Fixpoint<T, N>
                where
                    T: Fixed + $target_trait<T, Output = T>,
                    Fractional<T, N>: Fraction<N>,
                {
                    type Output = Self;

                    #[inline]
                    fn [< $target_trait >]:case{snake} (self, Self(rhs): Self) -> Self::Output {
                        let Self(lhs) = self;

                        let target_value = <T as $target_trait> :: [< $target_trait >]:case{snake}(lhs, rhs);

                        Self(target_value)
                    }
                }
            )+
        );
    };
}

operator!(Add, Sub, Shl, Shr, Rem);

impl<T, const N: usize> Mul for Fixpoint<T, N>
where
    T: Fixed + Mul<T, Output = T> + Shr<usize, Output = T>,
    Fractional<T, N>: Fraction<N>,
{
    type Output = Self;
    #[inline]
    fn mul(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;

        let target_value = <T as Mul>::mul(lhs, rhs) >> N;

        Self(target_value)
    }
}

impl<T, const N: usize> Div for Fixpoint<T, N>
where
    T: Fixed + Div<T, Output = T> + Shl<usize, Output = T>,
    Fractional<T, N>: Fraction<N>,
{
    type Output = Self;
    #[inline]
    fn div(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;

        let target_value = <T as Div>::div(lhs << N, rhs);

        Self(target_value)
    }
}

macro_rules! fraction {
    () => {};
    (
        @ [$($target_size:literal),+ $(,)?] for $target_type:ident
    ) => {
        impl private::Sealed for $target_type {}

        impl Fixed for $target_type {
            #[inline]
            fn partwise<const N: usize>(self) -> Partwise {
                let integer_part = self >> N;

                let fractional_mask = (1u64 << N) - 1;

                let fractional_part = (self & fractional_mask as Self) as u64;

                let fractional_part = fractional_part << (u64::BITS - N as u32);

                let (integer, fractional) = (integer_part as u64, fractional_part as u64);

                Partwise { integer, fractional }
            }

            #[inline]
            fn interpret<const N: usize>(Partwise { integer, fractional }: Partwise) -> Self {
                let integer_part = integer << N;

                let fractional_part = fractional >> (u64::BITS - N as u32);

                (integer_part | fractional_part as u64) as Self
            }
        }

        $(
            impl private::Sealed for Fractional<$target_type, $target_size> {}

            impl Fraction<$target_size> for Fractional<$target_type, $target_size> {}
        )+
    };
    (
        [$($target_size:literal),+ $(,)?] for [
            $target_type:ident
        ]
    ) => {
        fraction!(@ [$($target_size),+] for $target_type);
    };
    (
        [$($target_size:literal),+ $(,)?] for [
            $target_type:ident,

            $($target_tt:tt)*
        ]
    ) => {
        fraction!(@ [$($target_size),+] for $target_type);

        fraction!([$($target_size),+] for [$($target_tt)*]);
    };
}

fraction!([1, 2, 3, 4, 5, 6, 7] for [u8]);

fraction!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] for [u16]);

fraction!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31] for [u32]);

fraction!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63] for [u64]);

#[cfg(test)]
mod tests {
    use crate::Fixpoint;

    fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_float_to_fixpoint_and_back_f32() {
        let values = [0.0_f32, -1.0, 1.5, -2.75, 123.456, 0.0001];

        for &val in &values {
            let fp = Fixpoint::<u32, 16>::convert(val);

            let roundtrip: f32 = fp.float();

            assert!(
                approx_eq(val.abs() as f64, roundtrip as f64, 0.00001),
                "Failed roundtrip: {} -> {:?} -> {}",
                val,
                fp,
                roundtrip
            );
        }
    }

    #[test]
    fn test_float_to_fixpoint_and_back_f64() {
        let values = [0.0_f64, 12.0, -21.0, -1.0, 1.5, -2.75, 123.456, -0.0001];

        for &val in &values {
            let fp = Fixpoint::<u64, 32>::convert(val);

            let roundtrip: f64 = fp.float();

            assert!(
                approx_eq(val.abs(), roundtrip, 0.00001),
                "Failed roundtrip: {} -> {:?} -> {}",
                val,
                fp,
                roundtrip
            );
        }
    }

    #[test]
    fn test_raw_construction_and_conversion() {
        let target_value: u32 = 12345;

        let target_number @ Fixpoint(target_inner) = Fixpoint::<u32, 8>::raw(target_value);

        let converted_number = Fixpoint::<u32, 8>::raw(target_inner);

        assert_eq!(target_number, converted_number);
    }

    #[test]
    fn test_addition() {
        let a = Fixpoint::<u32, 8>::convert(1.5);

        let b = Fixpoint::<u32, 8>::convert(2.25);

        let result = a + b;

        let float_result: f64 = result.float();

        assert!(approx_eq(float_result, 3.75, 0.01));
    }

    #[test]
    fn test_subtraction() {
        let a = Fixpoint::<u32, 8>::convert(5.0);

        let b = Fixpoint::<u32, 8>::convert(3.0);

        let result = a - b;

        let float_result: f64 = result.float();

        assert!(approx_eq(float_result, 2.0, 0.01));
    }

    #[test]
    fn test_multiplication() {
        let a = Fixpoint::<u32, 8>::convert(2.0);

        let b = Fixpoint::<u32, 8>::convert(3.5);

        let result = a * b;

        let float_result: f64 = result.float();

        assert!(approx_eq(float_result, 7.0, 0.1));
    }

    #[test]
    fn test_division() {
        let a = Fixpoint::<u32, 8>::convert(7.0);

        let b = Fixpoint::<u32, 8>::convert(2.0);

        let result = a / b;

        let float_result: f64 = result.float();

        assert!(approx_eq(float_result, 3.5, 0.1));
    }

    #[test]
    fn test_remainder() {
        let a = Fixpoint::<u32, 8>::convert(7.0);

        let b = Fixpoint::<u32, 8>::convert(2.0);

        let result = a % b;

        let float_result: f64 = result.float();

        assert!(approx_eq(float_result, 1.0, 0.1));
    }

    #[test]
    fn test_zero() {
        let zero = Fixpoint::<u32, 8>::convert(0.0);

        let back: f64 = zero.float();

        assert!(approx_eq(back, 0.0, 0.0001));
    }

    #[test]
    fn test_negative() {
        let neg = Fixpoint::<u32, 8>::convert(-42.625);

        let back: f64 = neg.float();

        assert!(back > 1.0, "{}", back);

        assert!(approx_eq(back, 42.625, 0.01));
    }
}
