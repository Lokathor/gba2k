use core::{cell::UnsafeCell, fmt::Debug};

use crate::{
  keys::{Keys, KeysLowActive},
  video::color::Color,
};

/// A GbaCell holds a `Copy` value that's accessed in a single machine
/// instruction.
///
/// This means that the type must be 1, 2, or 4 bytes, and it must either be a
/// primitive type or a single-field `repr(transparent)` struct over such a
/// type.
///
/// Use of the GbaCell type allows for safe mutable global data that can be
/// accessed by both the main program as well as the interrupt handler.
#[derive(Default)]
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
  /// Makes a new cell.
  pub const fn new(t: T) -> Self
  where
    T: GbaCellSafe,
  {
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
  #[inline]
  pub fn get(&self) -> *mut T {
    self.0.get()
  }
}

pub unsafe trait GbaCellSafe {}

unsafe impl GbaCellSafe for u8 {}
unsafe impl GbaCellSafe for i8 {}

unsafe impl GbaCellSafe for u16 {}
unsafe impl GbaCellSafe for i16 {}
unsafe impl GbaCellSafe for Color {}
unsafe impl GbaCellSafe for Keys {}
unsafe impl GbaCellSafe for KeysLowActive {}

unsafe impl GbaCellSafe for u32 {}
unsafe impl GbaCellSafe for i32 {}
