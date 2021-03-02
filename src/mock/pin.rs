use core::convert::Infallible;

use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};

use crate::switch::State;

pub struct Pin {
    state: State,
}

impl Pin {
    pub const fn new() -> Self {
        Self {
            state: State::Disabled,
        }
    }

    pub const fn with_state(state: State) -> Self {
        Self { state }
    }
}

impl InputPin for Pin {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.state == State::Enabled)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.state == State::Disabled)
    }
}

impl OutputPin for Pin {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = State::Disabled;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = State::Enabled;
        Ok(())
    }
}

impl StatefulOutputPin for Pin {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_high()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}

impl Default for Pin {
    fn default() -> Self {
        Self::new()
    }
}
