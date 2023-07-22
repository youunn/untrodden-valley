use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

fn find_path(g: &[Vec<usize>], from: usize, to: usize) -> Vec<usize> {
    let mut p = HashMap::new();
    let mut s = vec![to];
    let mut vs = vec![false; g.len()];
    vs[to] = true;

    while let Some(c) = s.pop() {
        if c == from {
            break;
        }

        for &i in g[c].iter() {
            if !vs[i] {
                p.insert(i, c);
                vs[i] = true;
                s.push(i);
            }
        }
    }

    let mut path = Vec::new();
    let mut c = from;
    path.push(c);

    while let Some(&p) = p.get(&c) {
        path.push(p);
        c = p;
    }

    path
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scan = &mut std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    const M: u64 = 998244353;

    let (n, s, t): (usize, usize, usize) = r3(scan)?;
    let (s, t) = (s - 1, t - 1);
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (u, v): (usize, usize) = r2(scan)?;
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let p = find_path(&g, s, t);

    let mut ds = vec![0; n];
    let mut vs = vec![false; n];
    let mut p = p.into_iter().rev();
    let mut cur = p.next().ok_or("")?;
    let mut d = 0;
    loop {
        let next = p.next();
        let mut s = vec![];
        vs[cur] = true;
        ds[cur] = d;
        for &i in g[cur].iter() {
            if Some(i) != next && !vs[i] {
                ds[i] = d;
                vs[i] = true;
                s.push(i);
            }
        }

        while let Some(c) = s.pop() {
            for &i in g[c].iter() {
                if !vs[i] {
                    ds[i] = d;
                    vs[i] = true;
                    s.push(i);
                }
            }
        }

        d += 1;
        if let Some(next) = next {
            cur = next;
        } else {
            break;
        }
    }

    for i in 0..n {
        if i == t {
            write!(out, "1 ")?;
        } else {
            write!(out, "{} ", ds[i] as u64 % M * g[i].len() as u64 % M)?;
        }
    }

    writeln!(out)?;
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
