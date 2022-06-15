pub mod display_control;
pub mod display_status;

use voladdress::*;
pub const VCOUNT: VolAddress<u8, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0006) };
