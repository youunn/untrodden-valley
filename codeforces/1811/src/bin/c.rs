#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
    input! {
        m: u16,
    }

    for _ in 0..m {
        input! {
            n: u32,
            xs: [u32; n - 1],
        }

        let i = xs
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        let (first, second) = xs.split_at(i + 1);

        print!("{}", first[0]);

        for x in first.iter().skip(1) {
            print!(" {}", x);
        }

        print!(" {}", xs[i]);

        for x in second {
            print!(" {}", x);
        }

        println!()
    }
}
