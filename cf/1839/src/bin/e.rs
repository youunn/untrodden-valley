#![allow(unreachable_code)]

fn split(v: &[usize], n: usize) -> Option<Vec<bool>> {
    let sum = v.iter().sum::<usize>();
    if sum % 2 == 1 {
        return None;
    }

    let target = sum / 2;
    let mut dp = vec![None; target + 1];
    dp[0] = Some(usize::MAX);

    for (i, &a) in v.iter().enumerate() {
        for s in (a..=target).rev() {
            if dp[s].is_none() && dp[s - a].is_some() {
                dp[s] = Some(i);
            }
        }
    }

    if dp[target].is_none() {
        return None;
    }

    let mut res = vec![false; n];
    let mut c = target;

    while c > 0 {
        if let Some(i) = dp[c] {
            res[i] = true;
            c -= v[i];
        } else {
            // unreachable
            break;
        }
    }

    Some(res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    let n = io.read::<usize>()?;
    let mut v = io.read_vec::<usize, Vec<_>>(n)?;

    if let Some(p) = split(&v, n) {
        io.print("Second")?;
        io.flush()?;

        loop {
            let m1 = io.read::<isize>()?;
            if m1 <= 0 {
                return Ok(());
            }
            let m1 = m1 as usize - 1;

            let i = v
                .iter()
                .enumerate()
                .find(|&(i, &a)| a != 0 && p[i] != p[m1])
                .map(|(i, _)| i)
                .ok_or("won't lose")?;
            io.print(i + 1)?;
            io.flush()?;

            let d = v[i].min(v[m1]);
            v[i] -= d;
            v[m1] -= d;
        }
    } else {
        io.print("First")?;
        io.flush()?;

        loop {
            // print any one
            let i = v
                .iter()
                .enumerate()
                .find(|&(_, &a)| a != 0)
                .map(|(i, _)| i)
                .ok_or("won't lose")?;
            io.print(i + 1)?;
            io.flush()?;

            let m2 = io.read::<isize>()?;
            if m2 <= 0 {
                return Ok(());
            }
            let m2 = m2 as usize - 1;

            let d = v[i].min(v[m2]);
            v[i] -= d;
            v[m2] -= d;
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

        pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<(), Box<dyn Error>> {
            Ok(self.out.write_fmt(fmt)?)
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

        pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
            Ok(self.out.flush()?)
        }
    }

    impl<'a> Default for IO<'a> {
        fn default() -> Self {
            IO::new()
        }
    }
}
