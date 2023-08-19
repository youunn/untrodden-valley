fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let s: String = io.get()?;
	let s = s.as_bytes();
	let n = s.len();
	let b0 = s[0];

	// 00 01 10 11

	let c = s.iter().filter(|&c| *c == b0).count();
	// c11 + (c10 + c01) / 2
	let m = (n - 1) * c / 2;
	let mut dp = vec![vec![None; m + 1]; c + 1];
	dp[0][0] = Some(0usize);

	for (i, &b) in s.iter().enumerate() {
		let p = if b == b0 { 1 } else { 0 };
		for j in (0..c).rev() {
			for k in 0..=m - i {
				if let Some(last) = dp[j][k] {
					dp[j + 1][k + i] /= cp::cmp::max / Some(last + p);
				}
			}
		}
	}

	io.put(c - dp[c][m].ok_or("")?)?;

	Ok(())
}
