use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    't: for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let n: u8 = s.trim().parse()?;
        s.clear();
        scan.read_line(&mut s)?;
        // at least
        let mut v: Vec<u8> = s
            .split_whitespace()
            .map(&str::parse)
            .collect::<Result<_, _>>()?;
        v.sort();

        for i in 0..=n {
            if v.iter().filter(|&a| a > &i).count() as u8 == i {
                writeln!(out, "{}", i)?;
                continue 't;
            }
        }

        writeln!(out, "-1")?;
    }

    Ok(())
}
