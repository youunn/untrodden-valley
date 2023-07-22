type Res = Result<(), Box<dyn std::error::Error>>;

fn solve(io: &mut io::IO) -> Res {
    let n = io.read::<usize>()?;
    let v = io.read_vec::<_, Vec<u64>>(n)?;

    // closest smaller value on right
    let mut v1 = vec![None; n];
    let mut s = Vec::<usize>::new();
    for i in (0..n).rev() {
        while !s.is_empty() {
            let last = *s.last().ok_or("v1")?;
            if v[i] >= v[last] {
                break;
            }
            s.pop();
            if v1[last].is_none() {
                v1[last] = Some(i);
            }
        }
        s.push(i);
    }

    // closest smaller value on left
    let mut v2 = vec![None; n];
    s.clear();
    for i in 0..n {
        while !s.is_empty() {
            let last = *s.last().ok_or("v2")?;
            if v[i] >= v[last] {
                break;
            }
            s.pop();
            if v2[last].is_none() {
                v2[last] = Some(i);
            }
        }
        s.push(i);
    }

    // closest greater value on right
    let mut v3 = vec![None; n];
    s.clear();
    let mut s2 = Vec::<usize>::new();
    for i in (0..n).rev() {
        while !s2.is_empty() {
            let last = *s2.last().ok_or("v3")?;
            if v[i] <= v[last] {
                break;
            }
            s2.pop();
            if v3[last].is_none() {
                v3[last] = Some(i);
            }
        }

        while !s.is_empty() {
            let last = *s.last().ok_or("s")?;
            if v[i] >= v[last] {
                break;
            }
            s.pop();
            if v3[last].is_none() {
                s2.push(last);
            }
        }

        s.push(i);
    }

    let mut res = 0;
    for i in 0..n {
        let x1 = (v1[i].map(|x| x as u64 + 1)).unwrap_or_default();
        let x2 = (v2[i].map(|x| x as u64)).unwrap_or(n as u64);
        let x3 = (v3[i].map(|x| x as u64 + 1)).unwrap_or_default();
        let i = i as u64;
        res += (x1 - x3) * (x2 - i);
    }

    let n = n as u64;
    // let total = (n - 1) * n * (n + 1) / (3 * 2 * 1);
    let total = (n * n * n - n) / 6;

    io.print(total - res)?;

    Ok(())
}

fn main() -> Res {
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
        io::{BufRead, BufWriter, StdinLock, StdoutLock, Write, stdin, stdout},
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
