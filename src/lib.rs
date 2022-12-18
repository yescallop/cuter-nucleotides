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
    while i + 256 <= len {
        for _ in 0..4 {
            let chunk = _mm512_loadu_si512(ptr.add(i).cast());
            let and = _mm512_and_si512(chunk, and_mask);
            let mul = _mm512_mullo_epi32(and, mul_const);

            let compress = _mm512_maskz_compress_epi8(COMPRESS_MASK, mul);
            _mm_storeu_si128(dst.cast(), _mm512_castsi512_si128(compress));

            dst = dst.add(16);
            i += 64;
        }
    }

    encode_rest(src, i, dst);
}

pub unsafe fn encode_bitshuffle(src: &[u8], mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    let idx = _mm512_setr_epi32(0, 8, 1, 9, 2, 10, 3, 11, 4, 12, 5, 13, 6, 14, 7, 15);
    let ctrl_lo = _mm512_set1_epi64(i64::from_le_bytes([1, 2, 9, 10, 17, 18, 25, 26]));
    let ctrl_hi = _mm512_set1_epi64(i64::from_le_bytes([33, 34, 41, 42, 49, 50, 57, 58]));

    let mut i = 0;
    while i + 256 <= len {
        for _ in 0..4 {
            let chunk = _mm512_loadu_si512(ptr.add(i).cast());
            let perm = _mm512_permutexvar_epi32(idx, chunk);
            let lo = _mm512_bitshuffle_epi64_mask(perm, ctrl_lo);
            let hi = _mm512_bitshuffle_epi64_mask(perm, ctrl_hi);

            _store_mask64(dst.cast(), lo);
            _store_mask64(dst.add(8).cast(), hi);

            dst = dst.add(16);
            i += 64;
        }
    }

    encode_rest(src, i, dst);
}

// Original: Daniel Liu, aqrit
pub unsafe fn encode_movepi8_mask(src: &[u8], mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    let idx = _mm512_setr_epi64(0, 4, 1, 5, 2, 6, 3, 7);

    let mut i = 0;
    while i + 256 <= len {
        for _ in 0..4 {
            let v = _mm512_loadu_si512(ptr.add(i).cast());
            let v = _mm512_permutexvar_epi64(idx, v);
            let lo = _mm512_slli_epi64(v, 6);
            let hi = _mm512_slli_epi64(v, 5);
            let a = _mm512_unpackhi_epi8(lo, hi);
            let b = _mm512_unpacklo_epi8(lo, hi);

            _store_mask64(dst.cast(), _mm512_movepi8_mask(b));
            _store_mask64(dst.add(8).cast(), _mm512_movepi8_mask(a));

            dst = dst.add(16);
            i += 64;
        }
    }

    encode_rest(src, i, dst);
}

// Source: Daniel Liu, aqrit
pub unsafe fn encode_avx2_movemask(src: &[u8], mut dst: *mut u8) {
    let len = src.len();
    let ptr = src.as_ptr();

    let mut i = 0;
    while i + 128 <= len {
        for _ in 0..4 {
            let v = _mm256_loadu_si256(ptr.add(i).cast());
            let v = _mm256_permute4x64_epi64(v, 0b11011000);
            let lo = _mm256_slli_epi64(v, 6);
            let hi = _mm256_slli_epi64(v, 5);
            let a = _mm256_unpackhi_epi8(lo, hi);
            let b = _mm256_unpacklo_epi8(lo, hi);

            dst.cast::<i32>().write_unaligned(_mm256_movemask_epi8(b));
            dst.add(4)
                .cast::<i32>()
                .write_unaligned(_mm256_movemask_epi8(a));

            dst = dst.add(8);
            i += 32;
        }
    }

    encode_rest(src, i, dst);
}

pub unsafe fn encode_bmi2_pext(src: &[u8], mut dst: *mut u8) {
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
