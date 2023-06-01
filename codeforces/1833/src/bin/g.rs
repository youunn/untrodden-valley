type Unit = Result<(), Box<dyn std::error::Error>>;

fn dfs(graph: &[Vec<(usize, usize)>], start: usize, parent: usize, ans: &mut Vec<usize>) -> usize {
    // count children of subtree after cutting all branch (mod 3)
    let mut count1 = 0;
    let mut count2 = 0;

    for &(edge, next) in &graph[start] {
        if next == parent {
            continue;
        }
        match dfs(graph, next, start, ans) {
            1 => count1 += 1,
            2 => count2 += 1,
            0 => ans.push(edge),
            _ => return usize::MAX,
        }
    }

    match (count1, count2) {
        (0, 0) | (0, 1) | (1, 0) | (2, 0) => (count1 * 1 + count2 * 2 + 1) % 3,
        _ => usize::MAX,
    }
}

fn solve(io: &mut io::IO) -> Unit {
    let n = io.read::<usize>()?;

    if n % 3 != 0 {
        for _ in 1..n {
            io.read_line()?;
        }
        io.print(-1)?;
        return Ok(());
    }

    let mut g = vec![vec![]; n];
    for i in 1..n {
        let (u, v) = io.read2::<usize, usize>()?;
        let (u, v) = (u - 1, v - 1);
        g[u].push((i, v));
        g[v].push((i, u));
    }

    let mut ans = vec![];
    if dfs(&g, 0, usize::MAX, &mut ans) == 0 {
        io.print(ans.len())?;
        io.print_vec(ans.into_iter())?;
    } else {
        io.print(-1)?;
    }

    Ok(())
}

fn main() -> Unit {
    let mut io = io::IO::new();
    for _t in 0..io.read::<usize>()? {
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
