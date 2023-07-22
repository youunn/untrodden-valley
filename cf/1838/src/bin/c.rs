fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    let t: usize = io.read()?;

    let mut prime = vec![false; 1001];
    prime[2] = true;
    'o: for o in 3..=1000 {
        for i in 2..o {
            if prime[i] {
                if o % i == 0 {
                    continue 'o;
                }
            }
        }
        prime[o] = true;
    }

    for _ in 0..t {
        let (n, m): (usize, usize) = io.read2()?;
        if !prime[n] {
            for i in 0..n {
                write!(io, "{}", i + 1)?;
                for j in 1..m {
                    write!(io, " {}", j * n + i + 1)?;
                }
                writeln!(io)?;
            }
            writeln!(io)?;
            continue;
        }
        if !prime[m] {
            for i in 0..n {
                write!(io, "{}", i * m + 1)?;
                for j in 1..m {
                    write!(io, " {}", i * m + j + 1)?;
                }
                writeln!(io)?;
            }
            writeln!(io)?;
            continue;
        }
        assert!(n > 2 && m > 2);
        if n <= m {
            for i in 0..n {
                write!(io, "{}", i * m + i + 1)?;
                for j in 1..m - i {
                    write!(io, " {}", i * m + j + i + 1)?;
                }
                for j in m - i..m {
                    write!(io, " {}", i * m + j - m + i + 1)?;
                }
                writeln!(io)?;
            }
            writeln!(io)?;
            continue;
        }

        for i in 0..n {
            write!(io, "{}", i + 1)?;
            for j in 1..m {
                if i + j < n {
                    write!(io, " {}", j * n + i + 1 + j)?;
                } else {
                    write!(io, " {}", j * n + i + 1 + j - n)?;
                }
            }
            writeln!(io)?;
        }
        writeln!(io)?;
        continue;
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
