# cuter nucleotides 🧬

Cuter tricks for binary encoding of nucleotides, powered by Intel AVX-512. Inspired by [Daniel Liu](https://github.com/Daniel-Liu-c0deb0t)'s [cute-nucleotides](https://github.com/Daniel-Liu-c0deb0t/cute-nucleotides).

## Benchmark results

| Encoding     | Throughput       |
|--------------|------------------|
| mul_compress | **73.327 GiB/s** |
| bitshuffle   | 49.767 GiB/s     |
| pext         | 29.411 GiB/s     |
