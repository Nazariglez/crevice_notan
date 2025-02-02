//! Defines traits and types for working with data adhering to GLSL's `std140`
//! layout specification.

mod dynamic_uniform;
mod primitives;
mod sizer;
mod traits;
#[cfg(feature = "std")]
mod writer;

pub use crate::bool::Bool;

pub use self::dynamic_uniform::*;
pub use self::primitives::*;
pub use self::sizer::*;
pub use self::traits::*;
#[cfg(feature = "std")]
pub use self::writer::*;

pub use crevice_notan_derive::AsStd140;
