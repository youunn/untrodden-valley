use cp::cmp::max;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();
    let t: usize = io.read()?;
    for _ in 0..t {
        let (n, k) = io.read2::<usize, usize>()?;
        let mut v = io.read_vec::<usize, Vec<_>>(n)?;
        v.sort();
        let mut c: usize = 1;
        let mut m: usize = 1;
        for i in 1..n {
            if v[i] - v[i - 1] <= k {
                c += 1;
            } else {
                m /= max / c;
                c = 1;
            }
        }
        io.print(n - (m / max / c))?;
    }

    Ok(())
}
