#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (h, w, k): (u32, u32, u32) = input3()?;

    // all conditions: c(hw,k)
    // select rectangle (h-i+1) * (w-j+1)
    // select corner 2-1: 2 * c(i-2,1) + 2 * c(j-2,)
    // select corner 2-2: c(hw-2) + c(hw-4)
    // select corner 1:
    // select corner 0:

    for i in 0..h {
        for j in 0..w {
            // corner
            // edge
            // center
        }
    }

    Ok(())
}

#[inline]
fn input3<T1, T2, T3>() -> Result<(T1, T2, T3), Box<dyn std::error::Error>>
where
    T1: std::str::FromStr,
    <T1>::Err: std::error::Error + 'static,
    T2: std::str::FromStr,
    <T2>::Err: std::error::Error + 'static,
    T3: std::str::FromStr,
    <T3>::Err: std::error::Error + 'static,
{
    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let v: Vec<&str> = v.trim().split(' ').collect();
    let first = v[0].parse::<T1>()?;
    let second = v[1].parse::<T2>()?;
    let third = v[2].parse::<T3>()?;
    Ok((first, second, third))
}
