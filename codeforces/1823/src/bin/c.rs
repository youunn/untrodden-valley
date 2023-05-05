use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

const MAXN: usize = 1e7 as usize + 20;
static mut PF: [u64; MAXN] = [0; MAXN];

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
                    PF[j] = i as u64;
                }
            }
        }
    }

    for i in 2..MAXN {
        if PF[i] == 0 {
            PF[i] = i as u64;
        }
    }
}

fn fact(m: &mut HashMap<u64, u64>, mut a: u64) {
    while a != 1 {
        let f = unsafe { PF[a as usize] };
        if let Some(v) = m.get_mut(&f) {
            *v += 1;
        } else {
            m.insert(f, 1);
        }
        a /= f;
    }
}

fn _check(v: Vec<u64>) -> bool {
    // prod(a+1) - len - 1 - len >= 0
    let mut ans = 1;
    let m = 2 * v.len() as u64 + 1;
    for a in &v {
        ans *= a + 1;
        if ans >= m {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let mut s: String = String::new();
    scan.read_line(&mut s)?;
    let t: u32 = s.trim().parse()?;

    unsafe {
        pf_init();
    }

    for _ in 0..t {
        s.clear();
        scan.read_line(&mut s)?;
        let _: usize = s.trim().parse()?;
        s.clear();
        scan.read_line(&mut s)?;
        let v: Vec<u64> = s
            .split_whitespace()
            .map(&str::parse)
            .collect::<Result<_, _>>()?;

        let mut m = HashMap::new();
        for a in v {
            fact(&mut m, a);
        }

        let v: Vec<_> = m.iter().map(|p| p.1).collect();

        // NO: two different
        // YES: two same or any others

        let mut ans = 0;
        let mut rest = 0;
        for a in v {
            ans += a / 2;
            rest += a % 2;
        }
        ans += rest / 3;

        writeln!(out, "{}", ans)?;
    }

    Ok(())
}
