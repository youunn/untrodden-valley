#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut a, mut b): (u64, u64) = input2()?;

    let mut i = 0;
    loop {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                i += b / a;
                b %= a;
                if b == 0 {
                    i -= 1;
                    break;
                }
            }
            std::cmp::Ordering::Greater => {
                i += a / b;
                a %= b;
                if a == 0 {
                    i -= 1;
                    break;
                }
            }
            std::cmp::Ordering::Equal => break,
        }
    }

    println!("{}", i);
    Ok(())
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
