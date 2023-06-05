const M: u64 = 1_000_000_007;
type N = number::Number<M>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    let t: usize = io.read()?;
    for _ in 0..t {
        let (n, m, k): (u64, u64, u64) = io.read3()?;
        io.read_line()?; // suppose elements in v are all one

        // for prefix `a` whose len is `j` in arrays whose len is `i`
        // dp count all possible arrays for every `i` and `j`

        let mut ans = N::new(k).pow(m);
        let mut c: N = 1.into();
        for i in 0..n as u64 {
            ans -= c * N::new(k - 1).pow(m - i);
            c *= N::new(m - i) / (i + 1).into()
        }
        io.print(ans.0)?;
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

        pub fn pow(mut self, mut b: u64) -> Self {
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

#[allow(dead_code)]
mod io {
    use std::{
        error::Error,
        fmt::{Arguments, Display},
        io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
        str::FromStr,
    };

    pub fn default<'a>() -> IO<'a> {
        Default::default()
    }

    macro_rules! read_impl {
        ( $f:ident, $($t:ident),+ ) => {
            #[allow(unused_parens)]
            pub fn $f<$($t),+>(&mut self) -> Result<($($t),+), Box<dyn std::error::Error>>
            where
                $($t: std::str::FromStr,
                <$t>::Err: std::error::Error + 'static,)+
            {
                self.scan.read_line(&mut self.buf)?;
                let s = std::mem::take(&mut self.buf);
                let mut s = s.split_whitespace();
                Ok(($(s.next().ok_or("")?.parse::<$t>()?),+))
            }
        };
    }

    pub struct IO<'a> {
        scan: StdinLock<'a>,
        out: BufWriter<StdoutLock<'a>>,
        buf: String,
    }

    impl<'a> IO<'a> {
        pub fn new() -> Self {
            Self {
                scan: stdin().lock(),
                out: BufWriter::new(stdout().lock()),
                buf: String::new(),
            }
        }

        pub fn read_line(&mut self) -> Result<String, Box<dyn Error>> {
            self.scan.read_line(&mut self.buf)?;
            Ok(std::mem::take(&mut self.buf))
        }

        pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Box<dyn Error>> {
            self.scan.read_line(&mut self.buf)?;
            let mut s = std::mem::take(&mut self.buf);
            s.truncate(n);
            Ok(s.into_bytes())
        }

        pub fn read<T>(&mut self) -> Result<T, Box<dyn std::error::Error>>
        where
            T: FromStr,
            <T>::Err: Error + 'static,
        {
            self.scan.read_line(&mut self.buf)?;
            Ok(std::mem::take(&mut self.buf).trim().parse::<T>()?)
        }

        read_impl!(read2, T1, T2);
        read_impl!(read3, T1, T2, T3);

        pub fn read_vec<T, V>(&mut self, n: usize) -> Result<V, Box<dyn Error>>
        where
            T: FromStr,
            <T>::Err: Error + 'static,
            V: FromIterator<T>,
        {
            self.scan.read_line(&mut self.buf)?;
            let v = std::mem::take(&mut self.buf)
                .split_whitespace()
                .take(n)
                .map(&str::parse)
                .collect::<Result<V, _>>()?;
            Ok(v)
        }

        pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
            self.out.write_fmt(fmt)
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

    impl<'a> Default for IO<'a> {
        fn default() -> Self {
            IO::new()
        }
    }
}
