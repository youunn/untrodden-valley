use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let n: u32 = s.trim().parse()?;

        if n == 1 {
            writeln!(out, "1")?;
        } else if n % 2 == 1 {
            writeln!(out, "-1")?;
        } else {
            for i in 0..n {
                if i % 2 == 0 {
                    write!(out, "{} ", n - i)?;
                } else {
                    write!(out, "{} ", i)?;
                }
            }
            writeln!(out)?;
        }
    }

    Ok(())
}
