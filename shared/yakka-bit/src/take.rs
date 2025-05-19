//! Type-encoded bit extration.

use yakka_number::scalar::Scalar;

use crate::state::State;

/// A trait for types who have been built upon a primitive scalar type.
pub trait HasPrimitive {
    /// The primitive scalar type of this type.
    type Primitive: Scalar;

    /// Convert this value into its primitive type.
    fn raw(self) -> Self::Primitive;
}

/// A trait for types whose bits can be manipulated individually.
pub trait Bits<const N: usize>: HasPrimitive + Scalar {
    /// Set the [`State`] of the target bit.
    ///
    /// Yields back the previous state of the bit.
    fn set(&mut self, target_state: State) -> State;

    /// Determine the [`State`] of the target bit.
    fn get(&self) -> State;

    /// Instantiate a new value of this type with the only target bit set.
    fn single() -> Self;

    /// Toggle the state of the target bit.
    ///
    /// Yields back the previous state of the bit.
    #[inline]
    fn toggle(&mut self) -> State {
        let target_state = self.get();

        match target_state {
            State::Set => self.set(State::Cleared),
            State::Cleared => self.set(State::Set),
        }
    }

    /// Copy this value, but with the target bit toggled.
    #[inline]
    fn toggled(&self) -> Self {
        let mut target_copy = *self;

        target_copy.toggle();

        target_copy
    }

    /// Copy this value, but with the target bit cleared.
    #[inline]
    fn cleared(&self) -> Self {
        let mut target_copy = *self;

        target_copy.set(State::Cleared);

        target_copy
    }

    /// Copy this value, but with the target bit set.
    #[inline]
    fn enabled(&self) -> Self {
        let mut target_copy = *self;

        target_copy.set(State::Set);

        target_copy
    }
}

macro_rules! bits {
    () => {};
    (
        [$($target_index:literal),*] for $target_type:ty
    ) => {
        permafrost::embed! {
            impl HasPrimitive for $target_type {
                type Primitive = $target_type;

                #[inline]
                fn raw(self) -> Self::Primitive {
                    self
                }
            }

            $(
                impl Bits<{ $target_index }> for $target_type
                where
                    Self: Scalar,
                {
                    #[inline]
                    fn single() -> Self {
                        const TARGET_MASK: $target_type = 1 << $target_index;

                        TARGET_MASK
                    }

                    #[inline]
                    fn set(&mut self, target_state: State) -> State {
                        const TARGET_MASK: $target_type = 1 << $target_index;

                        let set_state = Bits::<$target_index>::get(self);

                        match target_state {
                            State::Set => {
                                *self = (*self | TARGET_MASK);
                            }

                            State::Cleared => {
                                *self = (*self & !TARGET_MASK);
                            }
                        }

                        set_state
                    }

                    #[inline]
                    fn get(&self) -> State {
                        const TARGET_MASK: $target_type = 1 << $target_index;

                        let target_bit = *self & TARGET_MASK;

                        match target_bit {
                            0 => State::Cleared,
                            _ => State::Set,
                        }
                    }
                }
            )+
        }
    };
}

bits!([0, 1, 2, 3, 4, 5, 6, 7] for u8);

bits!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] for u16);
