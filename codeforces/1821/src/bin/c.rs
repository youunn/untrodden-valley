use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let mut s = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;
    for _ in 0..t {
        let mut s = String::new();
        scan.read_line(&mut s)?;
        let s = s.trim().as_bytes();
        let mut ans = u32::MAX;
        for c in b'a'..=b'z' {
            let mut m = 0;
            let mut x = 0;
            for s in s {
                if s == &c {
                    m = m.max(x);
                    x = 0;
                } else {
                    x += 1;
                }
            }
            m = m.max(x);
            ans = ans.min(m);
        }
        writeln!(out, "{}", u32::BITS - ans.leading_zeros())?;
    }
    Ok(())
}
