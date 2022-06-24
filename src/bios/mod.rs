#![allow(non_snake_case)]

//! The BIOS of the GBA provides some code that's built-in to the device.
//!
//! The BIOS performs a part of the hardware interrupt handling process (see the
//! [`interrupts`](crate::interrupts) module), and it also performs the entire
//! software interrupt handling process.
//!
//! A software interrupt is performed with the `swi` instruction. The important
//! difference compared to calling a function with `bx` or `bl` is that a
//! software interrupt has quite a bit more overhead to call. A standard
//! function call costs ten cycles or less, while a software interrupt costs
//! dozens of cycles.
//!
//! However, the BIOS functions don't take up any IWRAM or ROM space, so if the
//! situation fits you might want to use them just to keep your program smaller.
//!
//! Notably, the [`Halt`] function, and the other BIOS functions based on it,
//! *cannot* be duplicated in user code. They use a special MMIO location that
//! only accepts data when the CPU's program counter is in the BIOS region, so
//! only the BIOS can do it. It's not necessary to ever use `Halt`, but it's
//! friendly on the battery to let the CPU sleep as much as possible if you
//! don't have anything else to do. Even if the user is using an emulator it
//! saves them some energy to let the emulation skip the CPU computation.

// Note(Lokathor): Most all the functions here should ideally have some
// extensive docs on them. To keep things consistent and easy to edit, we're
// going to put each individual function in its own file. For simplicity, each
// sub-module will be named after the `swi` number of the function.

mod x02;
pub use x02::*;

mod x04;
pub use x04::*;

mod x05;
pub use x05::*;

mod x10;
pub use x10::*;

// Note(Lokathor): 0x2A is the highest SWI on the GBA.
