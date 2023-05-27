use std::collections::HashSet;

type Unit = Result<(), Box<dyn std::error::Error>>;

fn solve(io: &mut io::IO) -> Unit {
    let n = io.read::<usize>()?;
    let v = io.read_vec::<usize, Vec<_>>(n)?;
    let mut g = vec![HashSet::new(); n];
    for (i, j) in v.iter().enumerate() {
        g[i].insert(j - 1);
        g[j - 1].insert(i);
    }

    fn dfs(
        graph: &[HashSet<usize>],
        visited: *mut HashSet<usize>,
        start: usize,
        circles: &mut usize,
    ) {
        unsafe {
            (*visited).insert(start);
        }
        let mut neighbors = graph[start]
            .iter()
            .filter(|x| unsafe { !(*visited).contains(x) });
        if let Some(next) = neighbors.next() {
            let mut next = next;
            loop {
                dfs(graph, visited, *next, circles);
                next = match neighbors.next() {
                    Some(next) => next,
                    None => break,
                }
            }
        } else if graph[start].len() > 1 {
            *circles += 1;
        }
    }

    let mut circles = 0;
    let mut count = 0;
    let mut visited = HashSet::new();
    for i in 0..n {
        if !visited.contains(&i) {
            dfs(&g, &mut visited as *mut _, i, &mut circles);
            count += 1;
        }
    }

    io.print(format!(
        "{} {}",
        if count - circles == 0 { 0 } else { 1 } + circles,
        count,
    ))?;

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
