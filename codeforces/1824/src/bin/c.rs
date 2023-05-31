use std::collections::{HashMap, HashSet};

// `dp(v, x)` for operation to make XOR value of subtree of `v` become `x`.
// supposed for all `x` in `dp(v, x)`, `x_0` make `dp(v, x)` smallest,
// then one more operation to get any other `x_1`, where `dp(v, x_1) = dp(v, x_0) + 1`.
// note that the count of `x_0` may be not one.
// then just dp to merge all children to parent,
// where the key is find most frequent `x`.

fn dfs(g: &[Vec<usize>], a: &[u32], v: usize, p: usize) -> (usize, HashSet<u32>) {
    let is_root = if p == usize::max_value() { 1 } else { 0 };
    if is_root + g[v].len() == 1 {
        return (0, HashSet::from([a[v]]));
    }

    let mut m = HashMap::<u32, usize>::new();
    let mut sum = 0;

    for &u in &g[v] {
        if u == p {
            continue;
        }
        let (c, s) = dfs(g, a, u, v);
        for x in s {
            *m.entry(a[v] ^ x).or_default() += 1;
        }
        sum += c + 1;
    }

    let mut max = 0;
    let mut s = HashSet::new();
    for (x, c) in m {
        if c > max {
            s.clear();
            s.insert(x);
            max = c;
        } else if c == max {
            s.insert(x);
        }
    }

    (sum - max, s)
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
    let a = {
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
            g[u - 1].push(v - 1);
            g[v - 1].push(u - 1);
        }
        g
    };

    let (ans, m) = dfs(&g, &a, 0, usize::max_value());
    let offset = if m.contains(&0) { 0 } else { 1 };
    write!(out, "{}", ans + offset)?;
    Ok(())
}
