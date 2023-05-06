use std::collections::VecDeque;
use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scan = &mut std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let (n, l, r): (usize, usize, usize) = r3(scan)?;
    let mut g = vec![vec![]; n];
    for _ in 0..n {
        let (u, v): (usize, usize) = r2(scan)?;
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut v = vec![];
    let mut q = VecDeque::new();
    let mut vs = vec![false; g.len()];
    while let Some(i) = vs.iter().enumerate().find(|x| !*x.1).map(|x| x.0) {
        q.push_back(i);
        vs[i] = true;
        let mut c = 1;
        while let Some(i) = q.pop_front() {
            for &i in &g[i] {
                if !vs[i] {
                    q.push_back(i);
                    vs[i] = true;
                    c += 1;
                }
            }
        }
        v.push(c);
    }

    let ans = v
        .iter()
        .filter(|x| **x <= l + r - 1)
        .fold(0, |p, c| p ^ (c / l));

    if ans > 0 {
        writeln!(out, "Alice")?;
    } else {
        writeln!(out, "Bob")?;
    }

    Ok(())
}

macro_rules! r {
    ( $f:ident, $($t:ident),+ ) => {
        fn $f<$($t),+>(scan: &mut std::io::StdinLock) -> Result<($($t),+), Box<dyn std::error::Error>>
        where
            $($t: std::str::FromStr,
            <$t>::Err: std::error::Error + 'static,)+
        {
            let mut s: String = String::new();
            scan.read_line(&mut s)?;
            let mut s = s.split_whitespace();
            Ok(($(s.next().ok_or("")?.parse::<$t>()?),+))
        }
    };
}

r!(r2, T1, T2);
r!(r3, T1, T2, T3);
