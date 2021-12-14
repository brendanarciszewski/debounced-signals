use super::{NumericType, Shifter};
use core::ops::{Shl, Shr};

/// A Strategy
pub type IntegrandShifter<const N: u8> = Shifter<Integrand<N>>;

/// Implements an Integrotor for use in the Shifter strategy
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Integrand<const N: u8>(u8);

impl<const N: u8> Shl<u8> for Integrand<N> {
	type Output = Self;

	fn shl(self, rhs: u8) -> Self::Output {
		Self(self.0.saturating_add(rhs))
	}
}

impl<const N: u8> Shr<u8> for Integrand<N> {
	type Output = Self;

	fn shr(self, rhs: u8) -> Self::Output {
		Self(self.0.saturating_sub(rhs))
	}
}

impl<const N: u8> NumericType for Integrand<N> {
	const MAX: Self = Self(N);
	const MIN: Self = Self(0);
}

#[cfg(test)]
mod tests {
	use crate::active::{High, Low};
	use crate::Debounced;
	use crate::DebouncedIntegrandShift as DbShfInt;

	#[test]
	fn low_is_triggered() {
		let d = DbShfInt::<Low, _, 6>::with(|| false);
		assert_eq!(d.try_is_triggered(), None);
		assert_eq!(d.is_triggered_or_unset(), false);
		assert_eq!(d.is_triggered_latest(), false);
		assert_eq!(d.try_is_triggered(), None);
		assert_eq!(d.is_triggered_latest(), false);
		assert_eq!(d.is_triggered_latest(), true);
		assert_eq!(d.is_triggered_latest(), true);
		assert_eq!(d.is_triggered_or_unset(), true);
		assert_eq!(d.try_is_triggered(), Some(true));
	}

	#[test]
	fn size() {
		use core::mem::size_of_val;
		use core::num::NonZeroU8;
		let sz = size_of_val(&DbShfInt::<Low, _, 6>::with(|| false));
		assert_eq!(
			sz,
			size_of_val(&Debounced::<Low, _, _>::with_shifter::<u8>(|| false))
		);
		assert!(
			sz < size_of_val(&Debounced::<Low, _, _>::with_integrator(
				NonZeroU8::new(6).unwrap(),
				|| false
			))
		);
		assert_eq!(sz, 2);
	}

	#[test]
	fn zero() {
		let d = DbShfInt::<Low, _, 0>::with(|| false);
		assert_eq!(d.try_is_triggered(), Some(true));
		assert_eq!(d.is_triggered_or_unset(), true);
		assert_eq!(d.is_triggered_latest(), true);

		let d = DbShfInt::<High, _, 0>::with(|| false);
		assert_eq!(d.try_is_triggered(), Some(false));
		assert_eq!(d.is_triggered_or_unset(), false);
		assert_eq!(d.is_triggered_latest(), false);

		let d = DbShfInt::<Low, _, 0>::with(|| true);
		assert_eq!(d.try_is_triggered(), Some(true));
		assert_eq!(d.is_triggered_or_unset(), true);
		assert_eq!(d.is_triggered_latest(), true);

		let d = DbShfInt::<High, _, 0>::with(|| true);
		assert_eq!(d.try_is_triggered(), Some(false));
		assert_eq!(d.is_triggered_or_unset(), false);
		assert_eq!(d.is_triggered_latest(), false);
	}
}
