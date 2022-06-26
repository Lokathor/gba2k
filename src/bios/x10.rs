/// Stores info for unpacking data with [`BitUnPack`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BitUnpackInfo {
  /// Length of the source memory, in bytes.
  pub src_len_bytes: u16,
  /// Width of source elements in bits.
  ///
  /// Only supports 1, 2, 4, or 8
  pub src_elem_bits: u8,
  /// Width of destination elements in bits.
  ///
  /// Only supports 1, 2, 4, 8, 16, or 32
  pub dest_elem_bits: u8,
  /// How much to add to non-zero elements by, and if zero elements should have
  /// the value added too.
  pub offset: BitUnpackOffset,
}

/// Stores info for unpacking data with [`BitUnPack`].
///
/// * The `delta` is how much to add to non-zero elements
/// * The `offset_zeroes` field says if the delta should also be added to zero
///   elements as well.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BitUnpackOffset(u32);

impl BitUnpackOffset {
  pub_const_fn_new!();
  u32_val_field!(0 - 30, delta, with_delta);
  u32_bool_field!(31, offset_zeroes, with_offset_zeroes);
}

/// `swi #0x10`: Decompresses bit-packed elements.
///
/// Normally we have one *element* (unit of data) per *index* within a slice.
/// For compression purposes elements can be "bit-packed" so that more than one
/// element is in a single byte.
///
/// * This decompression processes each source byte as being one or more source
///   elements.
/// * Source elements can be 1,  2, 4, or 8 bits big. When source elements are
///   less than 8 bits each they're stored in a byte from the low bits to the
///   high bits.
/// * The unpacking info supplies an "offset" value which is added to all source
///   elements with a non-zero value. The unpack info can also specify for the
///   offset to be added to source elements that are zero. This creates a
///   destination element.
/// * Each destination element is 1, 2, 4, 8, 16, or 32 bits big. When
///   destination elements are less than 32 bits each they're placed into the
///   output buffer from the low bits to the high bits.
/// * Destination elements are collected into the output buffer until there's 32
///   bits of output stored, and then that word is written to the destination
///   slice.
/// * This means that the number of required destination indexes depends on the
///   number of source bytes, the bits per source element, and the bits per
///   destination element.
///
/// ## Edge Cases
///
/// * **If the number of source elements does not fully fill the output
///   buffer:** the final, partial output buffer *will not* be written.
/// * **If a source element plus the offset overflows the number of bits in a
///   destination element:** elements are combined in the output buffer using
///   `bitor` and shifting. If destination elements overflow they can corrupt
///   the value of following elements in the same output buffer chunk.
///
/// ## Safety
/// * The `src` must be readable for the number of bytes given in
///   `info.src_len_bytes`
/// * The `dest` must be aligned and writable for an appropriate amount of
///   indexes. The required number of indexes varies based on the `info`
///   configuration and the number of `src` bytes.
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
