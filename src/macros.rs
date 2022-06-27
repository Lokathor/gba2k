/// Inline assembly to read the stack pointer value.
///
/// The stack pointer doesn't usually move by much on the GBA.
#[macro_export]
macro_rules! read_sp {
  () => {{
    let sp_output: u32;
    unsafe {
      // * T32: `mov` between high and low registers doesn't set flags
      // * A32: `mov` without `s` on the end doesn't set flags.
      core::arch::asm! {
        "mov {sp_val}, sp",
        sp_val = out(reg) sp_output,
        options(nomem, nostack, preserves_flags)
      }
    }
    sp_output
  }};
}

/// Inline assembly to read the link register value.
///
/// This is the "return address" of the current function.
#[macro_export]
macro_rules! read_lr {
  () => {{
    let lr_output: u32;
    unsafe {
      // * T32: `mov` between high and low registers doesn't set flags
      // * A32: `mov` without `s` on the end doesn't set flags.
      core::arch::asm! {
        "mov {lr_val}, lr",
        lr_val = out(reg) lr_output,
        options(nomem, nostack, preserves_flags)
      }
    }
    lr_output
  }};
}

/// Inline assembly to read the program counter value.
///
/// Because of the CPU's pipeline, the program counter will hold the address of
/// the instruction that's *two* instructions ahead of the current instruction.
#[macro_export]
macro_rules! read_pc {
  () => {{
    let pc_output: u32;
    unsafe {
      // * T32: `mov` between high and low registers doesn't set flags
      // * A32: `mov` without `s` on the end doesn't set flags.
      core::arch::asm! {
        "mov {pc_val}, pc",
        pc_val = out(reg) pc_output,
        options(nomem, nostack, preserves_flags)
      }
    }
    pc_output
  }};
}

macro_rules! pub_const_fn_new {
  () => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn new() -> Self {
      Self(0)
    }
  };
}

macro_rules! unsafe_u16_enum_field {
  ($low:literal - $high:literal: $enum_ty:ty, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get_name(self) -> $enum_ty {
      unsafe {
        core::mem::transmute(crate::bit_utils::u16_get_region::<$low, $high>(
          self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with_name(self, val: $enum_ty) -> Self {
      Self(crate::bit_utils::u16_with_region::<$low, $high>(self.0, val as u16))
    }
  };
}

macro_rules! u16_val_field {
  ($low:literal - $high:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get_name(self) -> u16 {
      crate::bit_utils::u16_get_value::<$low, $high>(self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with_name(self, val: u16) -> Self {
      Self(crate::bit_utils::u16_with_value::<$low, $high>(self.0, val))
    }
  };
}

macro_rules! u32_val_field {
  ($low:literal - $high:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get_name(self) -> u32 {
      crate::bit_utils::u32_get_value::<$low, $high>(self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with_name(self, val: u32) -> Self {
      Self(crate::bit_utils::u32_with_value::<$low, $high>(self.0, val))
    }
  };
}

macro_rules! u16_bool_field {
  ($bit:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get_name(self) -> bool {
      crate::bit_utils::u16_get_bit::<$bit>(self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with_name(self, val: bool) -> Self {
      Self(crate::bit_utils::u16_with_bit::<$bit>(self.0, val))
    }
  };
}

/// Works like [u16_bool_field!] but inverts the meaning on input/output so that
/// a stored "0" is active and a stored "1" is inactive.
macro_rules! u16_low_active_bool_field {
  ($bit:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get_name(self) -> bool {
      // invert bit on output
      !crate::bit_utils::u16_get_bit::<$bit>(self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with_name(self, val: bool) -> Self {
      // invert bit on input
      Self(crate::bit_utils::u16_with_bit::<$bit>(self.0, !val))
    }
  };
}

macro_rules! u32_bool_field {
  ($bit:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get_name(self) -> bool {
      crate::bit_utils::u32_get_bit::<$bit>(self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with_name(self, val: bool) -> Self {
      Self(crate::bit_utils::u32_with_bit::<$bit>(self.0, val))
    }
  };
}

macro_rules! impl_bitops_for {
  ($t:ty) => {
    impl core::ops::BitAnd for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self {
        Self(self.0.bitand(rhs.0))
      }
    }
    impl core::ops::BitOr for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self {
        Self(self.0.bitor(rhs.0))
      }
    }
    impl core::ops::BitXor for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self {
        Self(self.0.bitxor(rhs.0))
      }
    }
    //
    impl core::ops::Not for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self {
        Self(self.0.not())
      }
    }
    //
    impl core::ops::BitAndAssign for $t {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $t {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $t {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0)
      }
    }
  };
}

macro_rules! size_of {
  ($t:ty) => {
    ::core::mem::size_of::<$t>()
  };
}
