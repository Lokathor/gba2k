use core::{cell::UnsafeCell, fmt::Debug};

use crate::{keys::KeyInput, video::Color};

/// A GbaCell holds a value that's accessed in a single machine instruction.
///
/// Use of the GbaCell type allows for safe mutable global data that can be
/// accessed by both the main program as well as the interrupt handler.
#[derive(Default)]
#[repr(transparent)]
pub struct GbaCell<T: GbaCellSafe>(UnsafeCell<T>);

impl<T> Debug for GbaCell<T>
where
  T: GbaCellSafe + Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = self.read();
    Debug::fmt(&s, f)
  }
}

unsafe impl<T: GbaCellSafe> Send for GbaCell<T> {}
unsafe impl<T: GbaCellSafe> Sync for GbaCell<T> {}

impl<T: GbaCellSafe> GbaCell<T> {
  /// Makes a new cell.
  #[inline]
  #[must_use]
  pub const fn new(t: T) -> Self {
    Self(UnsafeCell::new(t))
  }
  /// Reads the current value.
  #[inline]
  #[must_use]
  pub fn read(&self) -> T {
    unsafe { self.0.get().read_volatile() }
  }
  /// Writes a new value.
  #[inline]
  pub fn write(&self, t: T) {
    unsafe { self.0.get().write_volatile(t) }
  }
  /// Gets the raw pointer to the value.
  ///
  /// ## Safety
  /// A `GbaCell` is an [UnsafeCell] internally, and so this raw pointer must
  /// follow all the usuall "pointer into an UnsafeCell" rules.
  #[inline]
  #[must_use]
  pub fn get(&self) -> *mut T {
    self.0.get()
  }
}

/// Marker trait for all types that will safely work with a [GbaCell].
///
/// ## Safety
/// The type must be 1, 2, or 4 bytes that's accessed in a single machine
/// instruction. This mostly means integers or `repr(transparent)` wrapper
/// structs over an integer.
pub unsafe trait GbaCellSafe: Copy {}

unsafe impl GbaCellSafe for u8 {}
unsafe impl GbaCellSafe for i8 {}

unsafe impl GbaCellSafe for u16 {}
unsafe impl GbaCellSafe for i16 {}
unsafe impl GbaCellSafe for Color {}
unsafe impl GbaCellSafe for KeyInput {}

unsafe impl GbaCellSafe for u32 {}
unsafe impl GbaCellSafe for i32 {}

// Note(Lokathor): All `Option<fn>` types are GbaCellSafe, but I only want to
// type out so much at once. Feel free to add more impls here any time you need
// some other `fn` type to be usable.
unsafe impl GbaCellSafe for Option<extern "C" fn()> {}
unsafe impl<A> GbaCellSafe for Option<extern "C" fn(A)> {}
unsafe impl<A, B> GbaCellSafe for Option<extern "C" fn(A, B)> {}
unsafe impl<A, B, C> GbaCellSafe for Option<extern "C" fn(A, B, C)> {}
unsafe impl GbaCellSafe for Option<fn()> {}
unsafe impl<A> GbaCellSafe for Option<fn(A)> {}
unsafe impl<A, B> GbaCellSafe for Option<fn(A, B)> {}
unsafe impl<A, B, C> GbaCellSafe for Option<fn(A, B, C)> {}
