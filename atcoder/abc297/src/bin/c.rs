#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (h, w): (usize, usize) = input2()?;

    for _ in 0..h {
        let mut v = String::new();
        std::io::stdin().read_line(&mut v)?;
        let mut s = v.trim().to_string();
        let s = unsafe { s.as_bytes_mut() };

        let mut lc_is_t = false;

        for i in 0..w {
            let c = s[i];
            if lc_is_t {
                if c == b'T' {
                    s[i - 1] = b'P';
                    s[i] = b'C';
                    lc_is_t = false;
                } else {
                    lc_is_t = false;
                }
            } else if c == b'T' {
                lc_is_t = true;
            }
        }

        println!("{}", String::from_utf8(s.to_vec())?);
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
