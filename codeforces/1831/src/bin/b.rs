type Unit = Result<(), Box<dyn std::error::Error>>;

fn count(v: Vec<usize>) -> std::collections::BTreeMap<usize, usize> {
    let mut m = std::collections::BTreeMap::<usize, usize>::new();
    let mut last = usize::MAX;
    let mut count = 0;
    for c in v {
        if c == last {
            count += 1;
        } else {
            if count != 0 {
                let e = m.entry(last).or_default();
                *e = (*e).max(count);
            }
            count = 1;
            last = c;
        }
    }
    if count != 0 {
        let e = m.entry(last).or_default();
        *e = (*e).max(count);
    }
    m
}

fn solve1(_n: usize, va: Vec<usize>, vb: Vec<usize>) -> usize {
    let mut max = 1;
    let (mut ia, mut ib) = (count(va).into_iter(), count(vb).into_iter());
    let (mut ea, mut eb) = (ia.next(), ib.next());

    loop {
        match (ea, eb) {
            (Some((ka, va)), Some((kb, vb))) => match ka.cmp(&kb) {
                std::cmp::Ordering::Greater => {
                    max = max.max(vb);
                    eb = ib.next();
                }
                std::cmp::Ordering::Less => {
                    max = max.max(va);
                    ea = ia.next();
                }
                std::cmp::Ordering::Equal => {
                    max = max.max(va + vb);
                    ea = ia.next();
                    eb = ib.next();
                }
            },
            (Some((_, va)), _) => {
                max = max.max(va);
                ea = ia.next();
            }
            (_, Some((_, vb))) => {
                max = max.max(vb);
                eb = ib.next();
            }
            _ => break,
        }
    }

    max
}

fn solve(io: &mut io::IO) -> Unit {
    let n: usize = io.read()?;
    let a: Vec<usize> = io.read_vec(n)?;
    let b: Vec<usize> = io.read_vec(n)?;
    let res: usize = solve1(n, a, b);
    io.print(res)?;
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
