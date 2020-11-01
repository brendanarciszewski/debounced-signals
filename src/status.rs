use core::ops::Not;

/// # Input Status
/// A newtype describing if an input is low or high
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
	/// `false` or `0` input
	Low,
	/// `true` or `1` input
	High,
}

impl Not for Status {
	type Output = Status;

	fn not(self) -> Self::Output {
		match self {
			Status::Low => Status::High,
			Status::High => Status::Low,
		}
	}
}

impl From<Status> for bool {
	fn from(val: Status) -> Self {
		if val == Status::High {
			true
		} else {
			false
		}
	}
}

impl From<bool> for Status {
	fn from(val: bool) -> Self {
		if val {
			Status::High
		} else {
			Status::Low
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn inverses() {
		assert_eq!(Status::Low, !Status::High);
		assert_eq!(!Status::Low, Status::High);
	}

	#[test]
	fn into_bool() {
		assert_eq!(bool::from(Status::Low), false);
		assert_eq!(bool::from(Status::High), true);
	}
}
