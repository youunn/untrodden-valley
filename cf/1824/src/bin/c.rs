use std::collections::{HashMap, HashSet};

// `dp(v, x)` for operation to make XOR value of subtree of `v` become `x`.
// supposed for all `x` in `dp(v, x)`, `x_0` make `dp(v, x)` smallest,
// then one more operation to get any other `x_1`, where `dp(v, x_1) = dp(v, x_0) + 1`.
// note that the count of `x_0` may be not one.
// then just dp to merge all children to parent,
// where the key is find most frequent `x`.

fn dfs(g: &[Vec<usize>], a: &mut [u32], v: usize, p: usize) -> (usize, HashSet<u32>) {
    let is_root = if p == usize::MAX { 1 } else { 0 };
    let children = g[v].len() + is_root - 1;
    if children == 0 {
        return (0, HashSet::from([a[v]]));
    }

    let mut s = HashSet::new();
    let mut m = HashMap::<u32, usize>::new();
    let mut sum = children;
    let mut vs = Vec::with_capacity(children);

    for &u in &g[v] {
        if u == p {
            continue;
        }
        a[u] ^= a[v];
        let (c, mut sc) = dfs(g, a, u, v);
        if s.len() < sc.len() {
            std::mem::swap(&mut s, &mut sc);
        }
        vs.push(sc);
        sum += c;
    }

    for sc in vs {
        for x in sc {
            if !s.insert(x) {
                *m.entry(x).or_default() += 1;
            }
        }
    }

    let max = *m.iter().map(|(_, c)| c).max().unwrap_or(&0);
    if max != 0 {
        s = m
            .into_iter()
            .filter(|(_, c)| *c == max)
            .map(|(x, _)| x)
            .collect();
    }
    (sum - max - 1, s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{stdin, stdout, BufRead, BufWriter, Write};
    let mut scan = stdin().lock();
    let mut out = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    let buf = &mut buf;

    let n = {
        scan.read_line(buf)?;
        buf.trim().parse::<usize>()?
    };
    let mut a = {
        buf.clear();
        scan.read_line(buf)?;
        buf.split_whitespace()
            .map(&str::parse)
            .take(n)
            .collect::<Result<Vec<u32>, _>>()?
    };
    let g = {
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            buf.clear();
            scan.read_line(buf)?;
            let mut s = buf.split_whitespace();
            let u: usize = s.next().ok_or("")?.parse()?;
            let v: usize = s.next().ok_or("")?.parse()?;
            let (u, v) = (u - 1, v - 1);
            g[u].push(v);
            g[v].push(u);
        }
        g
    };

    let (ans, m) = dfs(&g, &mut a, 0, usize::MAX);
    let offset = if m.contains(&0) { 0 } else { 1 };
    write!(out, "{}", ans + offset)?;
    Ok(())
}
