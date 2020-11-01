//! # Debouncing Signals
//! Use the [`Debounced`] type for this crates functionality.

#![warn(clippy::cargo, clippy::cognitive_complexity)]
#![warn(missing_docs)]
#![no_std]

mod debounced;
mod status;

pub use debounced::Debounced;
pub use status::Status;
pub mod active;
pub mod strategy;

/// Convenience for [`Debounced<_, Integrator, _>`]
pub type DebouncedIntegrator<A, F> = Debounced<A, strategy::Integrator<A>, F>;
//pub type DebouncedShift<A, F> = Debounced<A, strategy::Shift, F>;

mod private {
	pub trait Sealed {}
}
