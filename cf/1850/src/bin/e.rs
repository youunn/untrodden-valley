fn isqrt(a: u128) -> u64 {
    let mut x = a;
    let mut y = 1;
    while x > y {
        x = (x + y) / 2;
        y = a / x;
    }
    x as u64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();
    let t: usize = io.read()?;
    for _ in 0..t {
        let (n, c) = io.read2::<usize, u64>()?;
        let v = io.read_vec::<u64, Vec<_>>(n)?;

        let sl = v.iter().sum::<u64>();
        let sp = v.iter().map(|x| x * x).sum::<u64>();

        // n \cdot m^2 + 2 \cdot sl \cdot m + (sp - c) = 0

        // let m = (isqrt(2 * sl * 2 * sl - 4 * n * (sp - c)) - 2 * sl) / 2 * n;
        // let m = (isqrt(sl * sl - n * (sp - c)) - sl) / n;
        let m = (isqrt(sl as u128 * sl as u128 + n as u128 * (c - sp) as u128) - sl) / n as u64;

        io.print(m / 2)?;
    }

    Ok(())
}
