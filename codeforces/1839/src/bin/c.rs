fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    for _ in 0..io.read::<usize>()? {
        let n = io.read::<usize>()?;
        let v: Vec<_> = io
            .read_line()?
            .split_whitespace()
            .map(|s| s.as_bytes())
            .map(|s| s[0])
            .collect();

        if v[n - 1] == b'1' {
            writeln!(io, "NO")?;
        } else {
            let mut ans = vec![];
            let mut iter = v.into_iter();
            let mut a = iter.next();
            while a.is_some() {
                let mut v = vec![];
                while let Some(c) = a {
                    a = iter.next();
                    if c == b'0' {
                        v.push(v.len());
                        break;
                    }
                    v.push(0);
                }
                ans.push(v);
            }

            writeln!(io, "YES")?;
            io.print_vec(ans.into_iter().rev().flatten())?;
        }
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
