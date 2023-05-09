const M: u64 = 1_000_000_007;

fn solve(io: &mut io::IO) -> Result<(), Box<dyn std::error::Error>> {
    // a <= 111111
    let (n, k): (usize, u32) = io.read2()?;
    let v: Vec<usize> = io.read_vec()?;
    let mut dp = vec![vec![0u64; 64]; n + 1];
    for i in 1..=n {
        let x = v[i - 1];
        for j in 0..64 {
            dp[i][j] += dp[i - 1][j];
            dp[i][j] %= M;
            dp[i][j & x] += dp[i - 1][j];
            dp[i][j & x] %= M;
        }
        dp[i][x] += 1;
        dp[i][x] %= M;
    }
    let mut ans = 0;
    for j in 0..64 {
        if (j as u8).count_ones() == k {
            ans += dp[n][j];
            ans %= M;
        }
    }
    io.print(ans)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::IO::new();
    let t: u32 = io.read()?;
    for _ in 0..t {
        solve(&mut io)?;
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
