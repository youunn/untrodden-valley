#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

#[inline]
fn yes() -> Result<(), Box<dyn std::error::Error>> {
    println!("YES");
    Ok(())
}

#[inline]
fn no() -> Result<(), Box<dyn std::error::Error>> {
    println!("NO");
    Ok(())
}

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (n, k) = input2::<usize, u64>()?;
    let len = n * n;
    let v = {
        let mut v = Vec::with_capacity(len);
        for _ in 0..n {
            let line = input_vec::<u8>(n)?;
            v.extend(line);
        }
        v
    };

    let mut difference_count = 0;

    for i in 0..n * n / 2 {
        if v[i] != v[len - 1 - i] {
            difference_count += 1;
        }
    }

    if difference_count > k {
        no()
    } else if n % 2 == 1 || (k - difference_count) % 2 == 0 {
        yes()
    } else {
        no()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m: u16 = input()?;

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
