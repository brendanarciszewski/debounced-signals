use crate::Status;

pub struct Low;
pub struct High;
pub trait Active: crate::private::Sealed {
	const ACTIVE_VALUE: Status;
}

impl crate::private::Sealed for Low {}
impl Active for Low {
	const ACTIVE_VALUE: Status = Status::Low;
}
impl crate::private::Sealed for High {}
impl Active for High {
	const ACTIVE_VALUE: Status = Status::High;
}
