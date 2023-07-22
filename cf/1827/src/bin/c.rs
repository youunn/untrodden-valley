type Void = Result<(), Box<dyn std::error::Error>>;

fn solve1(n: usize, s: Vec<u8>) -> usize {
    let mut l = 0;
    let mut r = 0;
    let mut w = vec![0; n];
    for i in 0..n {
        w[i] = if i >= r {
            0
        } else {
            std::cmp::min(w[l + r - i], r - i)
        };
        while i + w[i] < n && i >= w[i] + 1 && s[i + w[i]] == s[i - w[i] - 1] {
            w[i] += 1;
        }
        if i + w[i] > r {
            l = i - w[i];
            r = i + w[i];
        }
    }

    let log = std::mem::size_of::<usize>() * 8 - n.leading_zeros() as usize;
    let mut rmq = vec![vec![0; n]; log];
    for i in 0..n {
        rmq[0][i] = i - w[i];
    }
    for k in 1..log {
        let mut i = 0;
        while i + (1 << k) <= n {
            rmq[k][i] = rmq[k - 1][i].min(rmq[k - 1][i + (1 << (k - 1))]);
            i += 1;
        }
    }

    let mut count = vec![0; n + 1];
    for i in 0..n {
        let mut j = i + 1;
        for k in (0..log).rev() {
            if (j + (1 << k) <= n) && rmq[k][j] > i {
                j += 1 << k;
            }
        }
        if j * 2 - i <= n {
            count[j * 2 - i] += count[i] + 1;
        }
    }

    count.into_iter().sum()
}

fn solve(io: &mut io::IO) -> Void {
    let n = io.read::<usize>()?;
    let mut s = io.read_line()?.into_bytes();
    s.truncate(n);
    let res = solve1(n, s);
    io.print(res)?;

    Ok(())
}

fn main() -> Void {
    let mut io = io::IO::new();
    let t = io.read::<usize>()?;
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
