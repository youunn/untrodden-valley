type Unit = Result<(), Box<dyn std::error::Error>>;
const M: u64 = 1_000_000_007;

fn dfs(g: &[Vec<usize>], i: usize, size: &mut [usize], vs: &mut [bool]) {
    size[i] = 1;
    vs[i] = true;
    for &v in &g[i] {
        if !vs[v] {
            dfs(g, v, size, vs);
            size[i] += size[v];
        }
    }
}

fn c(n: u64, k: u64, fact: &[u64]) -> u64 {
    if n < k {
        return 0;
    }
    div(
        fact[n as usize],
        mul(fact[k as usize], fact[n as usize - k as usize]),
    )
}

fn add(x: u64, y: u64) -> u64 {
    (x + y) % M
}

fn mul(x: u64, y: u64) -> u64 {
    x * y % M
}

fn pow(mut a: u64, mut b: u64) -> u64 {
    let mut ans = 1;
    a %= M;
    while b > 0 {
        if b % 2 == 1 {
            ans *= a;
            ans %= M;
        }
        a *= a;
        a %= M;
        b /= 2;
    }
    ans
}

fn inv(x: u64) -> u64 {
    pow(x, M - 2)
}

fn div(x: u64, y: u64) -> u64 {
    x * inv(y) % M
}

fn solve(io: &mut io::IO) -> Unit {
    let (n, k): (usize, usize) = io.read2()?;
    let mut fact: Vec<_> = (0..=n as u64).collect();
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = mul(fact[i], fact[i - 1]);
    }

    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (u, v): (usize, usize) = io.read2()?;
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    if k % 2 == 1 {
        io.print(1)?;
        return Ok(());
    }

    let mut size = vec![0; n];
    let mut vs = vec![false; n];
    dfs(&g, 0, &mut size, &mut vs);

    let mut ans = 1;
    let (n, k) = (n as u64, k as u64);
    for x in size {
        let x = x as u64;
        ans = add(
            ans,
            div(
                mul(c(x, k / 2, &fact), c(n - x, k / 2, &fact)),
                c(n, k, &fact),
            ),
        );
    }

    io.print(ans)?;

    Ok(())
}

fn main() -> Unit {
    let mut io = io::IO::new();
    solve(&mut io)?;
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
