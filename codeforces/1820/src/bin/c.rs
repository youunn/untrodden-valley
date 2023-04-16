use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let t: u16 = scan.next()?;
    for _ in 0..t {
        let n: u32 = scan.next()?;
        let mut v: Vec<u64> = scan.next_vec(n as usize)?;
        // the operation must update all m + 1 to m
        let mut sv = v.to_vec();
        sv.sort();
        let (m, flag) = {
            let mut m = None;
            let mut past_plus_1 = 0;
            // find duplicate
            let mut flag = false;
            for a in &sv {
                let v = a + 1 - past_plus_1;
                if !flag {
                    if v == 0 {
                        flag = true;
                    } else if v > 1 {
                        m = Some(past_plus_1);
                        break;
                    }
                } else if v > 1 {
                    m = Some(past_plus_1);
                    break;
                }
                past_plus_1 = a + 1;
            }
            let m = m.unwrap_or(past_plus_1);
            if &m < sv.last().ok_or("")? {
                flag = true;
            }
            (m, flag)
        };
        // find first m + 1
        let mut first = None;
        for (i, a) in v.iter().enumerate() {
            if *a == m + 1 && first.is_none() {
                first = Some(i);
                break;
            }
        }
        if let Some(first) = first {
            let mut last = first;
            for (i, a) in v.iter().rev().enumerate() {
                if *a == m + 1 {
                    last = n as usize - 1 - i;
                    break;
                }
            }
            for a in v.iter_mut().take(last + 1).skip(first) {
                *a = m;
            }
            let mut sv = v.to_vec();
            sv.sort();
            let oldm = m;
            let m = {
                let mut m = None;
                let mut past_plus_1 = 0;
                for a in sv {
                    let v = a + 1 - past_plus_1;
                    if v > 1 {
                        m = Some(past_plus_1);
                        break;
                    }
                    past_plus_1 = a + 1;
                }
                m.unwrap_or(past_plus_1)
            };
            if m > oldm && m - oldm == 1 {
                writeln!(out, "Yes")?;
            } else {
                writeln!(out, "No")?;
            }
        } else {
            if flag {
                writeln!(out, "Yes")?;
            } else {
                writeln!(out, "No")?;
            }
            continue;
        }
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

    pub fn next_vec<V, T>(&mut self, n: usize) -> Result<V, Box<dyn std::error::Error>>
    where
        V: std::iter::FromIterator<T>,
        T: std::str::FromStr,
        <T>::Err: std::error::Error + 'static,
    {
        let mut s = String::new();
        self.reader.read_line(&mut s)?;
        Ok(s.split_whitespace()
            .take(n)
            .map(|s| s.parse())
            .collect::<Result<V, _>>()?)
    }
}
