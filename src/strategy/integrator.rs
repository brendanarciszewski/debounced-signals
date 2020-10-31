use crate::{active::Active, strategy::Strategy, Status};
use core::{cell::Cell, marker::PhantomData, num::NonZeroU8};

pub struct Integrator<A> {
	integrator: Cell<u8>,
	max: NonZeroU8,
	_a: PhantomData<A>,
}

impl<A: Active> Integrator<A> {
	pub fn new(max: NonZeroU8) -> Self {
		Self {
			integrator: Cell::new(if A::ACTIVE_VALUE == Status::Low {
				max.get()
			} else {
				0
			}),
			max,
			_a: PhantomData,
		}
	}
}

impl<A> Strategy for Integrator<A> {
	fn status(&self) -> Option<Status> {
		let i = self.integrator.get();
		if i == 0 {
			Some(Status::Low)
		} else if i >= self.max.get() {
			Some(Status::High)
		} else {
			None
		}
	}

	fn update(&self, status: Status) -> Option<Status> {
		let i = self.integrator.get();
		use Status::*;
		match (status, i >= self.max.get()) {
			(Low, _) => {
				self.integrator.set(i.saturating_sub(1));
			}
			(High, false) => {
				self.integrator.set(i + 1);
			}
			(High, true) => {}
		}
		self.status()
	}
}
