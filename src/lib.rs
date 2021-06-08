//! An experimental extension over [embedded_hal](https://docs.rs/embedded-hal) crate.
//! Currently work in progress.

#![no_std]

/// Directions representation.
pub mod direction;

pub mod switch;

pub mod time;

/// A `hal-x` prelude.
pub mod prelude;

pub mod button;

pub mod mock {
    pub use pin::Pin;

    mod pin;
}

pub mod tick;
