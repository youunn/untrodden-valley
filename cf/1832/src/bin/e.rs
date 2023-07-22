use std::collections::VecDeque;

use number::{MutNumber, Number};

type Unit = Result<(), Box<dyn std::error::Error>>;
type N = Number<998_244_353>;
type M = MutNumber;

fn solve(n: usize, a: u64, x: u64, y: u64, m: u64, k: usize) -> u64 {
    let mut va = Vec::with_capacity(n);
    va.push(M::new(a, m));
    let x = M::new(x, m);
    let y = M::new(y, m);
    for i in 1..n {
        va.push(va[i - 1] * x + y);
    }

    let mut vb: VecDeque<N> = va.into_iter().map(|a| a.value.into()).collect();

    for i in 0..=k {
        vb.push_front(0.into());
        for i in 1..=n + i {
            vb[i] = vb[i] + vb[i - 1];
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans ^= vb[i + 1].0 * i as u64;
    }

    ans
}

fn main() -> Unit {
    let mut io = io::default();
    let (n, a, x, y, m, k): (usize, u64, u64, u64, u64, usize) = io.read6()?;
    let res: u64 = solve(n, a, x, y, m, k);
    writeln!(io, "{}", res)?;
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

    #[derive(Clone, Copy)]
    pub struct MutNumber {
        pub value: u64,
        modulus: u64,
    }

    impl MutNumber {
        pub fn new(value: u64, modulus: u64) -> Self {
            Self {
                value: value % modulus,
                modulus,
            }
        }

        fn pow(mut self, mut b: u64) -> Self {
            let mut ans = Self {
                value: 1,
                modulus: self.modulus,
            };
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
            self.pow(self.modulus - 2)
        }
    }

    impl Add for MutNumber {
        type Output = MutNumber;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.value + rhs.value, self.modulus)
        }
    }
    impl Sub for MutNumber {
        type Output = MutNumber;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut lhs = self.value % self.modulus;
            let rhs = rhs.value % self.modulus;
            if lhs < rhs {
                lhs += self.modulus;
            }
            Self::new(lhs - rhs, self.modulus)
        }
    }
    impl Mul for MutNumber {
        type Output = MutNumber;
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.value * rhs.value, self.modulus)
        }
    }
    impl Div for MutNumber {
        type Output = MutNumber;
        fn div(self, rhs: Self) -> Self::Output {
            Mul::mul(self, rhs.inv())
        }
    }

    impl AddAssign for MutNumber {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for MutNumber {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    impl MulAssign for MutNumber {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }
    impl DivAssign for MutNumber {
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
        read_impl!(read6, T1, T2, T3, T4, T5, T6);

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
