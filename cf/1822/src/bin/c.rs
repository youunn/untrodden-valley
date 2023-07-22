use std::io::BufRead;
use std::io::Write;

fn solve(i: u64) -> u64 {
    // for 3
    // f3 = (1 + 1 + 1) + (2 + 2 + 1) * 2 - 1 + (3 + 1 + 1) = 17
    // for 4
    // f4 = f3 - 1 + 4 * 2 + 1 * 2 = 26
    // for 5
    // f5 = f4 - 1 + 5 * 2 + 1 * 2 = 37
    // let f(n) = f(n-1) + 2n + 1
    // f(n) = a * n^2 + b * n + c
    // 9a + 3b + c = 17
    // 16a + 4b + c = 26
    // 25a + 5b + c = 37
    i * i + 2 * i + 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let i: u64 = s.trim().parse()?;

        writeln!(out, "{}", solve(i))?;
    }

    Ok(())
}
