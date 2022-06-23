/// Stores info for unpacking data with [`BitUnPack`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BitUnpackInfo {
  /// Length of the source memory, in bytes.
  pub src_len_bytes: u16,
  /// Width of source elements in bits.
  ///
  /// Supports 1, 2, 4, or 8
  pub src_elem_bits: u8,
  /// Width of destination elements in bits.
  ///
  /// Supports 1, 2, 4, 8, 16, or 32
  pub dest_elem_bits: u8,
  /// This field has two pieces of information at once.
  ///
  /// * Bits other than the high bit give a number to add to all non-zero
  ///   elements as they're inflated.
  /// * The highest bit should be set if the offset value should *also* be
  ///   added to the zero elements as well.
  pub offset_and_zero_offset: u32,
}

/// `swi 0x10`: Decompresses bit-packed elements.
///
///
///
/// ## Safety
/// * The `src` must be readable for the number of bytes given in
///   `info.src_len_bytes`
/// * The `dest` must be aligned and writable for an appropriate amount. The
///   exact amount of space needed for the output depends on the ratio between
///   `info.src_elem_bits` and `info.dest_elem_bits`, as well as the number of
///   source bytes.
/// * `info.src_elem_bits` must be 1, 2, 4, or 8.
/// * `info.dest_elem_bits` must be 1, 2, 4, 8, 16, or 32.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BitUnPack(src: *const u8, dest: *mut u32, info: &BitUnpackInfo) {
  core::arch::asm! {
    "swi #0x10",
    inout("r0") src => _,
    inout("r1") dest => _,
    inout("r2") info => _,
    out("r3") _,
    options(preserves_flags),
  };
}
