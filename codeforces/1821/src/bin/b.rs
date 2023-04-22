use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let t: usize = scan.next()?;

    for _ in 0..t {
        let n: usize = scan.next()?;
        let a = scan.next_vec::<u32>()?;
        let b = scan.next_vec::<u32>()?;

        let mut from = 0;
        let mut to = n - 1;

        for i in 0..n {
            if a[i] != b[i] {
                from = i;
                break;
            }
        }

        for i in (0..n).rev() {
            if a[i] != b[i] {
                to = i;
                break;
            }
        }

        let mut v = b[from];
        let mut left = 0;
        for i in (0..from).rev() {
            if a[i] <= v {
                v = a[i];
            } else {
                left = i + 1;
                break;
            }
        }

        let mut v = b[to];
        let mut right = n - 1;
        for i in to + 1..n {
            if a[i] >= v {
                v = a[i];
            } else {
                right = i - 1;
                break;
            }
        }

        writeln!(out, "{} {}", left + 1, right + 1)?;
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

    pub fn next_vec<T>(&mut self) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: std::str::FromStr,
        <T>::Err: std::error::Error + 'static,
    {
        let mut s = String::new();
        self.reader.read_line(&mut s)?;
        let v = s
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<T>, _>>()?;
        Ok(v)
    }
}
