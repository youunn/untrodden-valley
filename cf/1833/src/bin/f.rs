use std::collections::BTreeMap;

use number::Number as N;

type Unit = Result<(), Box<dyn std::error::Error>>;
const M: u64 = 1_000_000_007;

fn solve(io: &mut io::IO) -> Unit {
    let (n, m) = io.read2::<usize, usize>()?;
    let mut v = io.read_vec::<u64, Vec<_>>(n)?;
    v.sort();

    let mut bm = BTreeMap::<u64, usize>::new();
    let mut last_key = v[0];
    let mut last = bm.entry(last_key).or_default();
    for i in v {
        if i == last_key {
            *last += 1;
        } else {
            last_key = i;
            last = bm.entry(last_key).or_insert(1);
        }
    }

    let mut iter = bm.iter();
    let mut ans = N(0);
    let mut count = N(1);
    let mut next = iter.next();
    'outer: while let Some(start) = next {
        let mut curr = *start.0;
        count *= N(*start.1 as u64);
        for _ in 1..m {
            let next1 = iter.next();
            if let Some((i, c)) = next1 {
                if *i != curr + 1 {
                    next = next1;
                    count = N(1);
                    continue 'outer;
                } else {
                    curr += 1;
                    count *= N(*c as u64); 
                }
            } else {
                break 'outer;
            }
        }

        ans += count;

        loop {
            let next1 = iter.next();
            if let Some((i, c)) = next1 {
                if *i != curr + 1 {
                    next = next1;
                    count = N(1);
                    continue 'outer;
                } else {
                    curr += 1;
                    count /= N(bm[&(curr - m as u64)] as u64);
                    count *= N(*c as u64); 
                    ans += count;
                }
            } else {
                break 'outer;
            }
        }
    }

    io.print(ans.0)?;

    Ok(())
}

fn main() -> Unit {
    let mut io = io::IO::new();
    for _ in 0..io.read::<usize>()? {
        solve(&mut io)?;
    }
    Ok(())
}

#[allow(dead_code)]
mod number {
    use super::M;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(Clone, Copy)]
    pub struct Number(pub u64);

    impl Number {
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

    impl Add for Number {
        type Output = Number;
        fn add(self, rhs: Self) -> Self::Output {
            Self((self.0 + rhs.0) % M)
        }
    }
    impl Sub for Number {
        type Output = Number;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut lhs = self.0 % M;
            let rhs = rhs.0 % M;
            if lhs < rhs {
                lhs += M;
            }
            Self((lhs - rhs) % M)
        }
    }
    impl Mul for Number {
        type Output = Number;
        fn mul(self, rhs: Self) -> Self::Output {
            Self((self.0 * rhs.0) % M)
        }
    }
    impl Div for Number {
        type Output = Number;
        fn div(self, rhs: Self) -> Self::Output {
            Mul::mul(self, rhs.inv())
        }
    }

    impl AddAssign for Number {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Number {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    impl MulAssign for Number {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }
    impl DivAssign for Number {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs
        }
    }
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
                write!(self.out, "{} ", v)?;
                for v in values {
                    write!(self.out, "{} ", v)?;
                }
            }
            writeln!(self.out)?;
            Ok(())
        }
    }
}
