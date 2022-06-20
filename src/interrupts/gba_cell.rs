use core::{cell::UnsafeCell, fmt::Debug};

use crate::{keys::Keys, rt0_rom_swp, rt0_rom_swpb};

/// A GbaCell holds a `Copy` value that's accessed in a single machine
/// instruction.
///
/// This means that the type must be 1, 2, or 4 bytes, and it must either be a
/// primitive type or a single-field `repr(transparent)` struct over such a
/// type.
///
/// Use of the GbaCell type allows for safe mutable global data that can be
/// accessed by both the main program as well as the interrupt handler.
#[repr(transparent)]
pub struct GbaCell<T: Copy>(UnsafeCell<T>);

impl<T> Debug for GbaCell<T>
where
  T: Copy + Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = self.read();
    Debug::fmt(&s, f)
  }
}

unsafe impl<T: Copy> Send for GbaCell<T> {}
unsafe impl<T: Copy> Sync for GbaCell<T> {}

impl<T: Copy> GbaCell<T> {
  /// ## Safety
  /// As per the type docs.
  #[inline]
  #[must_use]
  pub const unsafe fn new_unchecked(t: T) -> Self {
    Self(UnsafeCell::new(t))
  }
  #[inline]
  #[must_use]
  pub fn read(&self) -> T {
    unsafe { self.0.get().read_volatile() }
  }
  #[inline]
  pub fn write(&self, t: T) {
    unsafe { self.0.get().write_volatile(t) }
  }
}

impl GbaCell<i32> {
  #[inline]
  #[must_use]
  pub const fn new_i32(x: i32) -> Self {
    Self(UnsafeCell::new(x))
  }
  #[inline]
  pub fn swap(&self, x: i32) -> i32 {
    unsafe { rt0_rom_swp(x as u32, self.0.get().cast()) as i32 }
  }
}

impl GbaCell<u32> {
  #[inline]
  #[must_use]
  pub const fn new_u32(x: u32) -> Self {
    Self(UnsafeCell::new(x))
  }
  #[inline]
  pub fn swap(&self, x: u32) -> u32 {
    unsafe { rt0_rom_swp(x, self.0.get()) }
  }
}

impl GbaCell<Keys> {
  #[inline]
  #[must_use]
  pub const fn new_keys(x: Keys) -> Self {
    Self(UnsafeCell::new(x))
  }
}

impl GbaCell<i8> {
  #[inline]
  #[must_use]
  pub const fn new_i8(x: i8) -> Self {
    Self(UnsafeCell::new(x))
  }
  #[inline]
  pub fn swap(&self, x: i8) -> i8 {
    unsafe { rt0_rom_swpb(x as u8, self.0.get().cast()) as i8 }
  }
}

impl GbaCell<u8> {
  #[inline]
  #[must_use]
  pub const fn new_u8(x: u8) -> Self {
    Self(UnsafeCell::new(x))
  }
  #[inline]
  pub fn swap(&self, x: u8) -> u8 {
    unsafe { rt0_rom_swpb(x, self.0.get()) }
  }
}
