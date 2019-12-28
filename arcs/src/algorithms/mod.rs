//! Common algorithms.

mod bounding_box;
mod closest_point;
mod length;
mod translate;
mod approximate;

pub use bounding_box::Bounded;
pub use closest_point::{Closest, ClosestPoint};
pub use length::Length;
pub use translate::Translate;
pub use approximate::Approximate;