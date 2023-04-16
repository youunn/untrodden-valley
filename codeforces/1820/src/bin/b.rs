use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let m: u32 = scan.next()?;

    for _ in 0..m {
        let s = scan.next::<String>()?;
        let s = s.as_bytes();
        // has nothing with order
        // so just find the max length of consisting '1's
        let mut max = 0;
        let mut cur = 0;
        let mut first = None; // first 0
        for (i, c) in s.iter().enumerate() {
            if c == &b'1' {
                cur += 1;
            } else {
                if cur > max {
                    max = cur;
                }
                cur = 0;
                if first.is_none() {
                    first = Some(i);
                }
            }
        }

        if let Some(first) = first {
            cur += first;
            if cur > max {
                max = cur;
            }

            let x = max / 2 + 1;
            let ans = if max % 2 == 0 { x * (x - 1) } else { x * x };
            writeln!(out, "{}", ans)?;
        } else {
            let x = cur;
            let ans = x * x;
            writeln!(out, "{}", ans)?;
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
}
