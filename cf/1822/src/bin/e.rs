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
        let n: u32 = s.trim().parse()?;

        if n % 2 == 1 {
            scan.read_line(&mut s)?;
            writeln!(out, "-1")?;
            continue;
        }

        let h = n / 2;
        s.clear();
        scan.read_line(&mut s)?;
        let s = s.as_bytes();

        let mut m = vec![0u32; 26];
        let mut ans = vec![0u32; 26];
        for i in 0..h {
            let cl = s[i as usize];
            let cr = s[(n - i - 1) as usize];
            let kl = (cl - b'a') as usize;
            m[kl] += 1;

            if cl == cr {
                m[kl] += 1;
                ans[kl] += 1;
            } else {
                let kr = (cr - b'a') as usize;
                m[kr] += 1;
                if m[kr] > h {
                    writeln!(out, "-1")?;
                    continue 't;
                }
            }

            if m[kl] > h {
                writeln!(out, "-1")?;
                continue 't;
            }
        }

        let max = ans.iter().max().ok_or("")?;
        let sum: u32 = ans.iter().sum();
        let h = sum / 2;

        if max <= &h {
            writeln!(out, "{}", (sum + 1) / 2)?;
        } else {
            writeln!(out, "{}", max)?;
        }
    }

    Ok(())
}
