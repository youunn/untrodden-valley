#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
struct JeweledPagoda([u64; 4]);

impl JeweledPagoda {
    pub fn new(x: &u64) -> Self {
        let x = *x;
        Self([0, x, x, x])
    }

    #[inline]
    fn min(a: u64, b: u64, c: u64) -> u64 {
        std::cmp::min(std::cmp::min(a, b), c)
    }
}

impl Default for JeweledPagoda {
    fn default() -> Self {
        Self([1 << 40; 4])
    }
}

impl Add for JeweledPagoda {
    type Output = JeweledPagoda;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            Self::min(
                self.0[0b00] + rhs.0[0b10],
                self.0[0b01] + rhs.0[0b00],
                self.0[0b01] + rhs.0[0b10],
            ),
            Self::min(
                self.0[0b00] + rhs.0[0b11],
                self.0[0b01] + rhs.0[0b01],
                self.0[0b01] + rhs.0[0b11],
            ),
            Self::min(
                self.0[0b10] + rhs.0[0b10],
                self.0[0b11] + rhs.0[0b00],
                self.0[0b11] + rhs.0[0b10],
            ),
            Self::min(
                self.0[0b10] + rhs.0[0b11],
                self.0[0b11] + rhs.0[0b01],
                self.0[0b11] + rhs.0[0b11],
            ),
        ])
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = input::<usize>()?;
    let es = input_vec::<u64>(n - 1)?;
    let q: u32 = input()?;

    // swap in a sub group max of min
    // cost = 2 * sum(es)
    // let g(x) = 2 * f(x)
    // result is g(x)
    // split in a edge like following
    // f(1234567) = min(f(12) + f(4567), f(123) + f(567), ...)
    // f(4567) = min(f(4) + f(567), f(45) + f(67))
    // f is some operation like adding several numbers, we can call it `fsum` or `<>`
    // where `<>` has direction
    // for tree
    //        0                         0
    //    0       0       <-        0       0
    //  0   0   0   7             0   0   0   1
    // 1 2 3 4 5 6               2 3 4 5 6 7
    // fsum(1234567) = {[(1 <> 2) <> (3 <> 4)] <> [(5 <> 6) <> 7]}
    // 0 for remove the edge on one end, there are 00, 01, 10, 11
    // then cannot fsum a and b where the right of a and the left of b are 0.
    // 00 = min(00 with 10 or 01 with 00 or 01 with 10)
    // 01 = min(00 with 11 or 01 with 01 or 01 with 11)
    // 10 = min(10 with 10 or 11 with 00 or 11 with 10)
    // 11 = min(10 with 11 or 11 with 01 or 11 with 11)
    // then for | 5     | 6     | 7     |
    //          | 0  x5 | 0  x6 | 0  x7 |
    //          | x5 x5 | x6 x6 | x7 x7 |
    //      and | 5 <> 6                                                               |
    //          | min(0+x6, x5+0, x5+x6)  min(0+x6, x5+x6, x5+x6) -> min(x5, x6) x6    |
    //          | min(x5+x6, x5+0, x5+x6) min(x5+x6, x5+x6, x5+x6)   x5          x5+x6 |

    let mut st = ShouToramaru::new(es.iter().map(JeweledPagoda::new).collect(), n - 1);

    for _ in 0..q {
        let (mut k, x) = input2::<usize, u64>()?;
        k -= 1;
        st.update(k, JeweledPagoda::new(&x));
        println!("{}", 2 * st.query_all().0[0b11]);
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
    let v: Vec<&str> = v.trim().split(' ').collect();
    let mut result = Vec::with_capacity(n);
    for e in v {
        let element = e.parse::<T>()?;
        result.push(element);
    }
    Ok(result)
}

struct ShouToramaru<T>
where
    T: Add<Output = T> + Default + Clone + Copy + std::fmt::Debug,
{
    tree: Vec<T>,
    n: usize,
    offset: usize,
}

impl<T> ShouToramaru<T>
where
    T: Add<Output = T> + Default + Clone + Copy + std::fmt::Debug,
{
    pub fn new(v: Vec<T>, n: usize) -> Self {
        // index from 1
        // won't has only one child
        let mut tree = vec![T::default(); n];
        let mut l = 1;
        while n >> l > 0 {
            l += 1;
        }
        let offset = (1 << l) - n;
        let (v1, v2) = v.split_at(n - offset);
        tree.extend(v2);
        tree.extend(v1);
        for i in (1..n).rev() {
            tree[i] = tree[Self::cl(i)] + tree[Self::cr(i)];
        }
        Self { tree, n, offset }
    }

    pub fn update(&mut self, i: usize, value: T) {
        let j = (i + self.offset) % self.n;
        let index = self.n + j;
        self.tree[index] = value;
        let mut p = Self::p(index);
        while p != 0 {
            self.tree[p] = self.tree[Self::cl(p)] + self.tree[Self::cr(p)];
            p = Self::p(p);
        }
    }

    // [i, j)
    // WARN: process offset
    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        let mut ans = T::default();
        l += self.n;
        r += self.n;
        while l < r {
            if Self::is_l(l) {
                ans = ans + self.tree[l];
                l += 1;
            }
            if Self::is_l(r) {
                r -= 1;
                ans = ans + self.tree[r];
            }
            l = Self::p(l);
            r = Self::p(r);
        }
        ans
    }

    pub fn query_all(&self) -> T {
        self.tree[1]
    }

    #[inline]
    fn p(i: usize) -> usize {
        i / 2
    }

    #[inline]
    fn cl(i: usize) -> usize {
        i * 2
    }

    #[inline]
    fn cr(i: usize) -> usize {
        i * 2 + 1
    }

    #[inline]
    fn is_l(i: usize) -> bool {
        i % 2 == 1
    }

    #[inline]
    fn is_r(i: usize) -> bool {
        i % 2 == 0
    }
}
