fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = io::default();
    let (n, k, x, m): (usize, usize, u64, u64) = io.read4()?;

    let mut vm = Vec::with_capacity(n);
    let mut ps = Vec::with_capacity(n * 2 + 2);
    for _ in 0..n {
        let (l, r) = io.read2::<u64, u64>()?;
        vm.push((l, r));
        ps.push(l.min(x - m));
        ps.push(r.max(m) - m);
    }
    ps.push(0);
    ps.push(x - m);
    ps.sort();
    ps.dedup();
    vm.sort_by_key(|(l, r)| l + r);

    let mut ints = vec![vec![0; n]; ps.len()];
    for (&ps, i) in ps.iter().zip(ints.iter_mut()) {
        for (&(l, r), i) in vm.iter().zip(i.iter_mut()) {
            let l = l.max(ps);
            let r = r.min(ps + m);
            if r <= l {
                // *i = 0;
            } else {
                *i = r - l;
            }
        }
    }

    let mut bsg = vec![vec![0; n + 1]; n];
    for (i, iter) in ints.iter().enumerate() {
        for (j, &iter) in iter.iter().enumerate() {
            if iter > ints[bsg[j][j]][j] {
                bsg[j][j] = i;
            }
        }
    }

    let mut sints = vec![vec![0; n]; n];
    sints[0][0] = ints[bsg[0][0]][0];
    for i in 1..n {
        bsg[i][i] = bsg[i][i] / cmp::max / bsg[i - 1][i - 1];
        sints[i][i] = ints[bsg[i][i]][i];
    }

    let mut presum = vec![vec![0; n + 1]; ps.len()];
    for (presum, ints) in presum.iter_mut().zip(ints.iter()) {
        for i in 1..=n {
            presum[i] = presum[i - 1] + ints[i - 1];
        }
    }

    for l in (0..n).rev() {
        for r in l + 1..n {
            sints[l][r] = 0;
            for t in bsg[l][r - 1]..=bsg[l + 1][r] {
                let tmp = presum[t][r + 1] - presum[t][l];
                if tmp > sints[l][r] {
                    sints[l][r] = tmp;
                    bsg[l][r] = t;
                }
            }
        }
    }

    let mut dp = vec![vec![0; n]; n];
    let mut sp = bsg;
    for r in 0..n {
        dp[0][r] = sints[0][r];
        sp[0][r] = 0;
    }

    for i in 1..k {
        sp[i][n] = n - 1;
        for r in (0..n).rev() {
            dp[i][r] = 0;
            for l in sp[i - 1][r]..=sp[i][r + 1] {
                let mut ssints = 0;
                if l > 0 {
                    ssints += dp[i - 1][l - 1];
                }
                ssints += sints[l][r];
                if ssints > dp[i][r] {
                    // max
                    dp[i][r] = ssints;
                    sp[i][r] = l;
                }
            }
        }
    }

    let mut ans = (x - m) * n as u64;
    ans += dp[k - 1][n - 1];
    ans -= vm.into_iter().map(|(l, r)| r - l).sum::<u64>();

    io.print(ans)?;

    Ok(())
}

#[allow(dead_code)]
mod cmp {
    use std::ops::{Div, DivAssign};

    #[allow(non_camel_case_types)]
    pub struct max;
    #[allow(non_camel_case_types)]
    pub struct max_partial<T: Ord>(T);
    impl<T: Ord> Div<T> for max_partial<T> {
        type Output = T;
        fn div(self, rhs: T) -> Self::Output {
            self.0.max(rhs)
        }
    }
    impl<T: Ord> Div<T> for max {
        type Output = max_partial<T>;
        fn div(self, rhs: T) -> Self::Output {
            max_partial(rhs)
        }
    }

    #[allow(non_camel_case_types)]
    pub struct min;
    #[allow(non_camel_case_types)]
    pub struct min_partial<T: Ord>(T);
    impl<T: Ord> Div<T> for min_partial<T> {
        type Output = T;
        fn div(self, rhs: T) -> Self::Output {
            self.0.min(rhs)
        }
    }
    impl<T: Ord> Div<T> for min {
        type Output = min_partial<T>;
        fn div(self, rhs: T) -> Self::Output {
            min_partial(rhs)
        }
    }

    macro_rules! cmp_impl {
        ($t:ty) => {
            impl Div<max> for $t {
                type Output = max_partial<$t>;
                fn div(self, _: max) -> Self::Output {
                    max_partial(self)
                }
            }
            impl DivAssign<max_partial<$t>> for $t {
                fn div_assign(&mut self, rhs: max_partial<$t>) {
                    *self = (*self).max(rhs.0)
                }
            }
            impl Div<min> for $t {
                type Output = min_partial<$t>;
                fn div(self, _: min) -> Self::Output {
                    min_partial(self)
                }
            }
            impl DivAssign<min_partial<$t>> for $t {
                fn div_assign(&mut self, rhs: min_partial<$t>) {
                    *self = (*self).min(rhs.0)
                }
            }
        };
    }

    cmp_impl!(usize);
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

        pub fn read<T>(&mut self) -> Result<T, Box<dyn std::error::Error>>
        where
            T: FromStr,
            <T>::Err: Error + 'static,
        {
            self.scan.read_line(&mut self.buf)?;
            Ok(std::mem::take(&mut self.buf).trim().parse::<T>()?)
        }

        read_impl!(read2, T1, T2);
        read_impl!(read4, T1, T2, T3, T4);

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
    }

    impl<'a> Default for IO<'a> {
        fn default() -> Self {
            IO::new()
        }
    }
}
