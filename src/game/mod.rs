mod utils;
mod ver_other;
mod ver_x86;

#[cfg(not(target_arch = "x86_64"))]
pub use ver_other::*;
#[cfg(target_arch = "x86_64")]
pub use ver_x86::*;

pub use utils::{Corner, Dimensioned, Neighbor, Player, Reflection, Rotation, Transformation};
