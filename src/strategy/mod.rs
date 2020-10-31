mod integrator;
pub use integrator::Integrator;
mod shift;
pub use shift::Shift;

use crate::Status;

pub trait Strategy {
	fn status(&self) -> Option<Status>;
	fn update(&self, status: Status) -> Option<Status>;
}
