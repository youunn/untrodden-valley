use std::{
    error::Error,
    io::{stdin, stdout, BufRead, BufWriter, Write},
};

fn merge(dp1: Vec<(u64, u64)>, dp2: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut res = vec![(u64::max_value(), u64::max_value()); dp1.len() + dp2.len()];
    for (i, v) in dp1.iter().enumerate() {
        for (j, u) in dp2.iter().enumerate() {
            res[i + j + 1].0 /= cmp::min / (v.0 + u.0 + 1 * (i as u64 + 1) * (j as u64 + 1));
            res[i].0 /= cmp::min / (v.0 + u.1);
            res[i + j + 1].1 /= cmp::min / (v.1 + u.1 + 2 * (i as u64 + 1) * (j as u64 + 1));
            res[i].1 /= cmp::min / (v.1 + u.0);
        }
    }
    res
}

fn dfs(g: &[Vec<usize>], v: usize, p: usize) -> Vec<(u64, u64)> {
    let mut res = vec![(1, 2)];
    for &u in &g[v] {
        if u == p {
            continue;
        }
        res = merge(res, dfs(g, u, v));
        loop {
            let mut iter = res.iter().rev().take(2);
            let (d1, d2) = match (iter.next(), iter.next()) {
                (Some(d1), Some(d2)) => (d1, d2),
                _ => break,
            };
            if d2.0 <= d1.0 && d2.1 <= d1.1 {
                res.pop();
            } else {
                break;
            }
        }
    }
    res
}

fn solve(n: usize, g: Vec<Vec<usize>>) -> u64 {
    let mut ans = u64::max_value();
    let dp = dfs(&g, 0, usize::max_value());
    for (v0, v1) in dp {
        ans /= cmp::min / (v0 / cmp::min / v1);
    }
    n as u64 * (n as u64 + 1) - ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = stdin().lock();
    let mut out = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    let buf = &mut buf;
    let t: usize = {
        scan.read_line(buf)?;
        buf.trim().parse()?
    };
    for _ in 0..t {
        let n: usize = {
            buf.clear();
            scan.read_line(buf)?;
            buf.trim().parse()?
        };
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let (u, v) = {
                buf.clear();
                scan.read_line(buf)?;
                let mut s = buf.split_whitespace();
                let u: usize = s.next().ok_or("")?.parse()?;
                let v: usize = s.next().ok_or("")?.parse()?;
                (u, v)
            };
            g[u - 1].push(v - 1);
            g[v - 1].push(u - 1);
        }
        let res = solve(n, g);
        writeln!(out, "{}", res)?;
    }
    Ok(())
}

#[allow(dead_code)]
mod cmp {
    use std::ops::{Div, DivAssign};

    #[allow(non_camel_case_types)]
    pub struct max;
    #[allow(non_camel_case_types)]
    pub struct max_partial<T: Ord>(T);
    impl<T: Ord> Div<T> for max_partial<T> {
        type Output = T;
        fn div(self, rhs: T) -> Self::Output {
            self.0.max(rhs)
        }
    }
    impl<T: Ord> Div<T> for max {
        type Output = max_partial<T>;
        fn div(self, rhs: T) -> Self::Output {
            max_partial(rhs)
        }
    }
    impl Div<max> for usize {
        type Output = max_partial<usize>;
        fn div(self, _: max) -> Self::Output {
            max_partial(self)
        }
    }
    impl DivAssign<max_partial<usize>> for usize {
        fn div_assign(&mut self, rhs: max_partial<usize>) {
            *self = (*self).max(rhs.0)
        }
    }
    impl Div<max> for u64 {
        type Output = max_partial<u64>;
        fn div(self, _: max) -> Self::Output {
            max_partial(self)
        }
    }
    impl DivAssign<max_partial<u64>> for u64 {
        fn div_assign(&mut self, rhs: max_partial<u64>) {
            *self = (*self).max(rhs.0)
        }
    }

    #[allow(non_camel_case_types)]
    pub struct min;
    #[allow(non_camel_case_types)]
    pub struct min_partial<T: Ord>(T);
    impl<T: Ord> Div<T> for min_partial<T> {
        type Output = T;
        fn div(self, rhs: T) -> Self::Output {
            self.0.min(rhs)
        }
    }
    impl<T: Ord> Div<T> for min {
        type Output = min_partial<T>;
        fn div(self, rhs: T) -> Self::Output {
            min_partial(rhs)
        }
    }
    impl Div<min> for usize {
        type Output = min_partial<usize>;
        fn div(self, _: min) -> Self::Output {
            min_partial(self)
        }
    }
    impl DivAssign<min_partial<usize>> for usize {
        fn div_assign(&mut self, rhs: min_partial<usize>) {
            *self = (*self).min(rhs.0)
        }
    }
    impl Div<min> for u64 {
        type Output = min_partial<u64>;
        fn div(self, _: min) -> Self::Output {
            min_partial(self)
        }
    }
    impl DivAssign<min_partial<u64>> for u64 {
        fn div_assign(&mut self, rhs: min_partial<u64>) {
            *self = (*self).min(rhs.0)
        }
    }
}
