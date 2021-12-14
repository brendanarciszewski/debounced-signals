//! # Debouncing Signals
//! Use the [`Debounced`] type for this crates functionality.

#![forbid(unsafe_code)]
#![warn(clippy::cargo, clippy::cognitive_complexity)]
#![warn(missing_docs)]
#![no_std]

mod debounced;
mod status;

pub use debounced::{
	Debounced, DebouncedGenericShift, DebouncedIntegrandShift, DebouncedIntegrator,
};
pub use status::Status;
pub mod active;
pub mod strategy;

mod private {
	pub trait Sealed {}
}

/// # Computes the samples
///
/// Computes the number of samples which have occurred at the `sample_freq` (the
/// rate at which the input is probed in Hz) in the duration of `hold_time_ms`
/// (in ms). If a partial sample will have occurred, rounds up.
///
/// The purpose of this function is to determine the number of samples to wait
/// before the [`Integrator`](struct@strategy::Integrator) can settle on an
/// output.
///
/// As if computing `max = sampling_freq * min_hold_time`.
#[inline]
pub const fn samples(sample_freq: usize, hold_time_ms: usize) -> usize {
	let samples_in_1k_secs = sample_freq * hold_time_ms;
	// ceiling division
	if samples_in_1k_secs > 0 {
		1 + (samples_in_1k_secs - 1) / 1000
	} else {
		0
	}
}

#[cfg(test)]
mod test_samples {
	use super::*;

	#[test]
	fn samples_small() {
		const SMPL: u8 = samples(1000, 5) as u8;
		assert_eq!(SMPL, 5u8);
	}

	#[test]
	fn samples_irregular() {
		fn naive_sample_impl(freq: usize, hold_time_ms: usize) -> usize {
			freq * hold_time_ms / 1000
		}

		assert_eq!(naive_sample_impl(250, 3), 0);
		assert_eq!(samples(250, 3), 1);

		assert_eq!(naive_sample_impl(250, 4), 1);
		assert_eq!(samples(250, 4), 1);

		assert_eq!(naive_sample_impl(250, 5), 1);
		assert_eq!(samples(250, 5), 2);
	}
}
