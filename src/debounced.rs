use core::{cell::Cell, marker::PhantomData, num::NonZeroU8};

use crate::{
	active::Active,
	strategy::{self, Strategy},
	Status,
};

/// # Debounces Input
/// For any signal, uses the `strategy` to determine if the bit has settled.
/// This is typically used for buttons.
///
/// Maintains storage of the last settled value as determined by the
/// [`Strategy`]. The first stored value will be
/// [`!Active::ACTIVE_VALUE`](trait@Active).
///
/// Most implementations of debouncing return the last stored value, which
/// corresponds to the [`Self::get_latest`] and [`Self::is_triggered_latest`]
/// functions.
pub struct Debounced<A, S, F> {
	is_input_high: F,
	strategy: S,
	hysteresis: Cell<Status>,
	_a: PhantomData<A>,
}

/// Convenience for [`Debounced<_, Integrator, _>`]
pub type DebouncedIntegrator<A, F> = Debounced<A, strategy::Integrator<A>, F>;
/// Convenience for [`Debounced<_, Shifter, _>`]
pub type DebouncedShifter<A, T, F> = Debounced<A, strategy::Shifter<A, T>, F>;

impl<A, S, F> Debounced<A, S, F>
where
	A: Active,
	F: Fn() -> bool,
{
	/// Creates a new Debounced input using any [`Strategy`]
	pub fn new(strategy: S, is_input_high: F) -> Self {
		Self {
			is_input_high,
			strategy,
			hysteresis: Cell::new(!A::ACTIVE_VALUE),
			_a: PhantomData,
		}
	}
}

impl<A, F> DebouncedIntegrator<A, F>
where
	A: Active,
	F: Fn() -> bool,
{
	/// [Convenience](strategy::Integrator::new) to create a new
	/// integrator-debounced input
	pub fn with_integrator(max: NonZeroU8, is_input_high: F) -> Self {
		Self::new(strategy::Integrator::new(max), is_input_high)
	}
}

impl<A, F> DebouncedShifter<A, u8, F>
where
	A: Active,
	F: Fn() -> bool,
{
	/// [Convenience](strategy::Shifter::default) to create a new
	/// shift-debounced input
	pub fn with_shifter<T>(is_input_high: F) -> DebouncedShifter<A, T, F>
	where
		T: strategy::NumericType,
	{
		Debounced::new(strategy::Shifter::default(), is_input_high)
	}
}

impl<A, S, F> Debounced<A, S, F>
where
	S: Strategy,
	F: Fn() -> bool,
{
	#[inline]
	fn input_status(&self) -> Status {
		if (self.is_input_high)() {
			Status::High
		} else {
			Status::Low
		}
	}

	/// If the `strategy` has not settled on a [`Status`], will not pick one.
	pub fn try_get(&self) -> Option<Status> {
		let s = self.strategy.update(self.input_status());
		if let Some(s) = s {
			self.hysteresis.set(s);
		}
		s
	}

	/// If the `strategy` has not settled on a [`Status`], uses the last settled
	/// value.
	pub fn get_latest(&self) -> Status {
		self.try_get().unwrap_or_else(|| self.hysteresis.get())
	}

	/// Blocks until the `strategy` has settled on a [`Status`].
	pub fn get_blocking(&self) -> Status {
		let mut status;
		loop {
			status = self.try_get();
			if let Some(s) = status {
				return s;
			}
		}
	}
}

