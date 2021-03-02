/// Types that can be used as a direction parameter.
///
/// There are two base directions: [Normal] and [Reverse].
///
/// Below is a table of generic actions `A` and `B` which depends on the direction type.
///
/// | Normal | Reverse |
/// |--------|---------|
/// | A      | B       |
/// | B      | A       |
pub trait Direction {
    const IS_NORMAL: bool;
    const IS_REVERSE: bool = !Self::IS_NORMAL;
}

/// A normal direction.
///
/// Uses as type parameter in components which normalize its work.
///
/// # Example
///
/// ```rust
/// # use core::marker::PhantomData;
/// use vennix_hal::{Direction, Normal};
///
/// struct Component<D = Normal>
/// where
///     D: Direction
/// {
///     /* fields omitted */
///     _pd: PhantomData<D>
/// }
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash)]
pub struct Normal;

impl Direction for Normal {
    const IS_NORMAL: bool = true;
}

/// An reverse direction.
///
/// Uses as type parameter in components which reverses its work.
///
/// # Example
///
/// ```rust
/// # use core::marker::PhantomData;
/// use vennix_hal::{Direction, Reverse, Normal};
///
/// struct Component<D = Normal>
/// where
///     D: Direction
/// {
///     /* fields omitted */
///     _pd: PhantomData<D>
/// }
///
/// impl<D> Component<D>
/// where
///     D: Direction
/// {
///     fn new() -> Self {
///         /* constructor omitted */
///     }
/// }
///
/// fn main() {
///     let component = Component::<Reverse>::new();
/// }
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash)]
pub struct Reverse;

impl Direction for Reverse {
    const IS_NORMAL: bool = false;
}
