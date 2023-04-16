use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let t: u32 = scan.next()?;
    for _ in 0..t {
        let n: usize = scan.next()?;
        let mut parts_sorted_by_height = Vec::with_capacity(n);
        let mut parts_sorted_by_width = Vec::with_capacity(n);
        let mut total_blocks = 0;
        for i in 0..n {
            let h = scan.next::<u32>()?;
            let w = scan.next::<u32>()?;
            // 0 for height, 1 for width
            parts_sorted_by_height.push((h, w, i));
            parts_sorted_by_width.push((h, w, i));
            total_blocks += h * w;
        }
        parts_sorted_by_height.sort_by(|a, b| a.0.cmp(&b.0));
        parts_sorted_by_width.sort_by(|a, b| a.1.cmp(&b.1));
        let mut used = vec![false; n];

        // two results at most
        let part_with_max_height = *parts_sorted_by_height.last().ok_or("")?;
        let part_with_max_width = *parts_sorted_by_width.last().ok_or("")?;
        if total_blocks % part_with_max_height.0 != 0 {
            writeln!(out, "1")?;
            writeln!(
                out,
                "{} {}",
                total_blocks / part_with_max_width.1,
                part_with_max_width.1
            )?;
            continue;
        }
        if total_blocks % part_with_max_width.1 != 0 {
            writeln!(out, "1")?;
            writeln!(
                out,
                "{} {}",
                part_with_max_height.0,
                total_blocks / part_with_max_height.0
            )?;
            continue;
        }

        parts_sorted_by_height.pop();
        parts_sorted_by_width.pop();
        if !divide_and_conquer(
            part_with_max_height.0,
            total_blocks / part_with_max_height.0 - part_with_max_height.1,
            &mut parts_sorted_by_height,
            &mut parts_sorted_by_width,
            &mut used,
        ) {
            writeln!(out, "1")?;
            writeln!(
                out,
                "{} {}",
                total_blocks / part_with_max_width.1,
                part_with_max_width.1,
            )?;
        } else if !divide_and_conquer(
            total_blocks / part_with_max_width.1 - part_with_max_width.1,
            part_with_max_width.1,
            &mut parts_sorted_by_height,
            &mut parts_sorted_by_width,
            &mut used,
        ) {
            writeln!(out, "1")?;
            writeln!(
                out,
                "{} {}",
                part_with_max_height.0,
                total_blocks / part_with_max_height.0,
            )?;
        } else if part_with_max_width.0 == part_with_max_height.0 {
            writeln!(out, "1")?;
            writeln!(
                out,
                "{} {}",
                part_with_max_height.0,
                part_with_max_width.0,
            )?;
        }else {
            writeln!(out, "2")?;
            writeln!(
                out,
                "{} {}",
                part_with_max_height.0,
                total_blocks / part_with_max_height.0,
            )?;
            writeln!(
                out,
                "{} {}",
                total_blocks / part_with_max_width.1,
                part_with_max_width.1
            )?;
        }
    }
    Ok(())
}

use std::cmp::Ordering::{Equal, Less};

fn divide_and_conquer(
    mut total_height: u32,
    mut total_width: u32,
    parts_sorted_by_height: &mut Vec<(u32, u32, usize)>,
    parts_sorted_by_width: &mut Vec<(u32, u32, usize)>,
    used: &mut [bool],
) -> bool {
    let mut rollback = Vec::new();
    let part = loop {
        if let Some(part) = parts_sorted_by_height.pop() {
            rollback.push(part);
            if !used[part.2] {
                break part;
            }
        } else {
            parts_sorted_by_height.extend(rollback);
            return true;
        }
    };
    if match total_height.cmp(&part.0) {
        Less => {
            parts_sorted_by_height.extend(rollback);
            return false;
        }
        Equal => {
            total_height -= part.0;
            used[part.2] = true;
            let result = divide_and_conquer(
                total_height,
                total_width,
                parts_sorted_by_height,
                parts_sorted_by_width,
                used,
            );
            parts_sorted_by_height.extend(rollback);
            result
        }
        _ => false,
    } {
        return true;
    }

    let mut rollback = Vec::new();
    let part = loop {
        if let Some(&part) = parts_sorted_by_width.last() {
            rollback.push(part);
            if !used[part.2] {
                break part;
            }
        } else {
            parts_sorted_by_width.extend(rollback);
            return true;
        }
    };
    match total_width.cmp(&part.0) {
        Less => {
            parts_sorted_by_width.extend(rollback);
            false
        }
        Equal => {
            total_width -= part.1;
            used[part.2] = true;
            let result = divide_and_conquer(
                total_height,
                total_width,
                parts_sorted_by_height,
                parts_sorted_by_width,
                used,
            );
            parts_sorted_by_width.extend(rollback);
            result
        }
        _ => false,
    }
}

use std::io::BufRead;
use std::str::FromStr;

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
