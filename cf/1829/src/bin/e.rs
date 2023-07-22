#[derive(Clone, Copy, Default, Debug)]
struct Info {
    parent_i: usize,
    parent_j: usize,
    depth: u32,
    is_water: bool,
}

fn find_root(grid: &mut Vec<Vec<Info>>, i: usize, j: usize) -> (usize, usize) {
    if grid[i][j].parent_i == i && grid[i][j].parent_j == j {
        (i, j)
    } else {
        (grid[i][j].parent_i, grid[i][j].parent_j) =
            find_root(grid, grid[i][j].parent_i, grid[i][j].parent_j);
        (grid[i][j].parent_i, grid[i][j].parent_j)
    }
}

fn merge(grid: &mut Vec<Vec<Info>>, i0: usize, j0: usize, i1: usize, j1: usize) {
    let (i0, j0) = find_root(grid, i0, j0);
    let (i1, j1) = find_root(grid, i1, j1);
    if i0 != i1 || j0 != j1 {
        let (pi1, pj1) = (grid[i1][j1].parent_i, grid[i1][j1].parent_j);
        (grid[i1][j1].parent_i, grid[i1][j1].parent_j) = (i0, j0);
        grid[i0][j0].depth += std::mem::take(&mut grid[pi1][pj1].depth);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::IO::new();

    for _t in 0..io.read::<usize>()? {
        let (n, m): (usize, usize) = io.read2()?;
        let mut grid = vec![vec![Info::default(); m + 1]; n + 1];
        for i in 1..=n {
            let v: Vec<u32> = io.read_vec()?;
            for j in 1..=m {
                let depth = v[j - 1];

                if depth == 0 {
                    continue;
                }

                grid[i][j].is_water = true;
                grid[i][j].parent_i = i;
                grid[i][j].parent_j = j;
                grid[i][j].depth = depth;

                if grid[i - 1][j].is_water {
                    merge(&mut grid, i - 1, j, i, j);
                }

                if grid[i][j - 1].is_water {
                    merge(&mut grid, i, j - 1, i, j);
                }
            }
        }
        io.print(
            grid.into_iter()
                .map(|dp| {
                    dp.into_iter()
                        .filter(|x| x.depth != 0)
                        .map(|x| x.depth)
                        .max()
                        .unwrap_or_default()
                })
                .max()
                .unwrap_or_default(),
        )?;
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
