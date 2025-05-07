//! Ratios for indicating the relative size of two values.

use core::{
    num::NonZero,
    ops::{Deref, Div, Mul},
};



mod private {
    /// A private trait to prevent external implementations of local traits.
    pub trait Sealed {}
}

/// A helper trait for compile-time ratio constants.
pub trait Ratioable<const N: usize>: private::Sealed {
    /// The ratio of the value.
    const RATIO: Ratio = const {
        match NonZero::new(N) {
            Some(value) => Ratio(value),
            None => unreachable!(),
        }
    };
}

/// An uninhabited type for compile-time ratio constants.
///
/// See the [`Ratioable`] trait for more information.
pub enum Const<const N: usize> {}

impl<const N: usize> private::Sealed for Const<N> {}

impl<const N: usize> Ratioable<N> for Const<N> {}

/// A ratio between two values.
///
/// This has no meaning on its own, as it is just a number. To present itself with a meaning, it
/// must accompany a directionality in the relation.
///
/// For example, take a 2-tuple `(2, 4)`, which is a ratio of `2:4`. This can be interpreted as `2`
/// being half of `4`, or `4` being twice as much as `2`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Ratio(pub NonZero<usize>);

impl Deref for Ratio {
    type Target = NonZero<usize>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

/// A trait for types that can be transformed into a distinct equivalence through the use of a
/// [`Ratio`].
pub trait Transform {
    /// Scale the value up by the given [`Ratio`].
    fn up(self: Self, ratio: Ratio) -> Self;

    /// Scale the value down by the given [`Ratio`].
    fn down(self: Self, ratio: Ratio) -> Self;
}

macro_rules! transform {
    () => {};
    (
        for [
            $(
                $scalar_ty:ty
            ),*
        ]
    ) => {
        $(
            impl Transform for $scalar_ty {
                #[inline]
                fn up(self: Self, ratio: Ratio) -> Self {
                    let Ratio(target_ratio) = ratio;

                    let target_value = target_ratio.get() as $scalar_ty;

                    target_value.mul(self)
                }

                #[inline]
                fn down(self: Self, ratio: Ratio) -> Self {
                    let Ratio(target_ratio) = ratio;

                    let target_value = target_ratio.get() as $scalar_ty;

                    target_value.div(self)
                }
            }
        )*
    };
}

transform!(for [f32, f64] );
transform!(for [i8, i16, i32, i64, isize] );
transform!(for [u8, u16, u32, u64, usize]);
transform!(for [u128, i128]);
