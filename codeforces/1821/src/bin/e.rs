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
        let mut k: u8 = s.trim().parse()?;
        s.clear();
        scan.read_line(&mut s)?;
        let l = s.trim().as_bytes();

        // calculate sum of all right brackets to the end

        let n = l.len();
        let mut stack = Vec::with_capacity(n);
        let mut moves = vec![0; n + 1];
        let mut cost = 0;

        for (i, c) in l.iter().enumerate() {
            if c == &b'(' {
                cost += stack.len();
                stack.push(i);
            } else {
                moves[(i - stack.pop().ok_or("")?) / 2] += 1;
            }
        }

        for t in (0..=n).rev() {
            let m = k.min(moves[t]);
            cost -= m as usize * t;
            k -= m;
            if k == 0 {
                break;
            }
        }

        writeln!(out, "{}", cost)?;
    }

    Ok(())
}
