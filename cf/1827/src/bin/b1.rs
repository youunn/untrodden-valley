fn solve(n: usize, v: Vec<u64>) -> u64 {
    let mut left = vec![None; n];
    let mut s = vec![];
    for (i, &a) in v.iter().enumerate().rev() {
        loop {
            let last = match s.last() {
                Some(last) => *last,
                None => break,
            };
            if a >= v[last] {
                break;
            }
            s.pop();
            left[last] = Some(i);
        }
        s.push(i);
    }

    let mut right = vec![None; n];
    s.clear();
    for (i, &a) in v.iter().enumerate() {
        loop {
            let last = match s.last() {
                Some(last) => *last,
                None => break,
            };
            if a >= v[last] {
                break;
            }
            s.pop();
            right[last] = Some(i);
        }
        s.push(i);
    }

    let rmq = collections::SparseTable::new(&v, std::cmp::max);

    let mut ans = 0;
    for i in 1..n {
        let l = match left[i] {
            Some(l) => l,
            _ => continue,
        };
        let r = right[i].unwrap_or(n);
        let ll = rmq.find_left_greater(0, l, v[i]);
        let ll = match ll {
            Some(ll) => l - ll,
            _ => l + 1,
        };

        ans += (r - i) as u64 * ll as u64;
    }

    let n = n as u64;
    // let total = (n - 1) * n * (n + 1) / (3 * 2 * 1);
    (n * n * n - n) / 6 - ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{stdin, stdout, BufRead, BufWriter, Write};

    let mut r#in = stdin().lock();
    let mut out = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    let buf = &mut buf;

    let t: usize = {
        r#in.read_line(buf)?;
        buf.trim().parse()?
    };

    for _ in 0..t {
        let n: usize = {
            buf.clear();
            r#in.read_line(buf)?;
            buf.trim().parse()?
        };

        let v: Vec<u64> = {
            buf.clear();
            r#in.read_line(buf)?;
            buf.split_whitespace()
                .map(&str::parse)
                .collect::<Result<_, _>>()?
        };

        writeln!(out, "{}", solve(n, v))?;
    }

    Ok(())
}

#[allow(dead_code)]
mod collections {
    pub struct SparseTable<T> {
        data: Vec<Vec<T>>,
        f: fn(T, T) -> T,
    }

    impl<T: Default + Copy + Clone> SparseTable<T> {
        pub fn new(v: &Vec<T>, f: fn(T, T) -> T) -> Self {
            let n = v.len();
            let l = usize::BITS - n.leading_zeros();
            let l = l as usize;
            let mut data = vec![vec![T::default(); l]; n];
            for i in 0..n {
                data[i][0] = v[i];
            }
            for j in 0..l - 1 {
                for i in 0..n - (1 << j) {
                    data[i][j + 1] = f(data[i][j], data[i + (1 << j)][j]);
                }
            }
            Self { data, f }
        }

        pub fn query(&self, l: usize, r: usize) -> T {
            let len = r - l;
            let k = usize::BITS - len.leading_zeros();
            let k = k as usize - 1;
            (self.f)(self.data[l][k], self.data[r - (1 << k)][k])
        }
    }

    impl<T: Ord + std::fmt::Debug> SparseTable<T> {
        pub fn find_left_greater(&self, mut l: usize, mut r: usize, base: T) -> Option<usize> {
            let len = r - l;
            if len == 0 {
                return None;
            }

            let mut k = (usize::BITS - len.leading_zeros()) as usize;

            loop {
                k -= 1;
                if self.data[r - (1 << k)][k] > base {
                    l = r - (1 << k);
                } else if self.data[l][k] > base {
                    r = l + (1 << k);
                } else {
                    return None;
                }

                if k == 0 {
                    break;
                }
            }

            Some(l)
        }
    }
}
