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

    'outer: for _ in 0..t {
        let s = {
            scan.read_line(buf)?;
            std::mem::take(buf)
        };
        let s = s.trim().as_bytes();
        let l = s.len() / 2;
        let c0 = s[0];
        for &c in s.iter().take(l) {
            if c == c0 {
                continue;
            }
            writeln!(out, "YES")?;
            continue 'outer;
        }
        writeln!(out, "NO")?;
    }

    Ok(())
}
