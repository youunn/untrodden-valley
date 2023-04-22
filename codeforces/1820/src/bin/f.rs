use std::{collections::BTreeSet, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let t: u32 = scan.next()?;
    for _ in 0..t {
        let n: usize = scan.next()?;
        let m: usize = scan.next()?;

        let mut stores = Vec::with_capacity(n + 1);
        stores.push(BTreeSet::new());
        for _ in 0..n {
            // let k: usize = scan.next()?;
            let v: BTreeSet<usize> = scan.next_vec()?;
            stores.push(v);
        }

        // 恶行逆施，速度E
        let mut d4c = Vec::with_capacity(n + 1);
        d4c.push(0);
        for i in 1..=n {
            if stores[i].is_empty() {
                d4c.push(0);
                continue;
            }
            let mut l = 0;
            for j in (1..i).rev() {
                if stores[j].len() + stores[i].len() > m || !stores[j].is_disjoint(&stores[i]) {
                    l = j;
                    break;
                }
            }
            d4c.push(l);
        }

        let mut can_zero = BTreeSet::new();
        can_zero.insert(0);
        'a: for i in 1..=n {
            if stores[i].is_empty() {
                can_zero.insert(i);
                continue;
            }

            let mut l = None;
            for j in (d4c[i] + 1..i).rev() {
                if stores[j].is_empty() {
                    l = Some(j);
                    break;
                }
            }

            let l = if let Some(l) = l {
                l
            } else if d4c[i] > 0 {
                d4c[i]
            } else {
                // can_zero.push(false);
                continue;
            };

            let mut s = 0;
            for j in (0..l).rev() {
                if can_zero.contains(&j) {
                    s = j;
                    break;
                }
            }

            for i in (s + 1..i).rev() {
                if d4c[i] > s {
                    // can_zero.push(false);
                    continue 'a;
                }
            }

            can_zero.insert(i);
        }

        let mut max_remain = BTreeSet::new();
        'a: for &i in &can_zero {
            let mut has_zero = false;
            let mut sum = 0;
            for j in (i + 1..n + 1).rev() {
                if !has_zero && stores[j].is_empty() {
                    has_zero = true;
                } else {
                    if d4c[j] > i {
                        // can_zero.push(false);
                        continue 'a;
                    }
                    sum += stores[j].len();
                }
            }
            max_remain.insert(if has_zero { m } else { sum });
        }
        max_remain.insert(0);

        let ans = max_remain.iter().max().ok_or("")?;
        writeln!(out, "{}", ans)?;
    }

    Ok(())
}

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R> Scanner<R>
where
    R: std::io::BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    pub fn next<T>(&mut self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: std::str::FromStr,
        <T>::Err: std::error::Error + 'static,
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

    pub fn next_vec<V, T>(&mut self) -> Result<V, Box<dyn std::error::Error>>
    where
        V: std::iter::FromIterator<T>,
        T: std::str::FromStr,
        <T>::Err: std::error::Error + 'static,
    {
        let mut s = String::new();
        self.reader.read_line(&mut s)?;
        Ok(s.split_whitespace()
            .skip(1)
            .map(|s| s.parse())
            .collect::<Result<V, _>>()?)
    }
}
