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
        let n: usize = {
            scan.read_line(buf)?;
            std::mem::take(buf).trim().parse()?
        };
        let v: Vec<u64> = {
            scan.read_line(buf)?;
            let mut last = None;
            let mut v = Vec::with_capacity(n);
            for s in std::mem::take(buf).split_whitespace() {
                if Some(s) != last {
                    v.push(s.parse()?);
                    last = Some(s)
                }
            }
            v
        };
        let ans = match v.len() {
            1 => 1,
            len => {
                let mut ans = 2;
                for i in 1..len - 1 {
                    if v[i] < v[i + 1] && v[i] < v[i - 1] {
                        ans += 1;
                    }
                    if v[i] > v[i + 1] && v[i] > v[i - 1] {
                        ans += 1;
                    }
                }
                ans
            }
        };

        writeln!(out, "{}", ans)?;
    }

    Ok(())
}
