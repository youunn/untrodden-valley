use std::{
    cmp::Ordering::{Equal, Greater, Less},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = io::default();
    let n = io.read::<usize>()?;

    let mut s: Vec<(usize, usize)> = Vec::with_capacity(n * n + 1);
    for i in 1..=n {
        let iter: Box<dyn Iterator<Item = usize>> = if i % 2 == 0 {
            Box::new((1..=n).rev())
        } else {
            Box::new(1..=n)
        };
        for j in iter {
            s.push((i, j));
        }
    }
    s.push((n + 1, if n % 2 == 1 { n } else { 1 }));

    let mut grid = vec![vec![b' '; n + 2]; n + 2];

    for (c, n) in s.iter().zip(s.iter().skip(1)) {
        grid[c.0][c.1] = match (n.0.cmp(&c.0), n.1.cmp(&c.1)) {
            (Less, _) => b'^',
            (Greater, _) => b'v',
            (_, Less) => b'<',
            (_, Greater) => b'>',
            _ => unreachable!(),
        };
    }

    fn query(
        index: usize,
        n: usize,
        s: &mut [(usize, usize)],
        grid: &mut [Vec<u8>],
        io: &mut io::IO,
    ) -> Result<(usize, usize, u8), Box<dyn Error>> {
        let p = s[index];
        writeln!(io, "? {} {}", p.0, p.1)?;
        for line in grid.iter().skip(1).take(n) {
            for c in line.iter().skip(1).take(n) {
                write!(io, "{}", *c as char)?;
            }
            writeln!(io)?;
        }
        io.flush()?;

        let s = io.read_line()?;
        if s.as_bytes().first() == Some(&b'-') {
            return Ok((0, 0, b' '));
        }
        let mut s = s.split_whitespace();
        let i: usize = s.next().ok_or("")?.parse()?;
        let j: usize = s.next().ok_or("")?.parse()?;
        let pi = if i == 0 {
            1
        } else if i > n {
            i - 1
        } else {
            i
        };
        let pj = if j == 0 {
            1
        } else if j > n {
            j - 1
        } else {
            j
        };

        Ok((
            pi,
            pj,
            match (i.cmp(&pi), j.cmp(&pj)) {
                (Less, _) => b'^',
                (Greater, _) => b'v',
                (_, Less) => b'<',
                (_, Greater) => b'>',
                _ => unreachable!(),
            },
        ))
    }

    let mut ans = query(0, n, &mut s, &mut grid, &mut io)?;

    if ans.2 != b' ' {
        // reverse snake
        s.pop();
        s.reverse();
        s.push((0, 1));

        for (c, n) in s.iter().zip(s.iter().skip(1)) {
            grid[c.0][c.1] = match (n.0.cmp(&c.0), n.1.cmp(&c.1)) {
                (Less, _) => b'^',
                (Greater, _) => b'v',
                (_, Less) => b'<',
                (_, Greater) => b'>',
                _ => unreachable!(),
            };
        }

        ans = query(0, n, &mut s, &mut grid, &mut io)?;
    }

    if ans.2 == b' ' {
        let mut index = 0;
        for i in (0..14).rev() {
            let i = index + (1 << i);
            if i >= n * n {
                continue;
            }
            let ans = query(i, n, &mut s, &mut grid, &mut io)?;
            if ans.2 == b' ' {
                index = i;
            }
        }

        (ans.0, ans.1) = s[index];

        for i in 1..=n {
            match i.cmp(&ans.0) {
                Less => {
                    for j in 1..=n {
                        grid[i][j] = b'^';
                    }
                }
                Greater => {
                    for j in 1..=n {
                        grid[i][j] = b'v';
                    }
                }
                Equal => {
                    for j in 1..=n {
                        grid[i][j] = if j < ans.1 { b'<' } else { b'>' };
                    }
                }
            }
        }

        ans.2 = query(index, n, &mut s, &mut grid, &mut io)?.2;
    }

    writeln!(io, "! {} {} {}", ans.0, ans.1, ans.2 as char)?;
    io.flush()?;

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
