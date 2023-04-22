use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    const M: u64 = 998244353;

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let mut s = s.split_whitespace();
    let n: u32 = s.next().ok_or("io")?.parse()?;
    let m: u32 = s.next().ok_or("io")?.parse()?;
    let k: u32 = s.next().ok_or("io")?.parse()?;

    // 在n中选t段k长的不相邻的线段 * 2^t，即
    // 在n+1中选t段k+1长的线段 * 2^t
    // let n=n+1; let k=k+1; // 3e5
    // 在n中选t段k长的线段 * 2^t
    // f(n,t) = f(n-k,t) + sum(f(n-k,t-1)...f(n-2k+1,t-1))
    // 不会数学

    writeln!(out, "{}", n)?;

    Ok(())
}
