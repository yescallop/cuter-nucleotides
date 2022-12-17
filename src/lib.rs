#![feature(stdsimd)]

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

macro_rules! u8x8 {
    ($x:expr) => {
        $x * 0x01010101_01010101
    };
}

const AND_MASK: u64 = u8x8!(0b110);
const COMPRESS_MASK: u64 = u8x8!(0b10001000);

pub unsafe fn encode_mul_compress(src: &[u8], mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    let and_mask = _mm512_set1_epi64(AND_MASK as i64);
    let mul_const = _mm512_set1_epi32(0b100000100000100000100000);

    let mut i = 0;
    while i + 64 <= len {
        let chunk = _mm512_loadu_si512(ptr.add(i).cast());
        let and = _mm512_and_si512(chunk, and_mask);
        let mul = _mm512_mullo_epi32(and, mul_const);

        let compress = _mm512_maskz_compress_epi8(COMPRESS_MASK, mul);
        _mm_storeu_si128(dst.cast(), _mm512_castsi512_si128(compress));

        dst = dst.add(16);
        i += 64;
    }

    encode_rest(src, i, dst);
}

pub unsafe fn encode_bitshuffle(src: &[u8], mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    let ctrl1 = _mm512_set1_epi64(i64::from_le_bytes([1, 2, 9, 10, 17, 18, 25, 26]));
    let ctrl2 = _mm512_set1_epi64(i64::from_le_bytes([33, 34, 41, 42, 49, 50, 57, 58]));

    let mut i = 0;
    while i + 64 <= len {
        let chunk = _mm512_loadu_si512(ptr.add(i).cast());
        let gather_lo = _mm512_bitshuffle_epi64_mask(chunk, ctrl1);
        let gather_hi = _mm512_bitshuffle_epi64_mask(chunk, ctrl2);

        let unpack = _mm_unpacklo_epi8(
            _mm_cvtsi64_si128(gather_lo as i64),
            _mm_cvtsi64_si128(gather_hi as i64),
        );
        _mm_storeu_si128(dst.cast(), unpack);

        dst = dst.add(16);
        i += 64;
    }

    encode_rest(src, i, dst);
}

pub unsafe fn encode_pext(src: &[u8], mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    let mut i = 0;
    while i + 32 <= len {
        for _ in 0..4 {
            let chunk = ptr.add(i).cast::<u64>().read_unaligned();
            dst.cast::<u16>()
                .write_unaligned(_pext_u64(chunk, AND_MASK) as _);
            dst = dst.add(2);
            i += 8;
        }
    }

    encode_rest(src, i, dst);
}

unsafe fn encode_rest(src: &[u8], mut i: usize, mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    while i + 4 <= len {
        let chunk = ptr.add(i).cast::<u32>().read_unaligned();

        *dst = _pext_u32(chunk, AND_MASK as u32) as u8;
        dst = dst.add(1);
        i += 4;
    }

    // We use a PKCS#7-like padding, where the last byte is padded with
    // 2-bit integers indicating the number of nucleotides in the byte.
    let mut last = 0b01010101 * (len - i) as u8;
    while i < len {
        last = (last << 2) | ((src[i] >> 1) & 3);
        i += 1;
    }
    *dst = last;
}
