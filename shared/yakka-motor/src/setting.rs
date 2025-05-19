//! Basic motor settings.

use crate::prelude::Throttle;

/// The artificial throttle limits imposed to a motor controller.
///
/// This is interpreted as a limit to the `X %` of the real capacity of the motor.
///
/// This applies in a bidirectional manner, i.e, for a control limit `N`, the aceptable range is `-N% to N%`, inclusively.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct Limit(pub Throttle);

impl Limit {
    /// Wrap a [`Throttle`] value into a [`Limit`].
    #[inline]
    pub const fn wrap(target_value: Throttle) -> Self {
        Self(target_value)
    }
}
