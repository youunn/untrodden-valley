fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::IO::new();
    const N: usize = 2030;
    let mut dp2d = vec![vec![0; N]; N];
    let mut cur = 0u64;
    for i in 0..N {
        for j in 0..=i {
            cur += 1;
            dp2d[i][j] = cur * cur;
        }
    }
    for i in 1..N {
        for j in 0..i {
            dp2d[i][j] += dp2d[i - 1][j];
        }
    }
    for i in 0..N {
        for j in 0..i {
            dp2d[i][j + 1] += dp2d[i - 1][j];
        }
    }
    let mut dp1d = vec![0; N * N / 2];
    let mut cur = 0;
    for i in 0..N {
        for j in 0..i {
            dp1d[cur] = dp2d[i - 1][j];
            cur += 1;
        }
    }

    for _ in 0..io.read::<usize>()? {
        let n = io.read::<usize>()?;
        io.print(dp1d[n - 1])?;
    }
    Ok(())
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

        pub fn read_vec<T, V>(&mut self) -> Result<V, Box<dyn Error>>
        where
            T: FromStr,
            <T>::Err: Error + 'static,
            V: FromIterator<T>,
        {
            let mut s: String = String::new();
            self.scan.read_line(&mut s)?;
            let v = s
                .split_whitespace()
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
            values: impl Iterator<Item = T>,
        ) -> Result<(), Box<dyn Error>> {
            for v in values {
                write!(self.out, "{} ", v)?;
            }
            writeln!(self.out)?;
            Ok(())
        }
    }
}
