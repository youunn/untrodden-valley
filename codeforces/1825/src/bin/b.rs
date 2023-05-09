type Res = Result<(), Box<dyn std::error::Error>>;

fn solve(io: &mut io::IO) -> Res {
    let (mut n, mut m): (i32, i32) = io.read2()?;
    let v: Vec<i32> = io.read_vec()?;
    let (mut a, mut b, mut c, mut d) = (
        i32::max_value(),
        i32::max_value(),
        i32::min_value(),
        i32::min_value(),
    );
    for x in v {
        if x < a {
            b = a;
            a = x;
        } else if x < b {
            b = x;
        }
        if x > d {
            c = d;
            d = x;
        } else if x > c {
            c = x;
        }
    }
    if m < n {
        std::mem::swap(&mut m, &mut n);
    }
    let ans = (d - a) * (m - 1) * n + (d - b).max(c - a) * (n - 1);
    io.print(ans)?;
    Ok(())
}

fn main() -> Res {
    let mut io = io::IO::new();
    for _ in 0..io.read::<u32>()? {
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
