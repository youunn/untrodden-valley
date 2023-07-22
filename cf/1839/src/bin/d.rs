fn chmax<T: Ord>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    for _ in 0..io.read::<usize>()? {
        let n = io.read::<usize>()?;
        let mut v = vec![0];
        v.extend(io.read_vec::<usize, Vec<_>>(n)?);
        v.push(n + 1);

        // max count of fixed balls among first i balls
        // where i is fixed and the others formed j segments
        let mut dp = vec![vec![Option::<usize>::None; n + 2]; n + 2];
        dp[0][0] = Some(0);
        // next fixed ball
        for i in 1..n + 2 {
            // current last fixed ball
            for j in 0..i {
                if v[j] > v[i] {
                    continue;
                }
                for k in 0..n + 2 {
                    let x = match dp[j][k] {
                        Some(x) => Some(x + 1),
                        _ => continue,
                    };
                    if i - j > 1 {
                        chmax(&mut dp[i][k + 1], x);
                    } else {
                        chmax(&mut dp[i][k], x);
                    }
                }
            }
        }

        let mut ans = Vec::with_capacity(n);
        for k in 1..=n {
            let x = dp[n + 1][k - 1];
            chmax(&mut dp[n + 1][k], x);
            let x = dp[n + 1][k].ok_or("won't be none")?;
            ans.push(n + 1 - x);
        }

        io.print_vec(ans.into_iter())?;
    }
    Ok(())
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
            values: impl Iterator<Item = T>,
        ) -> Result<(), Box<dyn Error>> {
            for v in values {
                write!(self.out, "{} ", v)?;
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
