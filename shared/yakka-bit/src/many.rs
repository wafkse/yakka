//! Extract many bits at a time.

use yakka_number::scalar::Scalar;

use crate::{
    handle::{Bit, BitMut},
    size::{For, Size},
    take::Bits,
};

/// A trait for types that can have a sequence of bits `N..=M` extracted from them.
pub trait Extract<const N: usize, const M: usize>: Scalar {
    /// The output scalar type.
    type Output: Scalar;

    /// Extract the target bits from the target value, and return them as a new type: [`Extract::Output`].
    fn extract(&self) -> Self::Output;

    /// Fuse the target [`Extract::Output`] value back into the source type.
    ///
    /// Yields back the previous extracted value.
    fn fuse(&mut self, target_value: Self::Output) -> Self::Output;
}

/// A macro to implement the [`Extract`] trait for a target type.
///
/// This macro will take a list of valid bit indices, and implement all possible 2-element permutations of the [`Extract<N, M>`] trait for the target type.
macro_rules! extractor {
    (
        @ impl [$target_index:literal] for $target_type:ty
    ) => {};
    (
        @ impl override [$target_index:literal] for $target_type:ty
    ) => {};
    (
        # impl ($target_left:literal $target_right:literal) for $target_type:ty
    ) => {
        #[automatically_derived]
        impl Extract<$target_left, $target_right> for $target_type
        {
            type Output = <Size as For<{ $target_right - $target_left + 1 }>>::Target;

            #[inline]
            fn extract(&self) -> Self::Output {
                const BITSET_WIDTH: usize = $target_right - $target_left + 1;

                const BITSET_MSB: usize = BITSET_WIDTH - 1;

                const BITSET_MSB_MASK: $target_type = (1 << BITSET_MSB) - 1;

                const EXTRACT_MASK: $target_type = BITSET_MSB_MASK << $target_left;

                let target_value = *self;

                let target_value = target_value & EXTRACT_MASK;

                let target_value = target_value >> $target_left;

                target_value as <Self as Extract<$target_left, $target_right>>::Output
            }

            #[inline]
            fn fuse(&mut self, target_output: Self::Output) -> Self::Output {
                const BITSET_WIDTH: usize = $target_right - $target_left + 1;

                const BITSET_MSB: usize = BITSET_WIDTH - 1;

                const BITSET_MSB_MASK: $target_type = (1 << BITSET_MSB) - 1;

                const EXTRACT_MASK: $target_type = BITSET_MSB_MASK << $target_left;

                const FUSE_MASK: $target_type = !EXTRACT_MASK;

                let existing_value = <Self as Extract<$target_left, $target_right>>::extract(self);

                let target_value = target_output as $target_type;

                let target_value = target_value << $target_left;

                let target_value = target_value & EXTRACT_MASK;

                let target_value = target_value | (*self & FUSE_MASK);

                *self = target_value;

                existing_value
            }
        }
    };
    (
        @ impl override [$target_index:literal $target_next_index:literal $($target_rest:literal)*] for $target_type:ty
    ) => {
        extractor!(# impl ($target_index $target_next_index) for $target_type);

        extractor!(@ impl override [$target_index $($target_rest)*] for $target_type);
    };
    (
        @ impl [$target_index:literal $target_next_index:literal $($target_rest:literal)*] for $target_type:ty
    ) => {
        extractor!(# impl ($target_index $target_next_index) for $target_type);

        extractor!(@ impl override [$target_index $($target_rest)*] for $target_type);
        extractor!(@ impl [$target_next_index $($target_rest)*] for $target_type);
    };
    (
        [
            $($target_index:literal),*
        ] for $target_type:ty
    ) => {
        extractor!(@ impl [ $($target_index)* ] for $target_type);
    };
}

extractor!(
    /* 8 indices -> 28 implementations */
    [0, 1, 2, 3, 4, 5, 6, 7] for u8
);

extractor!(
    /* 16 indices -> 120 implementations */
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] for u16
);

/// An immutable wrapper around the [`Extract<N, M>`] trait's output type.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Extracted<const N: usize, const M: usize, T>(T, T::Output)
where
    T: Extract<N, M>;

impl<const N: usize, const M: usize, T> Extracted<N, M, T>
where
    T: Extract<N, M>,
{
    /// Wrap the target value's extracted bits in an [`Extracted`] struct.
    #[inline]
    pub fn wrap(&target_value: &T) -> Self {
        let target_output = target_value.extract();

        Self(target_value, target_output)
    }

    /// Unwrap the extracted value from this wrapper.
    #[inline]
    pub const fn value(&self) -> T {
        let &Self(target_value, ..) = self;

        target_value
    }

    /// Unwrap the extracted value from this wrapper.
    #[inline]
    pub const fn output(&self) -> T::Output {
        let &Self(.., target_value) = self;

        target_value
    }
}

impl<const N: usize, const M: usize, T> Extracted<N, M, T>
where
    T: Extract<N, M>,
{
    /// Overwrite the extracted bits with a the target sequence.
    #[inline]
    pub fn overwrite(&self, target_value: T::Output) -> Self {
        let &Self(mut target_clone, ..) = self;

        let _ = target_clone.fuse(target_value);

        Self::wrap(&target_clone)
    }
}

impl<const N: usize, const M: usize, T> Extracted<N, M, T>
where
    T: Extract<N, M>,
{
    /// Access the `K`-th bit of the extracted value through the [`Bit`] wrapper.
    #[inline]
    pub const fn bit<'a, const K: usize>(&'a self) -> Bit<'a, T, K>
    where
        T: Bits<K>,
    {
        let &Self(ref target_value, ..) = self;

        Bit::wrap(target_value)
    }

    /// Access the `K`-th bit of the extracted value through the [`BitMut`] wrapper.
    #[inline]
    pub const fn bit_mut<'a, const K: usize>(&'a mut self) -> BitMut<'a, T, K>
    where
        T: Bits<K>,
    {
        let &mut Self(ref mut target_value, ..) = self;

        BitMut::wrap(target_value)
    }
}

/// A wrapper around the [`Extract<N, M>`] trait's output type.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ExtractedMut<'a, const N: usize, const M: usize, T>(&'a mut T)
where
    T: Extract<N, M>;

impl<'a, const N: usize, const M: usize, T> ExtractedMut<'a, N, M, T>
where
    T: Extract<N, M>,
{
    /// Wrap the target value's extracted bits in an [`Extracted`] struct.
    #[inline]
    pub const fn wrap(target_value: &'a mut T) -> Self {
        Self(target_value)
    }

    /// Unwrap the extracted value from this wrapper.
    #[inline]
    pub fn value(&self) -> T::Output {
        let &Self(ref target_output) = self;

        target_output.extract()
    }
}

impl<'a, const N: usize, const M: usize, T> ExtractedMut<'a, N, M, T>
where
    T: Extract<N, M>,
{
    /// Access the `K`-th bit of the extracted value through the [`Bit`] wrapper.
    #[inline]
    pub const fn bit<const K: usize>(&'a self) -> Bit<'a, T, K>
    where
        T: Bits<K>,
    {
        let &Self(ref target_output) = self;

        Bit::wrap(target_output)
    }

    /// Access the `K`-th bit of the extracted value through the [`BitMut`] wrapper.
    #[inline]
    pub const fn bit_mut<const K: usize>(&'a mut self) -> BitMut<'a, T, K>
    where
        T: Bits<K>,
    {
        let &mut Self(ref mut target_output) = self;

        BitMut::wrap(target_output)
    }
}