impl<A, S, F> Debounced<A, S, F>
where
	A: Active,
	S: Strategy,
	F: Fn() -> bool,
{
	/// If the `strategy` has not settled on a [`Status`], uses the
	/// [inactive](trait@Active) value.
	pub fn get_or_unset(&self) -> Status {
		self.try_get().unwrap_or(!A::ACTIVE_VALUE)
	}

	/// Compares [`Self::try_get`] with the value of an active input
	pub fn try_is_triggered(&self) -> Option<bool> {
		self.try_get().map(|s| s == A::ACTIVE_VALUE)
	}

	/// Compares [`Self::get_latest`] with the value of an active input
	pub fn is_triggered_latest(&self) -> bool {
		self.get_latest() == A::ACTIVE_VALUE
	}

	/// Compares [`Self::get_blocking`] with the value of an active input
	pub fn is_triggered_blocking(&self) -> bool {
		self.get_blocking() == A::ACTIVE_VALUE
	}

	/// Compares [`Self::get_or_unset`] with the value of an active input
	pub fn is_triggered_or_unset(&self) -> bool {
		self.get_or_unset() == A::ACTIVE_VALUE
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::active::{High, Low};
	type DbInt<F> = DebouncedIntegrator<Low, F>;
	type DbShf<T, F> = DebouncedShifter<High, T, F>;

	#[test]
	fn low_is_triggered() {
		let d = DbInt::with_integrator(NonZeroU8::new(6).unwrap(), || false);
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
	fn blocking() {
		let d = DbInt::with_integrator(NonZeroU8::new(6).unwrap(), || false);
		assert_eq!(d.get_blocking(), Status::Low);
		let d = DbInt::with_integrator(NonZeroU8::new(6).unwrap(), || true);
		assert_eq!(d.get_blocking(), Status::High);

		let e = DbInt::with_integrator(NonZeroU8::new(6).unwrap(), || false);
		assert_eq!(e.is_triggered_blocking(), true);
		let e = DbInt::with_integrator(NonZeroU8::new(6).unwrap(), || true);
		assert_eq!(e.is_triggered_blocking(), false);

		let f = Debounced::<High, _, _>::with_integrator(NonZeroU8::new(6).unwrap(), || true);
		assert_eq!(f.get_blocking(), Status::High);
		let f = Debounced::<High, _, _>::with_integrator(NonZeroU8::new(6).unwrap(), || false);
		assert_eq!(f.get_blocking(), Status::Low);

		let g = Debounced::<High, _, _>::with_integrator(NonZeroU8::new(6).unwrap(), || true);
		assert_eq!(g.is_triggered_blocking(), true);
		let g = Debounced::<High, _, _>::with_integrator(NonZeroU8::new(6).unwrap(), || false);
		assert_eq!(g.is_triggered_blocking(), false);
	}

	#[test]
	fn low_status() {
		let bit = Cell::new(false);
		let d = DbInt::with_integrator(NonZeroU8::new(6).unwrap(), || bit.get());
		assert_eq!(d.try_get(), None);
		assert_eq!(d.get_or_unset(), Status::High);
		assert_eq!(d.get_latest(), Status::High);
		assert_eq!(d.try_get(), None);
		assert_eq!(d.get_latest(), Status::High);
		// next reads will all be low
		assert_eq!(d.get_latest(), Status::Low);
		assert_eq!(d.get_latest(), Status::Low);
		assert_eq!(d.get_or_unset(), Status::Low);
		assert_eq!(d.try_get(), Some(Status::Low));

		bit.set(true);
		assert_eq!(d.try_get(), None);
		assert_eq!(d.get_or_unset(), Status::High);
		assert_eq!(d.get_latest(), Status::Low);
		assert_eq!(d.try_get(), None);
		assert_eq!(d.get_latest(), Status::Low);
		// next reads will all be high
		assert_eq!(d.get_latest(), Status::High);
		assert_eq!(d.get_latest(), Status::High);
		assert_eq!(d.get_or_unset(), Status::High);
		assert_eq!(d.try_get(), Some(Status::High));
	}

	#[test]
	fn high_is_triggered() {
		let d = DbShf::with_shifter::<u8>(|| true);
		assert_eq!(d.try_is_triggered(), None);
		assert_eq!(d.is_triggered_or_unset(), false);
		assert_eq!(d.is_triggered_latest(), false);
		assert_eq!(d.try_is_triggered(), None);
		assert_eq!(d.is_triggered_latest(), false);
		assert_eq!(d.is_triggered_latest(), false);
		assert_eq!(d.is_triggered_latest(), true);
		assert_eq!(d.is_triggered_latest(), true);
		assert_eq!(d.is_triggered_or_unset(), true);
		assert_eq!(d.try_is_triggered(), Some(true));
	}
}
