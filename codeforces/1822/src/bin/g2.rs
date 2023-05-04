use std::collections::HashMap;
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
        let _: u32 = s.trim().parse()?;
        s.clear();
        scan.read_line(&mut s)?;

        let mut cache = HashMap::<u64, u64>::new();
        let mut v: Vec<u64> = s
            .split_whitespace()
            .map(|s| {
                let res = s.parse().unwrap_or(0);
                *cache.entry(res).or_default() += 1;
                res
            })
            .collect();
        v.sort();

        let mut ans = 0u64;
        let max = *v.last().ok_or("")?;
        let mut past = 0;

        for &a in &v {
            if a == past {
                continue;
            }

            let x = cache.get(&a).unwrap_or(&0);
            if *x >= 3 {
                ans += x * (x - 1) * (x - 2);
            }

            let mut b = 1;
            while b * b <= a && a * b <= max {
                if a % b != 0 {
                    b += 1;
                    continue;
                }

                let ai = a / b;
                if b != 1 {
                    let xi = cache.get(&ai).unwrap_or(&0);
                    let ak = a * b;
                    let xk = cache.get(&ak).unwrap_or(&0);
                    ans += xi * x * xk;
                }

                let b2 = ai;
                if b2 == b || a * b2 > max {
                    b += 1;
                    continue;
                }

                if b2 != 1 {
                    let ai = b;
                    let xi = cache.get(&ai).unwrap_or(&0);
                    let ak = a * b2;
                    let xk = cache.get(&ak).unwrap_or(&0);
                    ans += xi * x * xk;
                }

                b += 1;
            }
            past = a;
        }

        writeln!(out, "{}", ans)?;
    }

    Ok(())
}
