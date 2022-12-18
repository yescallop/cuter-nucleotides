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

        let mut dst = 0;
        unsafe {
            f(b"ATC", &mut dst);
            assert_eq!(dst, 0b11_011000);
            f(b"AT", &mut dst);
            assert_eq!(dst, 0b1010_1000);
            f(b"A", &mut dst);
            assert_eq!(dst, 0b010101_00);
            f(b"", &mut dst);
            assert_eq!(dst, 0);
        }
    };

    test(encode_bitshuffle);
    test(encode_mul_compress);
    test(encode_movepi8_mask);
    test(encode_avx2_movemask);
    test(encode_bmi2_pext);

    Ok(())
}

#[test]
fn test_decode() -> io::Result<()> {
    let src = fs::read("nucleotides.bin")?;
    let expected = fs::read("nucleotides.txt")?;

    let test = |f: unsafe fn(&[u8], *mut u8) -> usize| {
        let mut dst = Vec::with_capacity(src.len() * 4);
        unsafe {
            let len = f(&src, dst.as_mut_ptr());
            dst.set_len(len);
        }
        assert_eq!(dst, expected);

        let mut dst = [0; 4];
        unsafe {
            let len = f(&[0b11_011000], dst.as_mut_ptr());
            assert_eq!(&dst[..len], b"ATC");
            let len = f(&[0b1010_1000], dst.as_mut_ptr());
            assert_eq!(&dst[..len], b"AT");
            let len = f(&[0b010101_00], dst.as_mut_ptr());
            assert_eq!(&dst[..len], b"A");
            let len = f(&[0], dst.as_mut_ptr());
            assert_eq!(&dst[..len], b"");
            let len = f(&[], dst.as_mut_ptr());
            assert_eq!(&dst[..len], b"");
        }
    };

    test(decode_multishift);
    test(decode_shift_shuffle);
    test(decode_pdep_shuffle);
    test(decode_naive_lut);

    Ok(())
}
