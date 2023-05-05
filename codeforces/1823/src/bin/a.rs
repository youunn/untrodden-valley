use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    #[inline]
    fn c2(i: u32) -> u32 {
        if i == 0 {
            0
        } else {
            i * (i - 1) / 2
        }
    }

    't: for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let mut s = s.split_whitespace();
        let n: u32 = s.next().ok_or("")?.parse()?;
        let k: u32 = s.next().ok_or("")?.parse()?;

        // n = a + b
        // k = a * (a + 1) / 2 + b * (b + 1) / 2
        // find some u32 solution

        for a in 0..=n / 2 {
            let b = n - a;
            let k1 = c2(a) + c2(b);
            if k != k1 {
                continue;
            }

            writeln!(out, "YES")?;
            for _ in 0..a {
                write!(out, "1 ")?;
            }
            for _ in 0..b {
                write!(out, "-1 ")?;
            }
            writeln!(out)?;
            continue 't;
        }

        writeln!(out, "NO")?;
    }

    Ok(())
}
