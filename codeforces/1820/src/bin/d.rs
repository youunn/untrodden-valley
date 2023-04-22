use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let t: u32 = scan.next()?;
    let n = 200000;
    let mut used = vec![false; n];
    for _ in 0..t {
        let n: usize = scan.next()?;
        let mut parts = Vec::with_capacity(n);
        let mut total_blocks: u64 = 0;
        for i in 0..n {
            let h = scan.next::<u32>()?;
            let w = scan.next::<u32>()?;
            parts.push((h, w, i));
            total_blocks += h as u64 * w as u64;
        }
        parts.sort_by(|a, b| a.0.cmp(&b.0));
        let mut parts_sorted_by_height: Vec<_> = parts.iter().map(|x| x.2).collect();
        parts.sort_by(|a, b| a.1.cmp(&b.1));
        let mut parts_sorted_by_width: Vec<_> = parts.iter().map(|x| x.2).collect();
        parts.sort_by(|a, b| a.2.cmp(&b.2));

        let part_with_max_height = *parts_sorted_by_height.last().ok_or("")?;
        let part_with_max_width = *parts_sorted_by_width.last().ok_or("")?;

        let pair_by_height = (
            parts[part_with_max_height].0 as u64,
            total_blocks / parts[part_with_max_height].0 as u64,
        );

        let pair_by_width = (
            total_blocks / parts[part_with_max_width].1 as u64,
            parts[part_with_max_width].1 as u64,
        );

        if total_blocks % parts[part_with_max_height].0 as u64 != 0 {
            writeln!(out, "1")?;
            let result = &pair_by_width;
            writeln!(out, "{} {}", result.0, result.1)?;
            continue;
        }
        if total_blocks % parts[part_with_max_width].1 as u64 != 0 {
            writeln!(out, "1")?;
            let result = &pair_by_height;
            writeln!(out, "{} {}", result.0, result.1)?;
            continue;
        }

        let part = parts_sorted_by_height.pop().ok_or("")?;
        used[part] = true;
        if !rec(
            parts[part].0 as u64,
            total_blocks / parts[part].0 as u64 - parts[part].1 as u64,
            &parts,
            &mut parts_sorted_by_height,
            &mut parts_sorted_by_width,
            &mut used,
        ) {
            writeln!(out, "1")?;
            let result = &pair_by_width;
            writeln!(out, "{} {}", result.0, result.1)?;
            parts_sorted_by_height.push(part);
            used[part] = false;
            continue;
        }
        parts_sorted_by_height.push(part);
        used[part] = false;

        let part = parts_sorted_by_width.pop().ok_or("")?;
        used[part] = true;

        if !rec(
            total_blocks / parts[part].1 as u64 - parts[part].0 as u64,
            parts[part].1 as u64,
            &parts,
            &mut parts_sorted_by_height,
            &mut parts_sorted_by_width,
            &mut used,
        ) {
            writeln!(out, "1")?;
            let result = &pair_by_height;
            writeln!(out, "{} {}", result.0, result.1)?;
            parts_sorted_by_width.push(part);
            used[part] = false;
            continue;
        }
        parts_sorted_by_width.push(part);
        used[part] = false;

        if pair_by_height.0 == pair_by_width.0 && pair_by_height.1 == pair_by_width.1 {
            writeln!(out, "1")?;
            let result = &pair_by_height;
            writeln!(out, "{} {}", result.0, result.1)?;
            continue;
        }

        writeln!(out, "2")?;
        let result = &pair_by_height;
        writeln!(out, "{} {}", result.0, result.1)?;
        let result = &pair_by_width;
        writeln!(out, "{} {}", result.0, result.1)?;
    }
    Ok(())
}

use std::cmp::Ordering::{Equal, Greater, Less};

fn rec(
    mut total_height: u64,
    mut total_width: u64,
    parts: &[(u32, u32, usize)],
    parts_sorted_by_height: &mut Vec<usize>,
    parts_sorted_by_width: &mut Vec<usize>,
    used: &mut [bool],
) -> bool {
    let mut rollback_by_height = Vec::new();
    let mut rollback_by_width = Vec::new();
    let mut used_indices = Vec::new();
    let result = 'a: loop {
        let part = loop {
            if let Some(&part) = parts_sorted_by_height.last() {
                if used[part] {
                    parts_sorted_by_height.pop();
                    rollback_by_height.push(part);
                    continue;
                }
                match total_height.cmp(&(parts[part].0 as u64)) {
                    Less => {
                        break 'a false;
                    }
                    Equal => {
                        break Some(part);
                    }
                    Greater => {
                        break None;
                    }
                }
            } else {
                break 'a true;
            }
        };

        if let Some(part) = part {
            parts_sorted_by_height.pop();
            rollback_by_height.push(part);
            used[part] = true;
            used_indices.push(part);
            total_width -= parts[part].1 as u64;
            continue;
        }

        let part = loop {
            if let Some(&part) = parts_sorted_by_width.last() {
                if used[part] {
                    parts_sorted_by_width.pop();
                    rollback_by_width.push(part);
                    continue;
                }
                match total_width.cmp(&(parts[part].1 as u64)) {
                    Less => {
                        break 'a false;
                    }
                    Equal => {
                        break Some(part);
                    }
                    Greater => {
                        break None;
                    }
                }
            } else {
                break 'a true;
            }
        };

        if let Some(part) = part {
            parts_sorted_by_width.pop();
            rollback_by_width.push(part);
            used[part] = true;
            used_indices.push(part);
            total_height -= parts[part].0 as u64;
            continue;
        }

        break 'a false;
    };

    parts_sorted_by_height.extend(rollback_by_height.iter().rev());
    parts_sorted_by_width.extend(rollback_by_width.iter().rev());
    for i in used_indices {
        used[i] = false;
    }

    result
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
