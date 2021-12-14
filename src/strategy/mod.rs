//! # Debouncing Strategies
//!
//! The types in this module define the debouncing strategies available.
//!
//! The [`Strategy`] trait defines to what all strategies must conform. If a
//! custom strategy is desired, implement this trait and use the strategy in
//! a [`Debouncer`](crate::Debounced).

use crate::Status;
use core::ops::{Shl, Shr};

mod integrator;
pub use integrator::Integrator;
mod shift;
pub use shift::Shifter;
mod shift_generic;
pub use shift_generic::DebouncedSmall;

/// # Defining the Debouncing Algorithm
/// The strategy needs to do everything to debounce the input, but it should not
/// store the most recent state.
pub trait Strategy {
	/// The current status of the debouncing.
	///
	/// If the debouncer isn't confident that the input is stable, returns
	/// `None`.
	///
	/// If the debouncer thinks the input is stable, returns `Some(...)` of the
	/// stable value.
	fn status(&self) -> Option<Status>;

	/// Updates the debouncer algorithm with the latest `status`.
	///
	/// Returns [`Strategy::status`].
	fn update(&self, status: Status) -> Option<Status>;
}

/// # Types Which Are Like Integers
///
/// These can `<<`, `>>`, [`==`, `>`, `<`](PartialOrd), and have a `MAX` and
/// `MIN`, so that Shift Register Debouncing is possible.
///
/// Although the [`MAX`](NumericType::MAX) and [`MIN`](NumericType::MIN) don't
/// have to be physically the largest value the type can store, `MAX > MIN` must
/// hold.
///
/// [`Shl`] must increase the value (towards the max) and [`Shr`] must decrease the
/// value and (towards the min).
pub trait NumericType: Shl<u8, Output = Self> + Shr<u8, Output = Self> + Copy + PartialOrd {
	/// The type's maximum value (no value can be greater)
	const MAX: Self;
	/// The type's minimum value (no value can be smaller)
	const MIN: Self;
}

macro_rules! impl_numeric_type {
    ($($type:ty, $max:literal)+) => {
        $(
            impl NumericType for $type {
                const MAX: Self = 1 << $max;
                const MIN: Self = 1;
            }
        )+
    };
}

impl_numeric_type!(u8, 7 u16, 15 u32, 31 u64, 63);
