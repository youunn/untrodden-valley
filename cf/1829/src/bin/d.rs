fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MAXN: u32 = 10_000_002;
    let mut fs = std::collections::HashMap::<u32, u32>::new();
    let mut i = 1;
    let mut cur = 3;
    while cur < MAXN {
        fs.insert(cur, i);
        cur *= 3;
        i += 1;
    }

    let mut io = io::IO::new();
    for _ in 0..io.read::<u16>()? {
        let (mut n, mut m): (u32, u32) = io.read2()?;

        let nz = n.trailing_zeros();
        let mz = m.trailing_zeros();
        n = n >> nz;
        m = m >> mz;

        if m > n || nz > mz {
            io.print("NO")?;
            continue;
        }
        if m == n && mz == nz {
            io.print("YES")?;
            continue;
        }
        if n % m != 0 {
            io.print("NO")?;
            continue;
        }

        let d = match fs.get(&(n / m)) {
            Some(d) => *d,
            None => {
                io.print("NO")?;
                continue;
            }
        };

        if d < mz - nz {
            io.print("NO")?;
            continue;
        }

        io.print("YES")?;
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
