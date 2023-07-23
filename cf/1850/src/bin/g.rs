fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();
    let t: usize = io.read()?;
    for _ in 0..t {
        let n: usize = io.read()?;
        let mut mh = std::collections::HashMap::<i64, i64>::new();
        let mut mv = mh.clone();
        let mut mp = mh.clone();
        let mut mn = mh.clone();
        for _ in 0..n {
            let (x, y) = io.read2::<i64, i64>()?;
            *mh.entry(x).or_default() += 1;
            *mv.entry(y).or_default() += 1;
            *mp.entry(x - y).or_default() += 1;
            *mn.entry(x + y).or_default() += 1;
        }

        let mut ans = 0;

        for m in [mh, mv, mp, mn] {
            for (_, v) in m {
                ans += v * (v - 1);
            }
        }

        io.print(ans)?;
    }

    Ok(())
}
