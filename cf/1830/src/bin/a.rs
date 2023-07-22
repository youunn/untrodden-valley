type Unit = Result<(), Box<dyn std::error::Error>>;
type U = usize;

fn dfs(
    graph: &[Vec<(U, U)>],
    edge_to_parent: &mut Vec<U>,
    path_to_root: &mut Vec<U>,
    current: U,
    parent: U,
) {
    for &(edge, child) in &graph[current] {
        if parent == child {
            continue;
        }
        edge_to_parent[child] = edge;
        path_to_root[child] = path_to_root[current];
        if edge < edge_to_parent[current] {
            path_to_root[child] += 1
        };
        dfs(graph, edge_to_parent, path_to_root, child, current);
    }
}

fn solve(io: &mut io::IO) -> Unit {
    let n = io.read::<U>()?;
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (u, v) = io.read2::<U, U>()?;
        g[u - 1].push((i, v - 1));
        g[v - 1].push((i, u - 1));
    }

    let mut edge_to_parent = vec![0; n];
    edge_to_parent[0] = U::MAX;
    let mut step_to_root = vec![0; n];
    dfs(
        &g,
        &mut edge_to_parent,
        &mut step_to_root,
        0,
        U::MAX,
    );

    io.print(step_to_root.into_iter().max().unwrap_or_default())?;

    Ok(())
}

fn main() -> Unit {
    let mut io = io::IO::new();
    for _ in 0..io.read::<U>()? {
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
