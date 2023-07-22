#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

#[inline]
fn input3() -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
    let v = input_vec::<u32>(3)?;
    let n = v[0];
    let s1 = v[1];
    let s2 = v[2];
    Ok((n, s1, s2))
}

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (n, s1, s2) = input3()?;
    let xs = input_vec::<u32>(n as usize)?;
    let mut xs = xs.iter().enumerate().collect::<Vec<_>>();
    xs.sort_by(|a, b| b.1.cmp(a.1)); // descend

    // a = 1 x1 + 2 x2 + 3 x3 + ...
    // b = 1 x4 + 2 x5 + 3 x6 + ...
    // s = s1 * a + s2 * b
    // find argmin(s)
    // sort (1,2,3,...)(s1,s2)

    let mut m1 = (0, s1);
    let mut m2 = (0, s2);
    let mut ss1 = Vec::new();
    let mut ss2 = Vec::new();
    let mut c1 = 0;
    let mut c2 = 0;

    for i in 0..n {
        if m1.1 <= m2.1 {
            ss1.push(i as usize);
            m1.0 += 1;
            m1.1 += s1;
            c1 += 1;
        } else {
            ss2.push(i as usize);
            m2.0 += 1;
            m2.1 += s2;
            c2 += 1;
        }
    }

    // println!("---");
    // println!("{:?}", &xs);
    // println!("{:?}", &ss1);
    // println!("{:?}", &ss2);
    // println!("---");

    print!("{}", c1);
    for s in ss1 {
        print!(" {}", xs[s].0 + 1);
    }
    println!();

    print!("{}", c2);
    for s in ss2 {
        print!(" {}", xs[s].0 + 1);
    }
    println!();

    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let m = input::<u32>()?;

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
    let result = v.trim().parse::<T>()?;
    Ok(result)
}

#[inline]
fn input_vec<T>(n: usize) -> Result<Vec<T>, Box<dyn std::error::Error>>
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
