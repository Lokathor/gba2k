#![allow(non_snake_case)]

// Note(Lokathor): Most all the functions here should ideally have some
// extensive docs on them. To keep things easy to edit, we're going to put each
// individual function in its own file. The sub-modules will each be named after
// the `swi` number of the function.

mod x02;
pub use x02::*;

mod x04;
pub use x04::*;

mod x05;
pub use x05::*;

mod x10;
pub use x10::*;

// Note(Lokathor): 0x2A is the highest SWI on the GBA.
