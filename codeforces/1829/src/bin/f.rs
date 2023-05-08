use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    writeln!(out, "{}", t)?;

    Ok(())
}
