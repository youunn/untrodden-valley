#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn solve(
    fl: u32,
    fr: u32,
    es: Vec<((u32, u32), (u32, u32))>,
    dsu: &mut Daiyousei,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = dsu.history_len();
    dsu.update_leader0();
    let mut esl = Vec::new();
    let mut esr = Vec::new();
    let fm = (fl + fr) / 2;

    for e in es {
        if e.1 .0 <= fl && e.1 .1 >= fr {
            dsu.merge((e.0.0 as usize, e.0.1 as usize));
            continue;
        }

        if e.1 .0 <= fm {
            esl.push(e);
        }

        if e.1 .1 > fm {
            esr.push(e);
        }
    }

    if fl != fr {
        solve(fl, fm, esl, dsu)?;
        solve(fm + 1, fr, esr, dsu)?;
    }

    let end = dsu.history_len();
    for _ in start..end {
        dsu.unmerge()?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, m) = input2::<u32, u32>()?;

    let mut vs = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let (l, r) = input2::<u32, u32>()?;
        vs.push((l, r));
    }

    let mut es = Vec::with_capacity(m as usize);
    for _ in 0..m {
        let (a, b) = input2::<u32, u32>()?;
        let a = a - 1;
        let b = b - 1;
        let l = std::cmp::max(vs[a as usize].0, vs[b as usize].0);
        let r = std::cmp::min(vs[a as usize].1, vs[b as usize].1);
        if l > r {
            continue;
        }
        es.push(((a, b), (l, r)));
    }

    if es.is_empty() {
        print!("1 ");
        return Ok(());
    }

    let node = vs[0];
    let mut dsu = Daiyousei::new(n as usize);

    solve(node.0, node.1, es, &mut dsu)?;
    for (i, &x) in dsu.result.iter().enumerate() {
        if x {
            print!("{} ", i + 1);
        }
    }

    Ok(())
}

struct Daiyousei {
    n: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
    result: Vec<bool>,
    history: Vec<(usize, usize, bool)>,
    leader0: usize,
}

impl Daiyousei {
    pub fn new(n: usize) -> Self {
        let mut result = vec![false; n];
        result[0] = true;
        Self {
            n,
            parent: (0..n).collect(),
            size: vec![1; n],
            result,
            history: Vec::new(),
            leader0: 0,
        }
    }

    pub fn update_leader0(&mut self) {
        self.leader0 = self.leader(0);
    }

    pub fn merge(&mut self, (a, b): (usize, usize)) -> bool {
        let mut a = self.leader(a);
        let mut b = self.leader(b);
        if a == b {
            false
        } else {
            // 写反小于号D了一晚上TL
            if self.size[a] < self.size[b] {
                std::mem::swap(&mut a, &mut b);
            }
            self.history.push((b, a, self.result[a]));
            self.parent[b] = a;
            self.size[a] += self.size[b];

            if self.leader0 == b {
                self.leader0 = a;
                self.result[a] = true;
            } else if self.leader0 == a {
                // do nothing
            } else if self.result[a] {
                self.result[a] = false;
            }
            true
        }
    }

    pub fn unmerge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (b, a, flag) = self.history.pop().ok_or("")?;
        self.size[a] -= self.size[b];
        self.parent[b] = b;
        if self.result[a] {
            self.result[b] = true;
        } else if flag {
            self.result[a] = true;
        }
        Ok(())
    }

    pub fn leader(&self, mut a: usize) -> usize {
        while a != self.parent[a] {
            a = self.parent[a];
        }
        a
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

#[inline]
fn input2<T1, T2>() -> Result<(T1, T2), Box<dyn std::error::Error>>
where
    T1: std::str::FromStr,
    <T1>::Err: std::error::Error + 'static,
    T2: std::str::FromStr,
    <T2>::Err: std::error::Error + 'static,
{
    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let v: Vec<&str> = v.trim().split(' ').collect();
    let first = v[0].parse::<T1>()?;
    let second = v[1].parse::<T2>()?;
    Ok((first, second))
}
