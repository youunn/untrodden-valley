use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    't: for _ in 0..t {
        let (n, k, x, c) = {
            s.clear();
            scan.read_line(&mut s)?;
            let mut ss = s.split_whitespace();
            let n: u32 = ss.next().ok_or("")?.parse()?;
            let k: u8 = ss.next().ok_or("")?.parse()?;
            s.clear();
            scan.read_line(&mut s)?;
            let x: Vec<u32> = s
                .split_whitespace()
                .map(&str::parse)
                .collect::<Result<_, _>>()?;
            s.clear();
            scan.read_line(&mut s)?;
            let c: Vec<u32> = s
                .split_whitespace()
                .map(&str::parse)
                .collect::<Result<_, _>>()?;
            (n, k, x, c)
        };

        let mut cc = 0;
        let mut cx = 0;

        let l2 = [b'x', b'y', b'z'];
        let mut l = 0;
        let mut s = Vec::with_capacity(n as usize);
        for i in 0..k {
            let ii = i as usize;

            if i == 0 {
                if c[ii] - cc > x[ii] - cx {
                    writeln!(out, "NO")?;
                    continue 't;
                }
                for _ in 3..c[ii] {
                    s.push(l2[l]);
                }
                for _ in c[ii] - 3..x[ii] {
                    s.push(l2[l % 3]);
                    l += 1;
                }
            } else {
                if c[ii] - cc > x[ii] - cx {
                    writeln!(out, "NO")?;
                    continue 't;
                }

                for _ in cc..c[ii] {
                    s.push(b'a' + i);
                }

                for _ in c[ii] - cc..x[ii] - cx {
                    s.push(l2[l % 3]);
                    l += 1;
                }
            }

            cc = c[ii];
            cx = x[ii];
        }
        writeln!(out, "YES")?;
        writeln!(out, "{}", std::str::from_utf8(&s)?)?;
    }

    Ok(())
}
