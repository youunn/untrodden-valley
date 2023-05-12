use std::collections::HashMap;
use std::io::Write;

type Res = Result<(), Box<dyn std::error::Error>>;

fn find_cycle(g: &[Vec<usize>], n: usize, v: usize) -> Option<Vec<usize>> {
    let mut visited = vec![false; n];
    let mut parent = vec![n; n];
    let mut stack = vec![];
    stack.push(v);
    while let Some(c) = stack.pop() {
        visited[c] = true;

        if c == v && parent[v] != n {
            let mut res = vec![];
            let mut p = v;
            res.push(v);
            while parent[p] != v {
                p = parent[p];
                res.push(p);
            }
            res.push(v);
            return Some(res);
        }

        let p = parent[c];
        for &u in &g[c] {
            if (!visited[u] || u == v) && u != p {
                stack.push(u);
                parent[u] = c;
            }
        }
    }

    None
}

fn solve(io: &mut io::IO) -> Res {
    let (n, m) = io.read2::<usize, usize>()?;
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = io.read2::<usize, usize>()?;
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    // find cycle with some node connect 2 other nodes
    let mut 乆 = None;
    for (v, _) in g.iter().enumerate().filter(|(_, adj)| adj.len() >= 4) {
        let 〇 = find_cycle(&g, n, v);
        if 〇.is_some() {
            乆 = 〇;
            break;
        }
    }

    if let Some(〇) = 乆 {
        io.print("YES")?;
        let len = 〇.len();
        let map = 〇
            .iter()
            .enumerate()
            .take(len - 2)
            .skip(2)
            .map(|(i, v)| (v, i))
            .collect::<HashMap<_, _>>();
        let first = *〇.first().ok_or("first of cycle")?;
        let mut inside = vec![];
        let mut outside = vec![];
        for &v in &g[first] {
            if map.contains_key(&v) {
                inside.push(v);
            } else if v != 〇[1] && v != 〇[len - 2] {
                outside.push(v);
            }
        }
        if outside.len() >= 2 {
            io.print(len - 1 + 2)?;
            for i in 0..len - 1 {
                writeln!(io.out, "{} {}", 〇[i] + 1, 〇[i + 1] + 1)?;
            }
            writeln!(io.out, "{} {}", first + 1, outside[0] + 1)?;
            writeln!(io.out, "{} {}", first + 1, outside[1] + 1)?;
        } else {
            let max = inside.iter().max_by_key(|x| map[x]).ok_or("max of map")?;
            let i = map[max];
            let v = 〇[i];
            io.print(len - map[max] + 2)?;
            for i in i..len - 1 {
                writeln!(io.out, "{} {}", 〇[i] + 1, 〇[i + 1] + 1)?;
            }
            writeln!(io.out, "{} {}", first + 1, 〇[1] + 1)?;
            writeln!(io.out, "{} {}", first + 1, v + 1)?;
            writeln!(
                io.out,
                "{} {}",
                first + 1,
                if outside.is_empty() {
                    inside
                        .into_iter()
                        .filter(|&x| x != v)
                        .next()
                        .ok_or("first of inside")?
                } else {
                    outside.into_iter().next().ok_or("first of outside")?
                } + 1
            )?;
        }
    } else {
        io.print("NO")?;
    }

    Ok(())
}

fn main() -> Res {
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
                let mut s = String::new();
                self.scan.read_line(&mut s)?;
                let mut s = s.split_whitespace();
                Ok(($(s.next().ok_or("parse string")?.parse::<$t>()?),+))
            }
        };
    }

    pub struct IO<'a> {
        scan: StdinLock<'a>,
        pub out: BufWriter<StdoutLock<'a>>,
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
