//! Motor layout configuration.

use crate::driver::Motor;

use crate::prelude::Throttle;
use crate::setting::Limit;

/// The throttle parameters for the principal axes of an aircraft.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Axes {
    /// The pitch throttle parameter.
    pitch: Throttle,

    /// The yaw throttle parameter.
    yaw: Throttle,

    /// The roll throttle parameter.
    roll: Throttle,
}

impl Axes {
    /// A zeroed axes control vector.
    pub const ZERO: Self = Self {
        pitch: Throttle::MIN,
        yaw: Throttle::MIN,
        roll: Throttle::MIN,
    };
}

/// The control vector of an aircraft.
///
/// This is the combination of:
///  - The throttle baseline.
///  - The respective throttles for each principal axis: pitch, yaw, and roll.
pub struct ControlVector {
    /// The "baseline" throttle input.
    throttle_baseline: Throttle,

    /// The respective throttle for each respective aircraft axis.
    axes_throttle: Axes,
}

impl ControlVector {
    /// Instantiate a new bare, throttle-only control vector.
    #[inline]
    pub const fn bare(target_throttle: Throttle) -> Self {
        Self {
            throttle_baseline: target_throttle,
            axes_throttle: Axes::ZERO,
        }
    }
}

/// The actuator over an aircraft's physical powerplant.
pub trait Actuator<const N: usize> {
    /// Apply a control vector to all motors under this actuator.
    ///
    /// Yields back the [`Throttle`] control already present in each motor, if any.
    fn control(&mut self, target_vec: ControlVector) -> [Option<Throttle>; N];

    /// Set the throttle limit for all motors under this actuator.
    ///
    /// Yields back the [`Limit`] already present in each motor, if any.
    fn limit(&mut self, target_limit: Limit) -> [Option<Limit>; N];
}

/// A quadcopter motor layout arranged in an `X` shape.
///
/// ```no_run
///     UPPER
///       ↑
///       |
/// (M1)     (M2)
///  CCW       CW
///    \     /
///     \   /
///      \ /
///      / \
///     /   \
///  CW       CCW
/// (M4)     (M3)
///       |
///       ↓
///      LOWER
/// ```
///
/// See also: `https://ardupilot.org/copter/_images/m_01_01_quad_x.svg`
pub struct Quad<M>
where
    M: Motor,
{
    /// The motor located in the upper left quadrant.
    ///
    /// The rotation of this motor is counter-clockwise (CCW).
    upper_left: M,

    /// The motor located in the upper right quadrant.
    ///
    /// The rotation of this motor is clockwise (CW).
    upper_right: M,

    /// The motor located in the lower left quadrant.
    ///
    /// The rotation of this motor is clockwise (CW).
    lower_left: M,

    /// The motor located in the lower right quadrant.
    ///
    /// The rotation of this motor is counter-clockwise (CCW).
    lower_right: M,
}

impl<M> Quad<M>
where
    M: Motor,
{
    /// Create a new quadcopter motor layout from a 4-tuple of its constituent motors.
    ///
    /// The tuple components are named to avoid confusion.
    #[inline]
    pub fn tuple((upper_left, upper_right, lower_left, lower_right): (M, M, M, M)) -> Self {
        Self {
            upper_left,
            upper_right,
            lower_left,
            lower_right,
        }
    }
}

impl<M> Actuator<4> for Quad<M>
where
    M: Motor,
{
    #[inline]
    fn control(&mut self, target_vec: ControlVector) -> [Option<Throttle>; 4] {
        let ControlVector {
            throttle_baseline,
            axes_throttle: Axes { pitch, yaw, roll },
        } = target_vec;

        let &mut Self {
            ref mut upper_left,
            ref mut upper_right,
            ref mut lower_left,
            ref mut lower_right,
        } = self;

        /*
            M1 = T + P + R - Y  (Upper Left, CCW)
            M2 = T + P - R + Y  (Upper Right, CW)
            M3 = T - P - R - Y  (Lower Right, CCW)
            M4 = T - P + R + Y  (Lower Left, CW)
        */

        let t = throttle_baseline.float();
        let (p, y, r) = (pitch.float(), yaw.float(), roll.float());

        let t0 = upper_left.control(Throttle::percent(t + p + r - y));
        let t1 = upper_right.control(Throttle::percent(t + p - r + y));
        let t2 = lower_left.control(Throttle::percent(t - p - r - y));
        let t3 = lower_right.control(Throttle::percent(t + p + r + y));

        [t0, t1, t2, t3]
    }

    #[inline]
    fn limit(&mut self, target_limit: Limit) -> [Option<Limit>; 4] {
        let &mut Self {
            ref mut upper_left,
            ref mut upper_right,
            ref mut lower_left,
            ref mut lower_right,
        } = self;

        let l0 = upper_left.limit(target_limit);
        let l1 = upper_right.limit(target_limit);
        let l2 = lower_left.limit(target_limit);
        let l3 = lower_right.limit(target_limit);

        [l0, l1, l2, l3]
    }
}
