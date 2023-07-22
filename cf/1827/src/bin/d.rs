use collections::Fenwick;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::IO::new();
    for _ in 0..io.read::<usize>()? {
        let n: usize = io.read::<usize>()?;
        let v: Vec<usize> = io.read_vec(n - 1)?;
        let res = solve(n, v);
        io.print_vec(res.into_iter())?;
    }
    Ok(())
}

fn solve(n: usize, v: Vec<usize>) -> Vec<usize> {
    let mut g = vec![vec![]; n];
    for (i, j) in v.iter().enumerate() {
        g[j - 1].push(i + 1);
    }

    let mut visited = Vec::with_capacity(n);
    let mut start = vec![0; n];
    let mut end = vec![0; n];
    // the whole tree
    dfs(&g, &mut visited, &mut start, &mut end, 0);

    // grow while querying
    let mut ft = Fenwick::<usize>::new(n);
    ft.add(0, 1);

    let mut current = 0;
    let mut max = 0;
    let mut ans = Vec::with_capacity(n - 1);

    for i in 1..n {
        ft.add(start[i], 1);
        if start[current] <= start[i] && end[current] >= end[i] {
            // find the child of current which is acestor of i
            let mut left = 0;
            let mut right = g[current].len() - 1;
            while left < right {
                let mid = (left + right) / 2;
                if end[g[current][mid]] < start[i] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            let child = g[current][left];
            let size = ft.query(end[child]) - ft.query(start[child] - 1);
            if size * 2 > i + 1 {
                // move down
                current = child;
            } else {
                max = max.max(size);
            }
        } else {
            let size = ft.query(end[current]) - ft.query(start[current] - 1);
            let other = i + 1 - size;
            if other * 2 > i + 1 {
                // move up
                current = v[current - 1] - 1;
            } else {
                max = max.max(other);
            }
        }
        ans.push(i + 1 - max * 2);
    }
    ans
}

fn dfs(
    graph: &[Vec<usize>],
    visited: &mut Vec<usize>,
    start: &mut Vec<usize>,
    end: &mut Vec<usize>,
    root: usize,
) {
    start[root] = visited.len();
    visited.push(root);
    for &c in &graph[root] {
        dfs(graph, visited, start, end, c);
    }
    end[root] = visited.len() - 1;
}

#[allow(dead_code)]
mod collections {
    use std::ops::{Add, AddAssign};

    pub struct Fenwick<T>(Vec<T>)
    where
        T: Add<Output = T> + AddAssign + Copy + Default;

    impl<T> Fenwick<T>
    where
        T: Add<Output = T> + AddAssign + Copy + Default,
    {
        pub fn new(n: usize) -> Self {
            Self(vec![T::default(); n + 1])
        }

        pub fn add(&mut self, i: usize, v: T) {
            let mut i = i + 1;
            let n = self.0.len();
            while i < n {
                self.0[i] += v;
                i += 1 << i.trailing_zeros();
            }
        }

        pub fn query(&self, i: usize) -> T {
            let mut i = i + 1;
            let mut res = T::default();
            while i > 0 {
                res += self.0[i];
                i -= 1 << i.trailing_zeros();
            }
            res
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     #[test]
    //     fn fen() {
    //         let mut f = super::Fenwick::new(5);
    //         f.add(0, 1);
    //         f.add(1, 2);
    //         f.add(2, 3);
    //         f.add(3, 4);
    //         f.add(4, 5);
    //         assert_eq!(1, f.query(0));
    //         assert_eq!(15, f.query(4));
    //     }
    // }
}

#[allow(dead_code)]
mod io {
    use std::{
        error::Error,
        fmt::Display,
        io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
        str::FromStr,
    };

    macro_rules! read_impl {
        ( $f:ident, $($t:ident),+ ) => {
            #[allow(unused_parens)]
            pub fn $f<$($t),+>(&mut self) -> Result<($($t),+), Box<dyn std::error::Error>>
            where
                $($t: std::str::FromStr,
                <$t>::Err: std::error::Error + 'static,)+
            {
                let mut s: String = String::new();
                self.scan.read_line(&mut s)?;
                let mut s = s.split_whitespace();
                Ok(($(s.next().ok_or("")?.parse::<$t>()?),+))
            }
        };
    }

    pub struct IO<'a> {
        scan: StdinLock<'a>,
        out: BufWriter<StdoutLock<'a>>,
    }

    impl<'a> IO<'a> {
        pub fn new() -> Self {
            Self {
                scan: stdin().lock(),
                out: BufWriter::new(stdout().lock()),
            }
        }

        read_impl!(read, T1);
        read_impl!(read2, T1, T2);
        read_impl!(read3, T1, T2, T3);

        pub fn read_line(&mut self) -> Result<String, Box<dyn Error>> {
            let mut s: String = String::new();
            self.scan.read_line(&mut s)?;
            Ok(s)
        }

        pub fn read_vec<T, V>(&mut self, n: usize) -> Result<V, Box<dyn Error>>
        where
            T: FromStr,
            <T>::Err: Error + 'static,
            V: FromIterator<T>,
        {
            let mut s: String = String::new();
            self.scan.read_line(&mut s)?;
            let v = s
                .split_whitespace()
                .take(n)
                .map(&str::parse)
                .collect::<Result<V, _>>()?;
            Ok(v)
        }

        pub fn print<T: Display>(&mut self, value: T) -> Result<(), Box<dyn Error>> {
            writeln!(self.out, "{}", value)?;
            Ok(())
        }

        pub fn print_vec<T: Display>(
            &mut self,
            mut values: impl Iterator<Item = T>,
        ) -> Result<(), Box<dyn Error>> {
            if let Some(v) = values.next() {
                write!(self.out, "{}", v)?;
                for v in values {
                    write!(self.out, " {}", v)?;
                }
            }
            writeln!(self.out)?;
            Ok(())
        }
    }
}
