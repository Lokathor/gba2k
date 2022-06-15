macro_rules! pub_const_fn_new {
  () => {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
      Self(0)
    }
  };
}

macro_rules! unsafe_u16_enum_field {
  ($low:literal - $high:literal: $enum_ty:ty, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    pub const fn $get_name(self) -> $enum_ty {
      unsafe {
        core::mem::transmute(crate::bit_utils::u16_get_region::<$low, $high>(
          self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    pub const fn $with_name(self, val: $enum_ty) -> Self {
      Self(crate::bit_utils::u16_with_region::<$low, $high>(self.0, val as u16))
    }
  };
}

macro_rules! unsafe_u16_val_field {
  ($low:literal - $high:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    pub const fn $get_name(self) -> u16 {
      crate::bit_utils::u16_get_value::<$low, $high>(self.0)
    }
    #[inline]
    #[must_use]
    pub const fn $with_name(self, val: u16) -> Self {
      Self(crate::bit_utils::u16_with_value::<$low, $high>(self.0, val))
    }
  };
}

macro_rules! u16_bool_field {
  ($bit:literal, $get_name:ident, $with_name: ident) => {
    #[inline]
    #[must_use]
    pub const fn $get_name(self) -> bool {
      crate::bit_utils::u16_get_bit::<$bit>(self.0)
    }
    #[inline]
    #[must_use]
    pub const fn $with_name(self, val: bool) -> Self {
      Self(crate::bit_utils::u16_with_bit::<$bit>(self.0, val))
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
