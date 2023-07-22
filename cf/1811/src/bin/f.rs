#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::error::Error;
use std::io::*;
use std::num::*;
use std::result::Result;

fn input1() -> Result<u32, Box<dyn Error>> {
    let mut s = String::new();
    stdin().read_line(&mut s)?;
    let m = s.trim().parse::<u32>()?;
    Ok(m)
}

fn input2() -> Result<(u32, u32), Box<dyn Error>> {
    let mut s = String::new();
    stdin().read_line(&mut s)?;
    let v = s.trim().split(' ').collect::<Vec<_>>();
    let m = v.first().ok_or("")?.parse::<u32>()?;
    let n = v.get(1).ok_or("")?.parse::<u32>()?;
    Ok((m, n))
}

fn pass() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    stdin().read_line(&mut s)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let m = input1()?;

    for _ in 0..m {
        solve()?;
    }

    Ok(())
}

#[inline]
fn no() -> Result<(), Box<dyn Error>> {
    println!("NO");
    Ok(())
}

trait C {
    fn count(&self, n: &u32) -> u32;
}

impl C for Vec<Vec<u32>> {
    #[inline]
    fn count(&self, n: &u32) -> u32 {
        self[*n as usize].len() as u32
    }
}

fn solve() -> Result<(), Box<dyn Error>> {
    pass()?;
    let (nv, ne) = input2()?;

    let x = (nv as f64).sqrt() as u32;
    if x * x != nv || x <= 2 {
        for _ in 0..ne {
            pass()?;
        }
        return no();
    }

    let g = {
        let mut g = vec![vec![]; nv as usize];
        for _ in 0..ne {
            let (v1, v2) = input2()?;
            g[v1 as usize - 1].push(v2 - 1);
            g[v2 as usize - 1].push(v1 - 1);
        }
        g
    };

    for n in &g {
        let l = n.len();
        if l != 2 && l != 4 {
            return no();
        }
    }

    let mut visited = HashSet::new();
    let mut first = None;
    let mut flag = true;

    for (i, n) in g.iter().enumerate() {
        let i = i as u32;

        if visited.contains(&i) || g.count(&i) == 2 {
            continue;
        }

        visited.insert(i);

        if flag {
            first = Some(i);
            flag = false;
        }

        if !check(&g, &mut visited, i, n, x, 2) {
            return no();
        }
    }

    let first = match first {
        Some(first) => first,
        _ => return no(),
    };
    let mut visited = HashSet::new();
    visited.insert(first);
    let n = &g[first as usize];

    if !check(&g, &mut visited, first, n, x, 4) {
        return no();
    }

    println!("YES");
    Ok(())
}

fn check(
    g: &Vec<Vec<u32>>,
    visited: &mut HashSet<u32>,
    i: u32,
    n: &Vec<u32>,
    x: u32,
    count: u32,
) -> bool {
    let mut m = n;

    for _ in 0..x - 1 {
        // find a neighbour with count not in cache
        let k = {
            let mut k = None;
            for node in m {
                if visited.contains(node) {
                    continue;
                }
                if g.count(node) == count {
                    k = Some(*node);
                    break;
                }
            }
            match k {
                Some(k) => k,
                _ => return false,
            }
        };
        m = &g[k as usize];
        visited.insert(k);
    }

    // if not cycle return false
    if m.iter().all(|node| *node != i) {
        return false;
    }

    true
}
