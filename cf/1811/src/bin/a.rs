#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let m = input::<u32>()?;
    for _ in 0..m {
        let (l, d) = {
            let mut v = String::new();
            std::io::stdin().read_line(&mut v)?;
            let v: Vec<&str> = v.trim().split(' ').collect();
            (v[0].parse::<u32>()?, v[1].parse::<char>()?)
        };
        let s = {
            let mut v = String::new();
            std::io::stdin().read_line(&mut v)?;
            Vec::from(v)
        };

        let mut flag = true;

        for i in 0..l {
            let c = s[i as usize] as char;
            if flag && c < d {
                print!("{}", d);
                flag = false;
            }
            print!("{}", c);
        }

        if flag {
            print!("{}", d);
        }

        println!();
    }

    Ok(())
}

#[inline]
fn input<T>() -> std::result::Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    <T>::Err: std::error::Error + 'static,
{
    let mut v = String::new();
    std::io::stdin().read_line(&mut v)?;
    let result = v.trim().parse::<T>()?;
    Ok(result)
}

#[inline]
fn input_vec<T>(n: usize) -> std::result::Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    <T>::Err: std::error::Error + 'static,
{
    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let v: Vec<&str> = v.trim().split(' ').collect();
    let mut result = Vec::with_capacity(n);
    for e in v {
        let element = e.parse::<T>()?;
        result.push(element);
    }
    Ok(result)
}
