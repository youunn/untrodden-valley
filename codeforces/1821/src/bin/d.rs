use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    for _ in 0..t {
        let mut s: String = String::new();
        scan.read_line(&mut s)?;
        let mut v = s.split_whitespace();
        let _n: usize = v.next().ok_or("")?.parse()?;
        let k: usize = v.next().ok_or("")?.parse()?;
        s.clear();
        scan.read_line(&mut s)?;
        let vl: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        s.clear();
        scan.read_line(&mut s)?;
        let vr: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        let mut s = 0;
        let mut ans = usize::MAX;
        let mut h = BinaryHeap::new();

        for (l, r) in vl.into_iter().zip(vr.into_iter()) {
            let len = r - l + 1;
            h.push(Reverse(len));
            s += len;
            while s >= k {
                ans = ans.min(2 * h.len() + k + r - s);
                s -= h.pop().ok_or("")?.0;
            }
        }

        if ans != usize::MAX {
            writeln!(out, "{}", ans)?;
        } else {
            writeln!(out, "{}", -1)?;
        }
    }

    Ok(())
}
