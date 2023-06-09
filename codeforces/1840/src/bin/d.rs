fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    let t: usize = io.read()?;
    for _ in 0..t {
        let n: usize = io.read()?;
        let mut v: Vec<u64> = io.read_vec(n)?;

        if n <= 3 {
            io.print(0)?;
            continue;
        }

        v.sort();

        // TL
        // let mut ans = (v[n - 1] - v[0] + 1) / 2;
        // for i in 1..n - 1 {
        //     for j in i + 1..n {
        //         let max1 = (v[i - 1] - v[0] + 1) / 2;
        //         let max2 = (v[j - 1] - v[i] + 1) / 2;
        //         let max3 = (v[n - 1] - v[j] + 1) / 2;
        //         ans = ans.min(max1.max(max2).max(max3))
        //     }
        // }

        let min = v[0];
        let max = v[n - 1];
        let mut l = 0;
        let mut r = 1_000_000_000;
        while r > l + 1 {
            let m = (r + l) / 2 - 1;
            let i = v
                .iter()
                .enumerate()
                .skip(1)
                .find(|(_, v)| *v - min > 2 * m)
                .map(|(i, _)| i)
                .unwrap_or(n - 1);
            let j = v
                .iter()
                .enumerate()
                .rev()
                .find(|(_, v)| max - *v > 2 * m)
                .map(|(i, _)| i)
                .unwrap_or(0);
            if i > j || v[j] - v[i] <= 2 * m {
                r = m + 1;
            } else {
                l = m + 1;
            }
        }
        io.print(l)?;
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

        pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Box<dyn Error>> {
            self.scan.read_line(&mut self.buf)?;
            let mut s = std::mem::take(&mut self.buf);
            s.truncate(n);
            Ok(s.into_bytes())
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
