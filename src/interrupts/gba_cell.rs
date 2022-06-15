use core::cell::UnsafeCell;

use super::a32_swp_raw;

#[repr(transparent)]
pub struct GbaCell<T>(UnsafeCell<T>);

unsafe impl<T> Send for GbaCell<T> {}
unsafe impl<T> Sync for GbaCell<T> {}

impl<T> GbaCell<T> {
  /// ## Safety
  /// Types stored in a `GbaCell` should be types that can be read/written with
  /// a single machine instruction. This basically means any 1, 2, or 4 byte
  /// type that's a single field.
  pub const unsafe fn new_unchecked(t: T) -> Self {
    Self(UnsafeCell::new(t))
  }
  pub fn read(&self) -> T {
    unsafe { self.0.get().read_volatile() }
  }
  pub fn write(&self, t: T) {
    unsafe { self.0.get().write_volatile(t) }
  }
}

impl GbaCell<i32> {
  pub const fn new_i32(x: i32) -> Self {
    Self(UnsafeCell::new(x))
  }
  pub fn swap(&self, x: i32) -> i32 {
    unsafe { a32_swp_raw(x as u32, self.0.get().cast()) as i32 }
  }
}

impl GbaCell<u32> {
  pub const fn new_u32(x: u32) -> Self {
    Self(UnsafeCell::new(x))
  }
  pub fn swap(&self, x: u32) -> u32 {
    unsafe { a32_swp_raw(x, self.0.get()) }
  }
}
