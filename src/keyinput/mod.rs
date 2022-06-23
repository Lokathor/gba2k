mod keys;
pub use keys::*;

mod keys_low_active;
pub use keys_low_active::*;

use voladdress::*;
pub const KEYINPUT: VolAddress<KeysLowActive, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };

#[inline]
#[must_use]
pub fn get_keys() -> Keys {
  KEYINPUT.read().into()
}
