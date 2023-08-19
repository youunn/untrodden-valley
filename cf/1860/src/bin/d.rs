use std::io::{prelude::*, stdin, stdout, BufRead, BufWriter};

trait OrdAssign<T> {
	fn max_assign(&mut self, rhs: T);
	fn min_assign(&mut self, rhs: T);
}

impl OrdAssign<usize> for usize {
	fn max_assign(&mut self, rhs: usize) {
		*self = std::cmp::max(*self, rhs);
	}
	fn min_assign(&mut self, rhs: usize) {
		*self = std::cmp::min(*self, rhs);
	}
}

fn main() -> std::io::Result<()> {
	let mut scan = stdin().lock();
	let mut out = BufWriter::new(stdout().lock());

	let mut s = String::new();
	scan.read_line(&mut s)?;
	let s = s.trim().as_bytes();
	let n = s.len();
	let b0 = s[0];

	let m = vec![vec![usize::MAX; (n + 1) * (n + 1)]; n + 1];
	// s -> [c0][c01]
	let mut dpl = vec![vec![0]];
	let mut dpc: Vec<_>;

	for (i, &b) in s.iter().enumerate() {
		dpc = m.clone();
		let p0 = if b == b0 { 1 } else { 0 };

		for c0 in 0..=i {
			for c01 in 0..=c0 * (i - c0) {
				let trans = dpl[c0][c01] + 1 - p0;
				dpc[c0 + 1][c01].min_assign(trans);
				let trans = dpl[c0][c01] + p0;
				dpc[c0][c01 + c0].min_assign(trans);
			}
		}

		dpl = std::mem::take(&mut dpc);
	}

	let c0 = s.iter().filter(|&b| *b == b0).count();
	let c01 = c0 * (n - c0) / 2;
	writeln!(out, "{}", dpl[c0][c01] / 2)?;

	Ok(())
}
