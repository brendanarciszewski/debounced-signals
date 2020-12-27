use crate::{active::Active, strategy::Strategy, Status};
use core::{cell::Cell, num::NonZeroU8};

/// # Integrating Strategy for Debouncing
/// Uses an integrator (counter) to determine if an input has stabilized
///
/// If the integrator is minimum (0), the input is stable low. If it is `max`,
/// the input is stable high. The integrator starts as stable on the
/// [inactive](trait@Active) value.
///
/// Anywhere in-between min and max is unstable (`None`).
pub struct Integrator {
	integrator: Cell<u8>,
	max: NonZeroU8,
}

impl Integrator {
	/// Create a new Integrator
	///
	/// You will likely want to use [`samples`](fn@crate::samples) to compute
	/// the distance (number of steps) between high and low inputs.
	///
	/// In other words, the minimum number of times
	/// [`update`](Integrator::update) needs to be called to toggle the
	/// integrator's output is the `distance`.
	pub fn new<A: Active>(distance: NonZeroU8) -> Self {
		Self {
			integrator: Cell::new(if A::ACTIVE_VALUE == Status::Low {
				distance.get()
			} else {
				0
			}),
			max: distance,
		}
	}
}

impl Strategy for Integrator {
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::active::{High, Low};

	#[test]
	fn update_progress() {
		let i = Integrator::new::<Low>(NonZeroU8::new(3).unwrap());
		assert_eq!(i.status(), Some(Status::High));
		assert_eq!(i.update(Status::High), Some(Status::High));
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), Some(Status::Low));
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), Some(Status::High));
	}

	#[test]
	fn update_high() {
		let i = Integrator::new::<High>(NonZeroU8::new(3).unwrap());
		assert_eq!(i.status(), Some(Status::Low));
	}
	#[test]
	fn update_low() {
		let i = Integrator::new::<Low>(NonZeroU8::new(3).unwrap());
		assert_eq!(i.status(), Some(Status::High));
	}
}
