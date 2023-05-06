use std::collections::BTreeSet;
use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    fn gcd(u: u64, v: &u64) -> u64 {
        let mut u = u;
        let mut v = *v;
        if u == 0 {
            return v;
        } else if v == 0 {
            return u;
        }

        let i = u.trailing_zeros();
        u >>= i;
        let j = v.trailing_zeros();
        v >>= j;
        let k = std::cmp::min(i, j);

        loop {
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }
            v -= u;
            if v == 0 {
                return u << k;
            }
            v >>= v.trailing_zeros();
        }
    }

    for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let n: usize = s.trim().parse()?;
        s.clear();
        scan.read_line(&mut s)?;
        let v: Vec<u64> = s
            .split_whitespace()
            .map(&str::parse)
            .collect::<Result<_, _>>()?;

        let mut res = BTreeSet::new();
        for i in 0..(n + 1) / 2 {
            res.insert(v[i].abs_diff(v[n - 1 - i]));
        }

        writeln!(out, "{}", res.iter().fold(0, gcd))?;
    }

    Ok(())
}
