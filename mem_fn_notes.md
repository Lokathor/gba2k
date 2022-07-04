
#[allow(dead_code)]
fn main() {
    test_memcpy1_forward();
    test_memcpy2_forward();
    test_memcpy2_reverse();
}

pub unsafe extern "C" fn memcpy1_forward(mut dest: *mut u8, mut src: *const u8, mut count: usize) {
    while count >= core::mem::size_of::<u8>() {
        dest.write_volatile(src.read_volatile());
        dest = dest.add(1);
        src = src.add(1);
        count -= core::mem::size_of::<u8>();
    }
}

/// Copies from the low to high address of each region.
/// If the regions overlap, the `src` must be *after* the `dest`.
pub unsafe extern "C" fn memcpy2_forward(
    mut dest: *mut u16,
    mut src: *const u16,
    mut count: usize,
) {
    while count >= core::mem::size_of::<u16>() {
        dest.write_volatile(src.read_volatile());
        dest = dest.add(1);
        src = src.add(1);
        count -= core::mem::size_of::<u16>();
    }
    if count > 0 {
        dest.cast::<u8>()
            .write_volatile(src.cast::<u8>().read_volatile());
    }
}

/// Copies from the high to low address of each region.
/// If the regions overlap, the `src` must be *before* the `dest`.
pub unsafe extern "C" fn memcpy2_reverse(mut dest: *mut u16, mut src: *const u16, count: usize) {
    let mut halfwords = count / 2;
    let spare_bytes = count % 2;
    dest = dest.add(halfwords);
    src = src.add(halfwords);
    if spare_bytes != 0 {
        dest.cast::<u8>()
            .write_volatile(src.cast::<u8>().read_volatile());
    }
    while halfwords > 0 {
        dest = dest.sub(1);
        src = src.sub(1);
        dest.write_volatile(src.read_volatile());
        halfwords -= 1;
    }
}

pub unsafe fn memmove2(dest: *mut u16, src: *const u16, count: usize) {
    if src < dest {
        memcpy2_reverse(dest, src, count)
    } else {
        memcpy2_forward(dest, src, count)
    }
}

/// Copies from the low to high address of each region.
/// If the regions overlap, the `src` must be *after* the `dest`.
pub unsafe extern "C" fn memcpy4_forward(mut dest: *mut u32, mut src: *const u32, mut count: usize) {
    let blocks = count / 32;
    // Copy u32x8 (32 bytes) at a time.
    // Because of push/pop overhead, only do this with 2+ bulk copies.
    if blocks > 2 {
        count -= blocks * 32;
        core::arch::asm!(
            "
            push {{r3, r4, r5, r6, r7, r8, r9, r10}}
            1:
            ldmia r1!, {{r3, r4, r5, r6, r7, r8, r9, r10}}
            stmia r0!, {{r3, r4, r5, r6, r7, r8, r9, r10}}
            subs  r12, r12, #1
            bne 1b
            pop {{r3, r4, r5, r6, r7, r8, r9, r10}}
            ",
            inlateout("r0") dest,
            inlateout("r1") src,
            inlateout("r2") count,
            inlateout("r12") blocks => _,
        );
    }
    let mut pairs = count / 8;
    while pairs > 0 {
        core::arch::asm!(
            "
            ldmia r1!, {{r3, r12}}
            stmia r0!, {{r3, r12}}
            ",
            inlateout("r0") dest,
            inlateout("r1") src,
            out("r3") _,
            out("r12") _,
        );
        pairs -= 1;
    }
    count %= 8;
    if count >= core::mem::size_of::<u32>() {
        dest.write(src.read());
        dest = dest.add(1);
        src = src.add(1);
        count -= core::mem::size_of::<u32>();
    }
    memcpy1_forward(dest.cast::<u8>(), src.cast::<u8>(), count)
}

#[cfg_attr(test,test)]
fn test_memcpy1_forward() {
    let s: [u8; 4] = [1, 2, 3, 4];
    let mut d: [u8; 4] = [0, 0, 0, 0];
    unsafe {
        memcpy1_forward(d.as_mut_ptr(), s.as_ptr(), 4);
        assert_eq!(d, [1, 2, 3, 4]);
        memcpy1_forward(d.as_mut_ptr(), s.as_ptr().add(2), 2);
        assert_eq!(d, [3, 4, 3, 4]);
    }
}

#[cfg_attr(test,test)]
fn test_memcpy2_forward() {
    let s: [u16; 4] = [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD];
    let mut d: [u16; 4] = [0, 0, 0, 0];
    unsafe {
        memcpy2_forward(d.as_mut_ptr(), s.as_ptr(), 8);
        assert_eq!(d, [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]);
        memcpy2_forward(d.as_mut_ptr(), s.as_ptr().add(2), 4);
        assert_eq!(d, [0xCCCC, 0xDDDD, 0xCCCC, 0xDDDD]);
        memcpy2_forward(d.as_mut_ptr().add(1), s.as_ptr(), 6);
        assert_eq!(d, [0xCCCC, 0xAAAA, 0xBBBB, 0xCCCC]);
        memcpy2_forward(d.as_mut_ptr(), s.as_ptr(), 7);
        assert_eq!(d, [0xAAAA, 0xBBBB, 0xCCCC, 0xCCDD]);
        // test overlap
        let d_ptr = d.as_mut_ptr();
        memcpy2_forward(d_ptr, d_ptr.add(1), 3);
        assert_eq!(d, [0xBBBB, 0xBBCC, 0xCCCC, 0xCCDD]);
    }
}

#[cfg_attr(test,test)]
fn test_memcpy2_reverse() {
    let s: [u16; 4] = [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD];
    let mut d: [u16; 4] = [0, 0, 0, 0];
    unsafe {
        memcpy2_reverse(d.as_mut_ptr(), s.as_ptr(), 8);
        assert_eq!(d, [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]);
        memcpy2_reverse(d.as_mut_ptr(), s.as_ptr().add(2), 4);
        assert_eq!(d, [0xCCCC, 0xDDDD, 0xCCCC, 0xDDDD]);
        memcpy2_reverse(d.as_mut_ptr().add(1), s.as_ptr(), 6);
        assert_eq!(d, [0xCCCC, 0xAAAA, 0xBBBB, 0xCCCC]);
        memcpy2_reverse(d.as_mut_ptr(), s.as_ptr(), 7);
        assert_eq!(d, [0xAAAA, 0xBBBB, 0xCCCC, 0xCCDD]);
        // test overlap
        let d_ptr = d.as_mut_ptr();
        memcpy2_reverse(d_ptr.add(1), d_ptr, 3);
        assert_eq!(d, [0xAAAA, 0xAAAA, 0xCCBB, 0xCCDD]);
    }
}
