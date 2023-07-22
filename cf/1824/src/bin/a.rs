type Unit = Result<(), Box<dyn std::error::Error>>;

fn solve(io: &mut io::IO) -> Unit {
    let (_n, m): (u32, u32) = io.read2()?;
    let mut c1 = 0u32;
    let mut c2 = 0u32;
    let mut v: Vec<u32> = io
        .read_vec::<i32, Vec<_>>()?
        .into_iter()
        .filter(|&a| {
            if a == -1 {
                c1 += 1;
            } else if a == -2 {
                c2 += 1;
            } else {
                return true;
            }
            false
        })
        .map(|a| a as u32)
        .collect();
    if v.len() == 0 {
        io.print(c1.max(c2).min(m))?;
        return Ok(());
    }
    v.sort();
    v.dedup();
    let len = v.len() as u32;
    let mut ans = m.min(c1.max(c2) + len);
    for (i, a) in v.into_iter().enumerate() {
        let l = a - 1 - i as u32;
        let r = m - a - (len - 1 - i as u32);
        ans = ans.max(l.min(c1) + r.min(c2) + len);
    }

    io.print(ans)?;
    Ok(())
}

fn main() -> Unit {
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
