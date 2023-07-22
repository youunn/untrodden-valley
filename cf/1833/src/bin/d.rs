type Unit = Result<(), Box<dyn std::error::Error>>;

fn solve(io: &mut io::IO) -> Unit {
    let n = io.read::<usize>()?;
    let v = io.read_vec::<usize, Vec<_>>(n)?;

    if n == 1 {
        io.print_vec(v.iter())?;
        return Ok(());
    }

    let max = v
        .iter()
        .enumerate()
        .skip(1)
        .max_by(|a, b| a.1.cmp(b.1))
        .ok_or("")?;
    let j = if max.0 == n - 1 { n } else { max.0 };

    let first = &v[0];
    let i = if j == 1 {
        j - 1
    } else {
        v.iter()
            .take(j - 1)
            .enumerate()
            .rev()
            .find(|x| x.1 <= first)
            .map(|x| x.0)
            .unwrap_or(0)
    };

    io.print_vec(
        (v[j..].iter())
            .chain(v[i + 1..j].iter().rev())
            .chain(v[0..i + 1].iter()),
    )?;

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
