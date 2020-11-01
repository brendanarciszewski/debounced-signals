//! # Active Low and Active High Inputs
//! These types encode behaviour about the physical state of an input.
//!
//! For example, considering a two-state button as active when pressed would
//! have to consider if the button is NC or NO, and if the input is normally
//! pulled up or down.
//!
//! Button | Input | Active
//! ------ | ----- | ------
//! NC     | PU    | **High**
//! NC     | PD    | **Low**
//! NO     | PU    | **Low**
//! NO     | PD    | **High**
//!
//! Use [`Low`] and [`High`] to encode this information, which drives the
//! beginning state of the debouncer.
//!
//! For non-buttons, the activity of the input should be the default-state; the
//! state that does not trigger something to occur. However, there is a lot of
//! domain specific information that should drive your choice, which may go
//! against the information here.

use crate::Status;

/// # Active Low Input
/// Used for templates. An instance of this type is never made.
pub struct Low;

/// # Active High Input
/// Used for templates. An instance of this type is never made.
pub struct High;

/// # [`Status`] of an Input
/// This trait can only be implemented for [`Low`] and [`High`], and is used to
/// retreive the corresponding runtime value.
pub trait Active: crate::private::Sealed {
	/// The runtime value where an action occurs if this input is active
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
