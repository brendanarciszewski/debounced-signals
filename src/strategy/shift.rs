use crate::{
	active::Active,
	strategy::{NumericType, Strategy},
	Status,
};
use core::{cell::Cell, marker::PhantomData};

/// # Shift Strategy for Debouncing
/// Uses a shift operation (counter) to determine if an input has stabilized
///
/// If the register is [`NumericType::MIN`], the input is stable low. If it is
/// [`NumericType::MAX`] the input is stable high. The shift starts as
/// stable on the [inactive](trait@Active) value.
///
/// Anywhere in-between min and max is unstable (`None`).
///
/// ## Comparison to [`Integrator`](crate::strategy::Integrator)
/// - more coding overhead to customize the unstable zone (requires NewType)
/// - can be more space efficient by (ab)using operator overloading since `MAX`
///   is an associated constant, not a stored value
pub struct Shifter<A, T> {
	reg: Cell<T>,
	_a: PhantomData<A>,
}

impl<A, T> Default for Shifter<A, T>
where
	T: NumericType,
	A: Active,
{
	fn default() -> Self {
		Self {
			reg: Cell::new(if A::ACTIVE_VALUE == Status::Low {
				T::MAX
			} else {
				T::MIN
			}),
			_a: PhantomData,
		}
	}
}

impl<A, T> Strategy for Shifter<A, T>
where
	T: NumericType+core::fmt::Debug,
{
	fn status(&self) -> Option<Status> {
		let reg = self.reg.get();
		if reg <= T::MIN {
			Some(Status::Low)
		} else if reg >= T::MAX {
			Some(Status::High)
		} else {
			None
		}
	}

	fn update(&self, status: Status) -> Option<Status> {
		let reg = self.reg.get();
		match status {
			Status::Low if reg > T::MIN => {
				self.reg.set(reg >> 1);
			},
			Status::High if reg < T::MAX => {
				self.reg.set(reg << 1);
			},
			_ => {},
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
		let i = Shifter::<Low, u8>::default();
		assert_eq!(i.status(), Some(Status::High));
		assert_eq!(i.update(Status::High), Some(Status::High));
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), None);
		assert_eq!(i.update(Status::Low), Some(Status::Low));
		assert_eq!(i.update(Status::Low), Some(Status::Low));
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), None);
		assert_eq!(i.update(Status::High), Some(Status::High));
		assert_eq!(i.update(Status::High), Some(Status::High));
	}

	#[test]
	fn update_high() {
		let i = Shifter::<High, u8>::default();
		assert_eq!(i.status(), Some(Status::Low));
	}
	#[test]
	fn update_low() {
		let i = Shifter::<Low, u8>::default();
		assert_eq!(i.status(), Some(Status::High));
	}
}
