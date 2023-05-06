use std::io::BufRead;
use std::io::Write;

const MAXN: usize = 1e6 as usize + 4;
static mut PF: [u32; MAXN] = [0; MAXN];

fn isqrt(a: u64) -> u64 {
    let mut x = a;
    let mut y = 1;
    while x > y {
        x = (x + y) / 2;
        y = a / x;
    }
    x
}

unsafe fn pf_init() {
    PF[1] = 1;
    for i in (2..MAXN).step_by(2) {
        PF[i] = 2;
    }
    let m = isqrt(MAXN as u64) as usize;
    for i in (3..=m).step_by(2) {
        if PF[i] == 0 {
            for j in (i * i..MAXN).step_by(i) {
                if PF[j] == 0 {
                    PF[j] = i as u32;
                }
            }
        }
    }

    for i in 2..MAXN {
        if PF[i] == 0 {
            PF[i] = i as u32;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    unsafe {
        pf_init();
    }

    let mut s = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let mut s = s.split_whitespace();
        let p: u32 = s.next().ok_or("")?.parse()?;
        let q: u32 = s.next().ok_or("")?.parse()?;

        if p == 1 {
            writeln!(out, "YES")?;
            continue;
        }

        if q >= unsafe { PF[p as usize] } {
            writeln!(out, "NO")?;
            continue;
        }

        writeln!(out, "YES")?;
    }

    Ok(())
}
