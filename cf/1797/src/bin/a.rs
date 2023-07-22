#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (n, m) = {
        let v = input_vec::<u64>(2)?;
        (v[0], v[1])
    };

    let (x1, y1, x2, y2) = {
        let v = input_vec::<u64>(4)?;
        (v[0], v[1], v[2], v[3])
    };

    // seal the block near the wall
    // if any cornor, print 2
    // if any wall, print 3
    // else 4
    let minx1 = std::cmp::min(x1, n + 1 - x1);
    let miny1 = std::cmp::min(y1, m + 1 - y1);
    let minx2 = std::cmp::min(x2, n + 1 - x2);
    let miny2 = std::cmp::min(y2, m + 1 - y2);
    if std::cmp::max(minx1, miny1) == 1 || std::cmp::max(minx2, miny2) == 1 {
        println!("2");
    } else if std::cmp::min(minx1, miny1) == 1 || std::cmp::min(minx2, miny2) == 1 {
        println!("3");
    } else {
        println!("4");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m: u32 = input()?;

    for _ in 0..m {
        solve()?;
    }

    Ok(())
}

#[inline]
fn input<T>() -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    <T>::Err: std::error::Error + 'static,
{
    let mut v = String::new();
    std::io::stdin().read_line(&mut v)?;
    let x = v.trim().parse()?;
    Ok(x)
}

#[inline]
fn input_vec<T>(n: usize) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    <T>::Err: std::error::Error + 'static,
{
    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let xs = v
        .trim()
        .split(' ')
        .take(n)
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    Ok(xs)
}
