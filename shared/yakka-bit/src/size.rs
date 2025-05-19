//! Type-level integer primitive type selector.

use yakka_number::scalar::Scalar;

/// A uninhabited, completely compile-time type selector for the minimum viable integer type for a target bit width.
///
/// As such, imagine this as a type-level function that selects the target type based on the constant generic parameter.
///
/// For instance, imagine a standalone requirement of 2-bits of storage:
///
/// ```no_run
/// <Size as For<2>>::Target
/// ```
///
/// This type will select the minimum viable integer type for 2-bits of storage, which is [`u8`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Size {}

/// A helper trait for the compile-time type selector [`Size`].
pub trait For<const N: usize> {
    /// The selected scalar type.
    type Target: Scalar;
}

macro_rules! selector {
    () => {};
    (
        [$($target_index:literal),* $(,)?] for $target_type:ty
    ) => {
        $(
            impl For<$target_index> for Size {
                type Target = $target_type;
            }
        )+
    };
}

selector!(
    [0, 1, 2, 3, 4, 5, 6, 7, 8] for u8
);

selector!(
    [9, 10, 11, 12, 13, 14, 15, 16] for u16
);

selector!(
    [17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32] for u32
);
