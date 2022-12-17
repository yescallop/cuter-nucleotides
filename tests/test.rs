use std::{fs, io};

use cuter_nucleotides::*;

#[test]
fn test_encode() -> io::Result<()> {
    let src = fs::read("nucleotides.txt")?;
    let expected = fs::read("nucleotides.bin")?;

    let test = |f: unsafe fn(&[u8], *mut u8)| {
        let mut dst = Vec::with_capacity(src.len() / 4 + 1);
        unsafe {
            f(&src, dst.as_mut_ptr());
            dst.set_len(dst.capacity());
        }
        assert_eq!(dst, expected);
    };

    test(encode_bitshuffle);
    test(encode_mul_compress);
    test(encode_movepi8_mask);
    test(encode_avx2_movemask);
    test(encode_bmi2_pext);

    Ok(())
}
