//! Video Mode 3: A single 240x160 bitmap.

use super::{Color, ColorAddress, VRAM_BASE};

use voladdress::*;

/// A scanline of the Video Mode 3 bitmap.
pub type Mode3Scanline = VolBlock<Color, Safe, Safe, 240>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mode3;

impl Mode3 {
  /// The screen width, 240 pixels.
  pub const WIDTH: usize = 240;

  /// The screen height, 160 pixels.
  pub const HEIGHT: usize = 160;

  #[inline]
  #[must_use]
  pub const fn scanlines(self) -> Mode3Scanlines {
    Mode3Scanlines::new()
  }

  #[inline]
  #[must_use]
  pub const fn pixel_xy(self, x: usize, y: usize) -> ColorAddress {
    assert!(x < Self::WIDTH);
    assert!(y < Self::HEIGHT);
    unsafe { ColorAddress::new(VRAM_BASE + (y * Self::WIDTH) + x) }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct Mode3Scanlines(usize);

impl Default for Mode3Scanlines {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::new()
  }
}

impl Mode3Scanlines {
  const OFFSET: usize = Mode3::WIDTH * size_of!(Color);
  const MAX: usize = VRAM_BASE + (Self::OFFSET * Mode3::HEIGHT);

  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self(VRAM_BASE)
  }

  #[inline]
  #[must_use]
  const fn current_row(&self) -> usize {
    (self.0 - VRAM_BASE) / Mode3Scanlines::OFFSET
  }
  #[inline]
  #[must_use]
  const fn remaining_rows(&self) -> usize {
    160 - self.current_row()
  }
}

impl Iterator for Mode3Scanlines {
  type Item = Mode3Scanline;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0 < Self::MAX {
      let out = Some(unsafe { Mode3Scanline::new(self.0) });
      self.0 += Self::OFFSET;
      out
    } else {
      None
    }
  }

  #[inline]
  #[must_use]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, Some(self.remaining_rows()))
  }

  #[inline]
  #[must_use]
  fn count(self) -> usize {
    self.remaining_rows()
  }

  #[inline]
  #[must_use]
  fn last(self) -> Option<Self::Item> {
    const LAST_LINE: usize =
      VRAM_BASE + (Mode3Scanlines::OFFSET * (Mode3::HEIGHT - 1));
    if self.0 < Self::MAX {
      Some(unsafe { Mode3Scanline::new(LAST_LINE) })
    } else {
      None
    }
  }

  #[inline]
  #[must_use]
  fn nth(&mut self, n: usize) -> Option<Self::Item> {
    if n > Mode3::HEIGHT {
      self.0 = Mode3Scanlines::MAX;
      None
    } else {
      self.0 += Mode3Scanlines::OFFSET * n;
      self.next()
    }
  }

  #[inline]
  #[must_use]
  fn max(self) -> Option<Self::Item> {
    self.last()
  }

  #[inline]
  #[must_use]
  fn min(mut self) -> Option<Self::Item> {
    self.next()
  }
}
