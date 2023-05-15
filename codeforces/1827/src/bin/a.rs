use io::IO;
use number::Number as N;

const M: u64 = 1_000_000_007;
const MAXN: usize = 200_000;
type Res = Result<(), Box<dyn std::error::Error>>;

fn solve(io: &mut IO) -> Res {
    let n = io.read::<usize>()?;
    let mut va = io.read_vec::<_, Vec<u64>>(n)?;
    let mut vb = io.read_vec::<_, Vec<u64>>(n)?;
    va.sort();
    va.sort_by_key(|x| std::cmp::Reverse(*x));
    vb.sort_by_key(|x| std::cmp::Reverse(*x));

    // 2 4 5 6 8 9
    // 1 1 3 4 5 6
    // 32
    let mut ans = N::new(1);
    let mut c = 0;
    for (i, b) in vb.iter().enumerate() {
        while c < n && &va[c] > b {
            c += 1;
        }
        if c <= i {
            io.print(0)?;
            return Ok(());
        }
        ans *= N::new((c - i) as u64);
    }
    io.print(ans.0)?;

    Ok(())
}

fn main() -> Res {
    let mut io = IO::new();
    for _ in 0..io.read::<usize>()? {
        solve(&mut io)?;
    }
    Ok(())
}

#[allow(dead_code)]
mod comb {
    use super::number::Number as N;
    use super::MAXN;

    macro_rules! get_or_init {
        (|| -> $t:ty $init:block) => {{
            static mut VALUE: std::mem::MaybeUninit<$t> = std::mem::MaybeUninit::uninit();
            static ONCE: std::sync::Once = std::sync::Once::new();
            ONCE.call_once(|| {
                let v = $init;
                unsafe {
                    VALUE.write(v);
                }
            });
            unsafe { VALUE.assume_init_ref() }
        }};
    }

    pub fn p(n: u64) -> N {
        let fact = get_or_init!(|| -> Vec<N> {
            let mut fact: Vec<_> = Vec::with_capacity(MAXN + 1);
            fact.push(N::new(1));
            for i in 1..=MAXN {
                fact[i] = fact[i - 1] * N::new(i as u64);
            }
            fact
        });
        fact[n as usize]
    }

    pub fn c(n: u64, k: u64) -> N {
        if n < k {
            return N::new(0);
        }

        p(n) / (p(k) * p(n - k))
    }
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
            self * rhs.inv()
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
