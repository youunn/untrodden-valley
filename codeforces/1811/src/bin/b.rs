#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
    input! {
        m: u32,
    }

    macro_rules! abs {
        ($x:expr, $c:expr) => {
            if $x <= $c { $c - $x + 1 } else { $x - $c }
        };
    }

    for _ in 0..m {
        input! {
            t: u32,
            x1: u32,
            y1: u32,
            x2: u32,
            y2: u32,
        }

        let c = t / 2;

        let x1 = abs!(x1, c);
        let y1 = abs!(y1, c);
        let x2 = abs!(x2, c);
        let y2 = abs!(y2, c);

        let level1 = std::cmp::max(x1, y1);
        let level2 = std::cmp::max(x2, y2);
        let res = level1.abs_diff(level2);
        println!("{}", res);
    }
}
