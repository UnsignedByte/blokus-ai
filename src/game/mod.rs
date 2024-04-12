mod utils;
mod ver_other;
mod ver_x86;

#[cfg(not(all(target_arch = "x86_64", use_x86)))]
pub use ver_other::*;
#[cfg(all(target_arch = "x86_64", use_x86))]
pub use ver_x86::*;

pub use utils::{Corner, Dimensioned, Neighbor, Player, Reflection, Rotation, Transformation};
