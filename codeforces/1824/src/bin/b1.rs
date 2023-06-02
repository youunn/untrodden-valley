const M: u64 = 1_000_000_007;
type E = Box<dyn std::error::Error>;
type N = number::Number<M>;

fn main() -> Result<(), E> {
    use std::io::{stdin, stdout, BufRead, BufWriter, StdinLock, Write};
    let mut r#in = stdin().lock();
    let mut out = BufWriter::new(stdout().lock());
    let mut buf = String::new();

    fn read2(r#in: &mut StdinLock, buf: &mut String) -> Result<(usize, usize), E> {
        buf.clear();
        r#in.read_line(buf)?;
        let mut s = buf.split_whitespace();
        let u = s.next().map(&str::parse);
        let v = s.next().map(&str::parse);
        match (u, v) {
            (Some(Ok(u)), Some(Ok(v))) => Ok((u, v)),
            _ => Err("".into()),
        }
    }

    let (n, k) = read2(&mut r#in, &mut buf)?;
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (u, v) = read2(&mut r#in, &mut buf)?;
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    if k == 1 || k == 3 {
        writeln!(out, "{}", 1)?;
    } else {
        // k == 2
        fn dfs(g: &Vec<Vec<usize>>, s: &mut Vec<usize>, v: usize, p: usize) {
            for &u in &g[v] {
                if u == p {
                    continue;
                }
                dfs(g, s, u, v);
                s[v] += s[u];
            }
        }

        let mut s = vec![1; n];
        dfs(&g, &mut s, 0, usize::MAX);
        let n: N = (n as u64).into();
        let mut ans: N = 0.into();
        for x in s {
            // x -> p
            let x: N = (x as u64).into();
            ans += (n - x) * x;
        }
        let all = (n * n - n) / 2.into();
        // 大概是这样，因为所有边一定是连接的
        // Count(Node) = Count(Edge) + 1
        // E(Count(Node)) = E(Count(Edge)) + 1
        // 点的期望的和等于边的期望的和加一
        let ans = ans / all + 1.into();
        writeln!(out, "{}", ans.0)?;
    }

    Ok(())
}

#[allow(dead_code)]
mod number {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(Clone, Copy)]
    pub struct Number<const M: u64>(pub u64);

    impl<const M: u64> Number<M> {
        pub fn new(x: u64) -> Self {
            Self(x % M)
        }

        fn pow(mut self, mut b: u64) -> Self {
            let mut ans = Self(1);
            while b > 0 {
                if b % 2 == 1 {
                    ans *= self;
                }
                self *= self;
                b /= 2;
            }
            ans
        }

        fn inv(self) -> Self {
            self.pow(M - 2)
        }
    }

    impl<const M: u64> From<u64> for Number<M> {
        fn from(x: u64) -> Self {
            Self::new(x)
        }
    }

    impl<const M: u64> Add for Number<M> {
        type Output = Number<M>;
        fn add(self, rhs: Self) -> Self::Output {
            Self((self.0 + rhs.0) % M)
        }
    }
    impl<const M: u64> Sub for Number<M> {
        type Output = Number<M>;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut lhs = self.0 % M;
            let rhs = rhs.0 % M;
            if lhs < rhs {
                lhs += M;
            }
            Self((lhs - rhs) % M)
        }
    }
    impl<const M: u64> Mul for Number<M> {
        type Output = Number<M>;
        fn mul(self, rhs: Self) -> Self::Output {
            Self((self.0 * rhs.0) % M)
        }
    }
    impl<const M: u64> Div for Number<M> {
        type Output = Number<M>;
        fn div(self, rhs: Self) -> Self::Output {
            Mul::mul(self, rhs.inv())
        }
    }

    impl<const M: u64> AddAssign for Number<M> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<const M: u64> SubAssign for Number<M> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    impl<const M: u64> MulAssign for Number<M> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }
    impl<const M: u64> DivAssign for Number<M> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs
        }
    }
}
