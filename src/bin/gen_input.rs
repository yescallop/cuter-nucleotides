use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use rand::{distributions::Slice, Rng};

fn main() -> io::Result<()> {
    let nucleotides = Slice::new(&[b'A', b'T', b'C', b'G']).unwrap();
    let mut bw_txt = BufWriter::new(File::create("nucleotides.txt")?);
    let mut bw_bin = BufWriter::new(File::create("nucleotides.bin")?);
    let mut rng = rand::thread_rng();
    for _ in 0..1024 * 1024 / 4 {
        let a = *rng.sample(nucleotides);
        let b = *rng.sample(nucleotides);
        let c = *rng.sample(nucleotides);
        let d = *rng.sample(nucleotides);
        bw_txt.write(&[a, b, c, d])?;

        let a = (a >> 1) & 3;
        let b = (b >> 1) & 3;
        let c = (c >> 1) & 3;
        let d = (d >> 1) & 3;
        bw_bin.write(&[a | b << 2 | c << 4 | d << 6])?;
    }
    bw_bin.write(&[0])?;

    Ok(())
}
