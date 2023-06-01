fn solve(n: usize, v: Vec<u64>) -> u64 {
    let min = rmq::SparseTable::new(&v, std::cmp::min);
    let max = rmq::SparseTable::new(&v, std::cmp::max);

    let mut ans = 0;
    for i in 0..n {
        let l = find_less_left(&min, 0, i, v[i]);
        if v[l] >= v[i] {
            continue;
        }
        let mut r = find_less_right(&min, i + 1, n, v[i]);
        if v[r] >= v[i] {
            r = n;
        }
        let mut ll = find_greater_left(&max, 0, l, v[i]);
        if v[ll] <= v[i] {
            ll = usize::MAX;
        }
        ans += (r - i) as u64 * (if ll == usize::MAX { l + 1 } else { l - ll }) as u64;
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
mod rmq {
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
}
