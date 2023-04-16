#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
    input! {
        m: u32,
    }

    fn f(n: u32) -> (u32, u32) {
        if n == 1 {
            return (1, 1);
        }
        let last = f(n - 1);
        (last.1, last.0 + last.1)
    }
    

    for _ in 0..m {
        input! {
            n: u32,
            mut x: u32,
            mut y: u32,
        }

        let (mut w, mut h) = f(n + 1);
        let mut flag = true;

        loop {
            if w == 1 {
                break;
            }

            let w1 = h - w;
            let h1 = w;

            if y > h - w && y <= w {
                print!("NO");
                println!();
                flag = false;
                break;
            } else if y > w {
                let tmp = x;
                x = y - w;
                y = tmp;
            } else {
                std::mem::swap(&mut x, &mut y);
            }

            h = h1;
            w = w1;
        }

        if flag {
            print!("YES");
            println!();
        }
    }
}
