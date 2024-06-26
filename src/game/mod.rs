mod utils;
pub mod ver_1;
pub mod ver_2;
pub mod ver_3;

#[cfg(not(any(alg_ver = "2", alg_ver = "3")))]
pub use ver_1::*;
#[cfg(alg_ver = "2")]
pub use ver_2::*;
#[cfg(alg_ver = "3")]
pub use ver_3::*;

pub use utils::{Corner, Dimensioned, Neighbor, Player, Reflection, Rotation, Transformation};
