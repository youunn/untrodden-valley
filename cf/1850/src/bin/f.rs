fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();
    let t: usize = io.read()?;
    for _ in 0..t {
        let n: usize = io.read()?;
        let mut c = vec![0; n + 1];
        for x in io.read_vec::<usize, Vec<usize>>(n)? {
            if x > n {
                continue;
            }
            c[x] += 1
        }
        let mut p = vec![0; n + 1];
        for (i, &x) in c.iter().enumerate().skip(1) {
            if x == 0 {
                continue;
            }
            for i in (i..n + 1).step_by(i) {
                p[i] += x;
            }
        }
        io.print(p.into_iter().max().unwrap_or_default())?;
    }

    Ok(())
}
