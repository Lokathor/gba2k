mod color;
pub use color::*;

mod display_control;
pub use display_control::*;

mod display_status;
pub use display_status::*;

use voladdress::*;
pub const VCOUNT: VolAddress<u8, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0006) };
