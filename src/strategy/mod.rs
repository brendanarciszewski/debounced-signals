//! # Debouncing Strategies
//!
//! The types in this module define the debouncing strategies available.
//!
//! The [`Strategy`] trait defines to what all strategies must conform. If a
//! custom strategy is desired, implement this trait and use the strategy in
//! a [`Debouncer`](crate::Debounced).

mod integrator;
pub use integrator::Integrator;
//mod shift;
//pub use shift::Shift;

use crate::Status;

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
