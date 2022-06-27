//! Module for utilizing the GBA's screen.
//!
//! ## Video Modes
//!
//! The GBA has 6 video modes. They're just referred to by their index, 0
//! through 5.
//!
//! Currently the crate only provides direct support for Mode 3. Support for
//! other modes is planned.

use voladdress::*;

pub mod mode3;

mod color;
pub use color::*;

mod display_control;
pub use display_control::*;

mod display_status;
pub use display_status::*;

const BG_PALETTE_BASE: usize = 0x0500_0000;
const OBJ_PALETTE_BASE: usize = 0x0500_0200;
const VRAM_BASE: usize = 0x0600_0000;

/// Returns the current scanline that the PPU is drawing (0 through 227).
pub const VCOUNT: VolAddress<u8, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0006) };

/// The color of the screen's "backdrop".
///
/// This is the color that's drawn when no other layer or object has a
/// non-transparent pixel in that location.
pub const BACKDROP_COLOR: VolAddress<Color, Safe, Safe> =
  unsafe { VolAddress::new(BG_PALETTE_BASE) };

/// The background palette entries.
///
/// This is the 8-bits-per-pixel view of the palette, with 256 entries. If you
/// want to access the background palette by individual palbanks use the
/// [bg_palbank] function.
///
/// Remember that a palette index of 0 always means "transparency". The 0th
/// entry of this palette is the same location as the backdrop color.
pub const BG_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(BG_PALETTE_BASE) };

/// The object palette entries.
///
/// This is the 8-bits-per-pixel view of the palette, with 256 entries. If you
/// want to access the background palette by individual palbanks use the
/// [obj_palbank] function.
///
/// Remember that a palette index of 0 always means "transparency". The 0th
/// color of the palette is not used for anything.
pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(OBJ_PALETTE_BASE) };

/// Alias for an individual palbank (16 color group) within the palette.
///
/// Remember that a palette index of 0 always means "transparency", so each
/// palbank is only 15 usable colors (indexes 1-15).
pub type PalBank = VolBlock<Color, Safe, Safe, 16>;

/// Gets palbank `x` of the background palette.
///
/// ## Panics
/// * `x` must be less than 16.
#[inline]
#[must_use]
pub const fn bg_palbank(x: usize) -> PalBank {
  assert!(x < 16);
  unsafe { VolBlock::new(BG_PALETTE_BASE + x * size_of!([Color; 16])) }
}

/// Gets palbank `x` of the object palette.
///
/// ## Panics
/// * `x` must be less than 16.
#[inline]
#[must_use]
pub const fn obj_palbank(x: usize) -> PalBank {
  assert!(x < 16);
  unsafe { VolBlock::new(OBJ_PALETTE_BASE + x * size_of!([Color; 16])) }
}
