use core::{cell::Cell, marker::PhantomData, num::NonZeroU8};

use crate::{
	active::Active,
	strategy::{self, Strategy},
	Status,
};

pub trait StateFunc: Fn() -> bool {}

impl<T> StateFunc for T where T: Fn() -> bool {}

pub struct Debounced<A, S, F> {
	is_bit_set_high: F,
	strategy: S,
	hysteresis: Cell<Status>,
	_a: PhantomData<A>,
}

impl<A, S, F> Debounced<A, S, F>
where
	A: Active,
	F: StateFunc,
{
	pub fn new(strategy: S, is_bit_set_high: F) -> Self {
		Self {
			is_bit_set_high,
			strategy,
			hysteresis: Cell::new(!A::ACTIVE_VALUE),
			_a: PhantomData,
		}
	}
}

impl<A, F> Debounced<A, strategy::Integrator<A>, F>
where
	A: Active,
	F: StateFunc,
{
	pub fn with_integrator(max: NonZeroU8, is_bit_set_high: F) -> Self {
		Self::new(strategy::Integrator::new(max), is_bit_set_high)
	}
}

impl<A, S, F> Debounced<A, S, F>
where
	S: Strategy,
	F: StateFunc,
{
	pub fn try_get(&self) -> Option<Status> {
		let s = self.strategy.update(if (self.is_bit_set_high)() {
			Status::High
		} else {
			Status::Low
		});
		if let Some(s) = s {
			self.hysteresis.set(s);
		}
		s
	}

	pub fn get_blocking(&self) -> Status {
		let mut status;
		loop {
			status = self.try_get();
			if let Some(s) = status {
				return s;
			}
		}
	}

	pub fn get_latest(&self) -> Status {
		self.try_get().unwrap_or_else(|| self.hysteresis.get())
	}
}

impl<A, S, F> Debounced<A, S, F>
where
	A: Active,
	S: Strategy,
	F: StateFunc,
{
	pub fn get_or_unset(&self) -> Status {
		self.try_get().unwrap_or(!A::ACTIVE_VALUE)
	}

	pub fn try_is_triggered(&self) -> Option<bool> {
		self.try_get().map(|s| s == A::ACTIVE_VALUE)
	}

	pub fn is_triggered_or_unset(&self) -> bool {
		self.get_or_unset() == A::ACTIVE_VALUE
	}

	pub fn is_triggered_latest(&self) -> bool {
		self.get_latest() == A::ACTIVE_VALUE
    }

    pub fn is_triggered_blocking(&self) -> bool {
        self.get_blocking() == A::ACTIVE_VALUE
    }
}

#[cfg(test)]
mod tests_integrator {
	use super::*;
	use crate::{
		active::{High, Low},
		strategy::{Integrator, Shift},
	};
	type DbInt<F> = crate::DebouncedIntegrator<Low, F>;
	//type DB_H_Shf<F> = Debounced<High, Shift, F>;

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
}
