# cuter nucleotides 🧬

Cuter tricks for binary encoding of nucleotides, powered by Intel AVX-512. Inspired by [Daniel Liu](https://github.com/Daniel-Liu-c0deb0t)'s [cute-nucleotides](https://github.com/Daniel-Liu-c0deb0t/cute-nucleotides).

## Benchmark results

All benchmarks were ran on an Intel Core i5-11300H (Tiger Lake H) processor.

| Encoding        | Throughput       |
|-----------------|------------------|
| mul_compress    | **90.464 GiB/s** |
| movepi8_mask    | 59.218 GiB/s     |
| bitshuffle      | 50.432 GiB/s     |
| *avx2_movemask* | 42.695 GiB/s     |
| *bmi2_pext*     | 29.502 GiB/s     |
