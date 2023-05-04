use std::{
    collections::VecDeque,
    io::{BufRead, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let t: u32 = r1(&mut scan)?;

    fn d(g: &Vec<Vec<usize>>, i: usize) -> Vec<u64> {
        let mut q = VecDeque::new();
        let mut d = vec![u64::max_value(); g.len()];
        d[i] = 0;
        q.push_back(i);

        while let Some(i) = q.pop_front() {
            for &j in &g[i] {
                if d[j] == u64::max_value() {
                    d[j] = d[i] + 1;
                    q.push_back(j);
                }
            }
        }

        d
    }

    for _ in 0..t {
        let (n, k, c): (usize, u64, u64) = r3(&mut scan)?;
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let (u, v): (usize, usize) = r2(&mut scan)?;
            g[u - 1].push(v - 1);
            g[v - 1].push(u - 1);
        }

        let d0 = d(&g, 0);
        let a = d0
            .iter()
            .enumerate()
            .max_by_key(|d| d.1)
            .map(|d| d.0)
            .ok_or("")?;
        let da = d(&g, a);
        let b = da
            .iter()
            .enumerate()
            .max_by_key(|d| d.1)
            .map(|d| d.0)
            .ok_or("")?;
        let db = d(&g, b);

        let mut ans = 0;
        let iter = da.into_iter().zip(db.into_iter()).zip(d0.into_iter());
        for ((a, b), x) in iter {
            let r#in = a.max(b) * k;
            let out = x * c;
            if r#in > out {
                ans = ans.max(r#in - out);
            }
        }

        writeln!(out, "{}", ans)?;
    }

    Ok(())
}

macro_rules! r {
    ( $f:ident, $($t:ident),+ ) => {
        #[allow(unused_parens)]
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

r!(r1, T1);
r!(r2, T1, T2);
r!(r3, T1, T2, T3);
