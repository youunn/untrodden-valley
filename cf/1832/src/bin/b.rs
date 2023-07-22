use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = stdin().lock();
    let mut out = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    let buf = &mut buf;
    let t: usize = {
        scan.read_line(buf)?;
        std::mem::take(buf).trim().parse()?
    };

    for _ in 0..t {
        let (n, k): (usize, usize) = {
            scan.read_line(buf)?;
            let buf = std::mem::take(buf);
            let mut s = buf.split_whitespace();
            (s.next().ok_or("")?.parse()?, s.next().ok_or("")?.parse()?)
        };
        let mut v: Vec<u64> = {
            scan.read_line(buf)?;
            std::mem::take(buf)
                .split_whitespace()
                .map(&str::parse)
                .collect::<Result<_, _>>()?
        };
        v.sort();
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + v[i];
        }
        let mut ans = 0;
        for i in 0..=k {
            ans /= cmp::max / (s[n - k + i] - s[i * 2]);
        }

        writeln!(out, "{}", ans)?;
    }

    Ok(())
}

#[allow(dead_code)]
mod cmp {
    use std::ops::{Div, DivAssign};

    #[allow(non_camel_case_types)]
    pub struct max;
    #[allow(non_camel_case_types)]
    pub struct max_partial<T: Ord>(T);
    impl Div<u64> for max {
        type Output = max_partial<u64>;
        fn div(self, rhs: u64) -> Self::Output {
            max_partial(rhs)
        }
    }
    impl DivAssign<max_partial<u64>> for u64 {
        fn div_assign(&mut self, rhs: max_partial<u64>) {
            *self = (*self).max(rhs.0)
        }
    }
}
