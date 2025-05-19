//! This module defines the [`Dimension`] trait and all physical dimensions.



mod private {
    /// A sealed trait to prevent extraneous [`Dimension`]s.
    ///
    /// [`Dimension`]: super::Dimension
    pub trait Sealed {}
}

/// A dimension of a physical unit.
pub trait Dimension: private::Sealed {}

/// A macro to define a [`Dimension`] type.
macro_rules! dimension {
    () => {};
    (
        $(
          #[$dim_meta:meta]
        )*
        $dim_vis:vis $dim_name:ident
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]

        $(
            #[$dim_meta]
        )*
        $dim_vis enum $dim_name {}

        impl private::Sealed for $dim_name {}

        impl Dimension for $dim_name {}
    };
}

dimension!(
    /// The dimension of length.
    pub Length
);

dimension!(
    /// The dimension of mass.
    pub Mass
);

dimension!(
    /// The dimension of time.
    pub Time
);

dimension!(
    /// The dimension of electric current.
    pub Current
);

dimension!(
    /// The dimension of temperature.
    pub Temperature
);

dimension!(
    /// The dimension of amount of substance.
    pub Substance
);

dimension!(
    /// The dimension of luminous intensity.
    pub Luminance
);
