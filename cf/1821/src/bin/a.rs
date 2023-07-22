use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let t: u32 = scan.next()?;

    for _ in 0..t {
        let s = scan.next_line()?;
        let s = s.as_bytes();
        if s[0] == b'0' {
            writeln!(out, "0")?;
            continue;
        }
        let mut ans = if s[0] == b'?' { 9 } else { 1 };

        for &c in &s[1..] {
            if c == b'?' {
                ans *= 10;
            }
        }
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

    pub fn next_line(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = String::new();
        self.reader.read_line(&mut s)?;
        Ok(s)
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
