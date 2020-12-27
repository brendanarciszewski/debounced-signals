use super::NumericType;
use super::Shifter;
use crate::{active::Active, DebouncedShifter};
use core::ops::{Shl, Shr};

pub type DebouncedSmall<A, F, const N: u8> = DebouncedShifter<A, ShifterData<N>, F>;

impl<A, F, const N: u8> DebouncedSmall<A, F, N>
where
	A: Active,
	F: Fn() -> bool,
{
	/// Create a new integration-debounced input with no size-overhead
	pub fn with(is_input_high: F) -> Self {
		Self::new(Shifter::default(), is_input_high)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShifterData<const N: u8>(u8);

impl<const N: u8> Shl<u8> for ShifterData<N> {
	type Output = Self;

	fn shl(self, rhs: u8) -> Self::Output {
		Self(self.0 + rhs)
	}
}

impl<const N: u8> Shr<u8> for ShifterData<N> {
	type Output = Self;

	fn shr(self, rhs: u8) -> Self::Output {
		Self(self.0 - rhs)
	}
}

impl<const N: u8> NumericType for ShifterData<N> {
	const MAX: Self = Self(N);
	const MIN: Self = Self(0);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::active::{High, Low};
	use crate::Debounced;

	#[test]
	fn low_is_triggered() {
		let d = DebouncedSmall::<Low, _, 6>::with(|| false);
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
		let sz = size_of_val(&DebouncedSmall::<Low, _, 6>::with(|| false));
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
		let d = DebouncedSmall::<Low, _, 0>::with(|| false);
		assert_eq!(d.try_is_triggered(), Some(true));
		assert_eq!(d.is_triggered_or_unset(), true);
		assert_eq!(d.is_triggered_latest(), true);

		let d = DebouncedSmall::<High, _, 0>::with(|| false);
		assert_eq!(d.try_is_triggered(), Some(false));
		assert_eq!(d.is_triggered_or_unset(), false);
        assert_eq!(d.is_triggered_latest(), false);
        
        let d = DebouncedSmall::<Low, _, 0>::with(|| true);
		assert_eq!(d.try_is_triggered(), Some(true));
		assert_eq!(d.is_triggered_or_unset(), true);
        assert_eq!(d.is_triggered_latest(), true);
        
        let d = DebouncedSmall::<High, _, 0>::with(|| true);
		assert_eq!(d.try_is_triggered(), Some(false));
		assert_eq!(d.is_triggered_or_unset(), false);
		assert_eq!(d.is_triggered_latest(), false);
	}
}
