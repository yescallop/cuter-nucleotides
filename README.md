# cuter nucleotides 🧬💻

Cuter tricks for binary encoding and decoding of nucleotides, powered by Intel AVX-512. Based on [Daniel Liu](https://github.com/Daniel-Liu-c0deb0t)'s [cute-nucleotides](https://github.com/Daniel-Liu-c0deb0t/cute-nucleotides).

## Cuter benchmark results 📈

All benchmarks were ran on an Intel Core i5-11300H (Tiger Lake H) processor.

No AVX-512 intrinsics are used in the *italicized functions*.

| Encoding function | Throughput[^1]   |
|-------------------|------------------|
| mul_compress      | **90.464 GiB/s** |
| bitshuffle        | 79.889 GiB/s     |
| movepi8_mask      | 59.218 GiB/s     |
| *avx2_movemask*   | 42.695 GiB/s     |
| *bmi2_pext*       | 29.502 GiB/s     |

| Decoding function | Throughput[^1]   |
|-------------------|------------------|
| multishift        | **64.005 GiB/s** |
| shift_shuffle     | **64.039 GiB/s** |
| *naive_lut*[^2]   | 20.091 GiB/s     |
| pdep_shuffle      | 15.868 GiB/s     |

[^1]: The length of input/output ASCII string divided by the time per iteration.
[^2]: Observed to be auto-vectorized.
