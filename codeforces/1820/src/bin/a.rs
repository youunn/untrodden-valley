use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let m: u8 = scan.next()?;

    for _ in 0..m {
        let s = scan.next::<String>()?;
        if s == "^" {
            writeln!(out, "{}", 1)?;
            continue;
        }
        let s = s.as_bytes();
        let mut ans = 0;
        // waiting for eyes
        let mut flag = true;
        for i in s {
            if flag {
                if i == &b'_' {
                    ans += 1;
                } else {
                    flag = false;
                }
            } else if i == &b'_' {
                flag = true;
            }
        }
        if flag {
            ans += 1;
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
