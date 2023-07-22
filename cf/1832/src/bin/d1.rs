use std::{
    error::Error,
    io::{stdin, stdout, BufRead, BufWriter, Write},
};

fn solve(n: usize, q: usize, mut va: Vec<i64>, vk: Vec<usize>) -> Vec<i64> {
    va.sort();

    let mut min = Vec::with_capacity(n);
    min.push(va[0]);
    for (i, &a) in va.iter().enumerate().skip(1) {
        min.push(min[i - 1].min(a - i as i64));
    }
    let sum = va.iter().sum::<i64>() - (n * (n - 1) / 2) as i64;

    let mut ans = Vec::with_capacity(q);

    for k in vk {
        if k < n {
            let m = min[k - 1] + k as i64;
            ans.push(m.min(va[k]));
        } else if k % 2 == n % 2 {
            // (1 -2) (3 -4) ... then
            // (k - (n - 1) -> a_(n-1)) ... (k - 1 -> a_1) (k -> a_0)
            let m = min[n - 1] + k as i64;
            let ops = ((k - n) / 2) as i64;
            let reserve = sum + (n * k) as i64 - m * n as i64;
            ans.push(if ops > reserve {
                // ceil
                m - (ops - reserve + n as i64 - 1) / n as i64
            } else {
                m
            });
        } else if n == 1 {
            ans.push(va[0] - k as i64 / 2)
        } else {
            // (1 -2) (3 -4) ... then
            // (do nothing other to a_(n-1) except -1) then
            // (k - (n - 2) -> a_(n-2)) ... (k - 1 -> a_1) (k -> a_0)
            let m = min[n - 2] + k as i64;
            let m = m.min(va[n - 1]);
            let ops = ((k - (n - 1)) / 2) as i64;
            let reserve = sum + (n - 1) as i64 + ((n - 1) * k) as i64 - m * n as i64;
            ans.push(if ops > reserve {
                m - (ops - reserve + n as i64 - 1) / n as i64
            } else {
                m
            });
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = stdin().lock();
    let mut out = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    let buf = &mut buf;

    let (n, q): (usize, usize) = {
        scan.read_line(buf)?;
        let buf = std::mem::take(buf);
        let mut s = buf.split_whitespace();
        (s.next().ok_or("")?.parse()?, s.next().ok_or("")?.parse()?)
    };

    let va: Vec<i64> = {
        scan.read_line(buf)?;
        let buf = std::mem::take(buf);
        buf.split_whitespace()
            .take(n)
            .map(&str::parse)
            .collect::<Result<_, _>>()?
    };

    let vk: Vec<usize> = {
        scan.read_line(buf)?;
        let buf = std::mem::take(buf);
        buf.split_whitespace()
            .take(q)
            .map(&str::parse)
            .collect::<Result<_, _>>()?
    };

    let mut ans = solve(n, q, va, vk).into_iter();
    write!(out, "{}", ans.next().ok_or("")?)?;
    while let Some(next) = ans.next() {
        write!(out, " {}", next)?;
    }
    writeln!(out)?;

    Ok(())
}
