#![warn(missing_docs)]

//! Allows reading the GBA's key inputs.
//!
//! The GBA has:
//! * Two primary buttons (A and B)
//! * Two secondary buttons (Start and Select)
//! * Two shoulder buttons (L and R)
//! * A 4-way directional pad
//!
//! The [`KEYINPUT`] value will update every single CPU cycle. Minor variations
//! in pressure can make a button seem to "bounce" up and down quite rapidly.
//! Because of this, you should usually read the key state just once per frame
//! (usually at v-blank) and then use that data for the entire frame. Otherwise,
//! the user can get inconsistent behavior when an early part of the frame's
//! computation thinks a button is pressed while later on in the same frame it's
//! released.

mod key_input;
pub use key_input::*;

mod key_control;
pub use key_control::*;
