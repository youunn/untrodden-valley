use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write};
use std::ops::Add;
use std::str::FromStr;

macro_rules! get_or_init {
    (|| -> $t:ty $init:block) => {{
        static mut VALUE: std::mem::MaybeUninit<$t> = std::mem::MaybeUninit::uninit();
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let v = $init;
            unsafe {
                VALUE.write(v);
            }
        });
        unsafe { VALUE.assume_init_ref() }
    }};
}

fn phi(i: usize) -> usize {
    let phi = get_or_init!(|| -> Vec<usize> {
        let n = 5000000;
        let mut v: Vec<_> = (0..=n).collect();
        for i in 2..=n {
            if v[i] != i {
                continue;
            }
            for j in (i..=n).step_by(i) {
                v[j] -= v[j] / i;
            }
        }
        v
    });
    phi[i]
}

#[derive(Clone, Copy)]
struct Node {
    v: usize,
    len: usize,
    ops: usize,
    mio: usize, // max in offspring
}

impl Add for Node {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut res = Self {
            v: 0,
            len: self.len + rhs.len,
            ops: self.ops + rhs.ops,
            mio: self.mio.max(rhs.mio),
        };
        loop {
            match self.v.cmp(&rhs.v) {
                Equal => break,
                Less => {
                    rhs.v = phi(rhs.v);
                    res.ops += rhs.len;
                }
                Greater => {
                    self.v = phi(self.v);
                    res.ops += self.len;
                }
            }
        }
        res.v = self.v;
        res
    }
}

struct SegTree {
    values: Vec<usize>,
    tree: Vec<Node>,
}

impl SegTree {
    fn new(values: Vec<usize>, n: usize) -> Self {
        let tree = vec![
            Node {
                v: 0,
                len: 1,
                ops: 0,
                mio: 0,
            };
            4 * n
        ];
        let mut res = Self { values, tree };
        res.build(1, 1, n);
        res
    }

    fn build(&mut self, root: usize, l: usize, r: usize) {
        if l == r {
            let v = self.values[l - 1];
            self.tree[root].v = v;
            self.tree[root].mio = v;
        } else {
            let m = (l + r) / 2;
            let lc = root * 2;
            let rc = lc + 1;
            self.build(lc, l, m);
            self.build(rc, m + 1, r);
            self.tree[root] = self.tree[lc] + self.tree[rc];
        }
    }

    pub fn 完全净化(&mut self, root: usize, lend: usize, rend: usize, from: usize, to: usize) {
        if self.tree[root].mio == 1 {
            return;
        }
        if lend == rend {
            let v = self.tree[root].v;
            self.tree[root].v = phi(v);
            self.tree[root].mio = v;
            return;
        }
        let mid = (lend + rend) / 2;
        let lc = root * 2;
        let rc = lc + 1;
        if to <= mid {
            self.完全净化(lc, lend, mid, from, to);
        } else if from > mid {
            self.完全净化(rc, mid + 1, rend, from, to);
        } else {
            self.完全净化(lc, lend, mid, from, mid);
            self.完全净化(rc, mid + 1, rend, mid + 1, to);
        }
        self.tree[root] = self.tree[lc] + self.tree[rc];
    }

    pub fn 正义威光(
        &self,
        root: usize,
        lend: usize,
        rend: usize,
        from: usize,
        to: usize,
    ) -> Node {
        if lend == from && rend == to {
            return self.tree[root];
        }
        let mid = (lend + rend) / 2;
        let lc = root * 2;
        let rc = lc + 1;
        if to <= mid {
            self.正义威光(lc, lend, mid, from, to)
        } else if from > mid {
            self.正义威光(rc, mid + 1, rend, from, to)
        } else {
            let lsum = self.正义威光(lc, lend, mid, from, mid);
            let rsum = self.正义威光(rc, mid + 1, rend, mid + 1, to);
            lsum + rsum
        }
    }
}

fn 和解(
    mut scan: Scanner<StdinLock>,
    mut out: BufWriter<StdoutLock>,
) -> Result<(), Box<dyn Error>> {
    let n: usize = scan.next()?;
    let m: u32 = scan.next()?;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let a: usize = scan.next()?;
        v.push(a);
    }
    let mut 星 = SegTree::new(v, n);
    for _ in 0..m {
        let op: u8 = scan.next()?;
        let l: usize = scan.next()?;
        let r: usize = scan.next()?;
        if op == 1 {
            星.完全净化(1, 1, n, l, r);
        } else {
            writeln!(out, "{}", 星.正义威光(1, 1, n, l, r).ops)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let scan = Scanner::new(stdin().lock());
    let out = BufWriter::new(stdout().lock());

    和解(scan, out)?;

    Ok(())
}

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R> Scanner<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    pub fn next<T>(&mut self) -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        <T>::Err: Error + 'static,
    {
        loop {
            if let Some(x) = self.buffer.pop() {
                let result = x.parse()?;
                return Ok(result);
            }
            let mut s = String::new();
            self.reader.read_line(&mut s)?;
            self.buffer = s.split_whitespace().rev().map(String::from).collect();
        }
    }
}
