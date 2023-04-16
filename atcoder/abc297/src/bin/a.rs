#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, d): (u8, u32) = input2()?;
    if n == 1 {
        print!("-1");
        return Ok(());
    }
    let vt: Vec<u32> = input_vec(n as usize)?;
    let mut l = vt[0];
    for i in 1..n {
        let c = vt[i as usize];
        if c - l <= d {
            print!("{}", c);
            return Ok(());
        }
        l = c;
    }
    print!("-1");
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
