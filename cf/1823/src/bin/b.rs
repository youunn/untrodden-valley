use std::collections::BTreeMap;
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
        let mut ss = s.split_whitespace();
        let _: usize = ss.next().ok_or("")?.parse()?;
        let k: usize = ss.next().ok_or("")?.parse()?;
        s.clear();
        scan.read_line(&mut s)?;
        let v: Vec<u32> = s
            .split_whitespace()
            .map(&str::parse)
            .collect::<Result<_, _>>()?;

        // There are k groups. If any group contains x elements which should be
        // in other group, then the result is 1 if x = 2, and is -1 else.
        // Where group(i) = i % k and i is the index in the ascending order
        
        let mut sv: Vec<_> = v.iter().collect();
        sv.sort();
        let m = BTreeMap::from_iter(sv.iter().enumerate().map(|(k, v)| (v, k)));
        let v: Vec<_> = v.iter().map(|ref a| m[a]).collect();

        let mut ans = 0;
        for (i, a) in v.iter().enumerate() {
            if i % k != a % k {
                ans += 1;
                if ans > 2 {
                    writeln!(out, "-1")?;
                    continue 't;
                }
            }
        }

        writeln!(out, "{}", ans / 2)?;
    }

    Ok(())
}
