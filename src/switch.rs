use core::{convert::Infallible, marker::PhantomData};

use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::direction::{Direction, Normal, Reverse};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum State {
    Disabled,
    Enabled,
}

impl Default for State {
    fn default() -> Self {
        Self::Disabled
    }
}

impl From<bool> for State {
    fn from(flag: bool) -> Self {
        if flag {
            State::Enabled
        } else {
            State::Disabled
        }
    }
}

/// A newtype over any type which implements [OutputPin] or [InputPin].
///
/// This type provides application-level interface to control your peripheries. It is suggested
/// to use [Switch::enable] ([Switch::try_enable] for fallible interfaces) and
/// [Switch::disable] ([Switch::try_disable] for fallible interfaces) methods instead
/// of hardware specific [OutputPin::set_high] and [OutputPin::set_low]. The important feature is
/// a [Direction] choice.
///
/// Below is a table of how directions work.
///
/// | Methods \ Directions | Normal     | Reverse     |
/// |----------------------|------------|------------|
/// | `{try_}disable`      | `set_low`  | `set_high` |
/// | `{try_}enable`       | `set_high` | `set_low`  |
#[derive(Clone, Debug)]
pub struct Switch<P, D = Normal> {
    inner: P,
    _pd: PhantomData<D>,
}

impl<P, D> Switch<P, D> {
    /// Creates a new instance. By default, switch direction is [Normal].
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::Switch;
    ///
    /// let switch = Switch::new(pin);
    /// ```
    ///
    /// You may [Reverse] the direction with manual specification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::{Reverse, Switch};
    ///
    /// let switch = Switch::<_, Reverse>::new(pin);
    /// ```
    pub const fn new(pin: P) -> Self {
        Self {
            inner: pin,
            _pd: PhantomData,
        }
    }

    /// Consumes this switch and returns the inner pin.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::Switch;
    ///
    /// let mut switch = Switch::new(pin);
    /// switch.enable();
    ///
    /// let mut pin = switch.into_inner();
    /// pin.set_low().unwrap();
    /// ```
    pub fn into_inner(self) -> P {
        self.inner
    }
}

impl<P, D> Switch<P, D>
where
    D: Direction,
{
    pub fn normalize(self) -> Option<Switch<P, Normal>> {
        if D::IS_NORMAL {
            Some(Switch::new(self.inner))
        } else {
            None
        }
    }

    pub fn reverberate(self) -> Option<Switch<P, Reverse>> {
        if D::IS_NORMAL {
            None
        } else {
            Some(Switch::new(self.inner))
        }
    }
}

impl<P, D> Switch<P, D>
where
    P: OutputPin,
    D: Direction,
{
    pub fn try_disable(&mut self) -> Result<(), <P as OutputPin>::Error> {
        if D::IS_NORMAL {
            self.inner.set_low()
        } else {
            self.inner.set_high()
        }
    }

    pub fn try_enable(&mut self) -> Result<(), <P as OutputPin>::Error> {
        if D::IS_NORMAL {
            self.inner.set_high()
        } else {
            self.inner.set_low()
        }
    }

    pub fn try_set_state(&mut self, state: State) -> Result<(), <P as OutputPin>::Error> {
        match state {
            State::Disabled => self.try_disable(),
            State::Enabled => self.try_enable(),
        }
    }
}

impl<P, D> Switch<P, D>
where
    P: OutputPin<Error = Infallible>,
    D: Direction,
{
    pub fn disable(&mut self) {
        if let Err(e) = self.try_disable() {
            match e {}
        }
    }

    pub fn enable(&mut self) {
        if let Err(e) = self.try_enable() {
            match e {}
        }
    }

    pub fn set_state(&mut self, state: State) {
        match state {
            State::Disabled => self.disable(),
            State::Enabled => self.enable(),
        }
    }
}

impl<P, D> Switch<P, D>
where
    P: InputPin,
    D: Direction,
{
    pub fn try_check_is_disabled(&self) -> Result<bool, <P as InputPin>::Error> {
        if D::IS_NORMAL {
            self.inner.is_low()
        } else {
            self.inner.is_high()
        }
    }

    pub fn try_check_is_enabled(&self) -> Result<bool, <P as InputPin>::Error> {
        if D::IS_NORMAL {
            self.inner.is_high()
        } else {
            self.inner.is_low()
        }
    }

    pub fn try_read_state(&self) -> Result<State, <P as InputPin>::Error> {
        self.try_check_is_enabled().map(Into::into)
    }
}

impl<P, D> Switch<P, D>
where
    P: InputPin<Error = Infallible>,
    D: Direction,
{
    pub fn is_disabled(&self) -> bool {
        match self.try_check_is_disabled() {
            Ok(result) => result,
            Err(e) => match e {},
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self.try_check_is_enabled() {
            Ok(result) => result,
            Err(e) => match e {},
        }
    }

    pub fn read_state(&self) -> State {
        self.is_enabled().into()
    }
}

impl<P> From<Switch<P, Normal>> for Switch<P, Reverse> {
    /// Transforms switch with [Normal] direction into [Reverse].
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::{Reverse, Switch};
    ///
    /// let normal_switch = Switch::new(pin);
    /// let revers_switch: Switch<_, Reverse> = From::from(normal_switch);
    /// ```
    fn from(switch: Switch<P, Normal>) -> Self {
        Switch::new(switch.inner)
    }
}

impl<P> From<Switch<P, Reverse>> for Switch<P, Normal> {
    /// Transforms switch with [Reverse] direction into [Normal].
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::{Reverse, Normal, Switch};
    ///
    /// let revers_switch = Switch::<_, Reverse>::new(pin);
    /// let normal_switch: Switch<_, Normal> = From::from(revers_switch);
    /// ```
    fn from(switch: Switch<P, Reverse>) -> Self {
        Switch::new(switch.inner)
    }
}

impl<P> Switch<P, Normal> {
    /// Transforms switch with [Normal] direction into [Reverse].
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::{Reverse, Switch};
    ///
    /// let normal_switch = Switch::new(pin);
    /// let revers_switch = normal_switch.into_revers();
    /// ```
    pub fn into_revers(self) -> Switch<P, Reverse> {
        Into::into(self)
    }
}

impl<P> Switch<P, Reverse> {
    /// Transforms switch with [Reverse] direction into [Normal].
    ///
    /// # Example
    ///
    /// ```rust
    /// use vennix_hal::{Reverse, Normal, Switch};
    ///
    /// let revers_switch = Switch::<_, Reverse>::new(pin);
    /// let normal_switch = revers_switch.into_normal();
    /// ```
    pub fn into_normal(self) -> Switch<P, Normal> {
        Into::into(self)
    }
}

/// Provides the conversion of any pin into switch.
pub trait IntoSwitch {
    /// Conversion any pin to a switch.
    ///
    /// # Example
    ///
    /// ```rust
    /// # struct AbstractPin;
    /// use vennix_hal::Normal;
    ///
    /// let pin = AbstractPin::new();
    /// let mut switch = pin.into_switch::<Normal>();
    /// switch.enable();
    /// ```
    fn into_switch<D>(self) -> Switch<Self, D>
    where
        Self: Sized,
        D: Direction;
}

impl<P> IntoSwitch for P {
    fn into_switch<D>(self) -> Switch<Self, D> {
        Switch::<P, D>::new(self)
    }
}
