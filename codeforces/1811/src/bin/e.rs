#![allow(unused_imports)]
use std::collections::*;

fn main() {
    let mut v1 = String::new();
    std::io::stdin().read_line(&mut v1).unwrap();
    let v0: Vec<&str> = v1.trim().split(' ').collect();
    let v2 = v0.len();
    let m = v0[(0) as usize].parse::<u32>().unwrap();

    fn solve(n: u64) -> u64 {
        let mut x = down(n);

        let mut ans = 0_u64;
        let mut num = 0_u32;
        let mut res = 0_u64;
        loop {
            let a: u8 = (x % 10) as u8;
            let b: u8;
            let c: u8;
            match a.cmp(&4) {
                std::cmp::Ordering::Equal => {
                    unreachable!();
                }
                std::cmp::Ordering::Greater => {
                    b = a - 1;
                    c = 1;
                }
                std::cmp::Ordering::Less => {
                    b = a;
                    c = 0;
                }
            }
            res += ans * b as u64;
            let w = 10u64.pow(num);
            res += w * c as u64;
            x /= 10;
            if x == 0 {
                return res;
            }
            ans = 9 * ans + w;
            num += 1;
        }
    }

    fn check(n: u64) -> u32 {
        let mut x = n;
        let mut num = 1;
        loop {
            let a: u8 = (x % 10) as u8;
            if a == 4 {
                return num;
            }
            x /= 10;
            num += 1;
            if x == 0 {
                return 0;
            }
        }
    }

    fn down(n: u64) -> u64 {
        let mut x = n;
        loop {
            let num = check(x);
            if num == 0 {
                return x;
            }
            let num = num - 1;
            let w = 10u64.pow(num);
            x = x / w * w - 1;
        }
    }

    fn up(n: u64) -> u64 {
        let mut x = n;
        loop {
            let num = check(x);
            if num == 0 {
                return x;
            }
            let num = num - 1;
            let w = 10u64.pow(num);
            x = x / w * w + w;
        }
    }

    for _ in 0..m {
        let mut v1 = String::new();
        std::io::stdin().read_line(&mut v1).unwrap();
        let v0: Vec<&str> = v1.trim().split(' ').collect();
        let v2 = v0.len();
        let n = v0[(0) as usize].parse::<u64>().unwrap();

        let mut left = n;
        let mut right = n * 10;
        let mut solution = (left + right) / 2;
        solution = down(solution);
        loop {
            let count = solve(solution);
            let m = solution - count;
            match m.cmp(&n) {
                std::cmp::Ordering::Equal => {
                    println!("{}", solution);
                    break;
                }
                std::cmp::Ordering::Greater => {
                    right = solution;
                }
                std::cmp::Ordering::Less => {
                    left = solution;
                }
            }
            let m = (left + right) / 2;
            let d = down(m);
            if solution == d {
                solution = up(m);
            } else {
                solution = d;
            }
        }
    }
}
