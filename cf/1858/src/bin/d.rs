fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let k: usize = io.get()?;
		let s: String = io.get()?;
		let s = s.as_bytes();

		let mut mp0 = vec![vec![0; n + 1]; n + 1];
		let mut ms0 = vec![vec![0; n + 1]; n + 1];

		#[allow(clippy::needless_range_loop)]
		for l in 0..n {
			let mut k = 0;
			for r in l + 1..=n {
				if s[r - 1] != b'0' {
					k += 1;
				}
				mp0[r][k] /= cp::cmp::max / (r - l);
				ms0[l][k] /= cp::cmp::max / (r - l);
			}
		}

		for r in 0..=n {
			for k in 0..=n {
				if r > 0 {
					let last = mp0[r - 1][k];
					mp0[r][k] /= cp::cmp::max / last;
				}
				if k > 0 {
					let last = mp0[r][k - 1];
					mp0[r][k] /= cp::cmp::max / last;
				}
			}
		}

		for l in (0..=n).rev() {
			for k in 0..=n {
				if l < n {
					let last = ms0[l + 1][k];
					ms0[l][k] /= cp::cmp::max / last;
				}
				if k > 0 {
					let last = ms0[l][k - 1];
					ms0[l][k] /= cp::cmp::max / last;
				}
			}
		}

		let mut mlen0 = vec![None; n + 1];
		for l in 0..n {
			let mut c = 0;
			for r in l..=n {
				if r > l && s[r - 1] == b'0' {
					c += 1;
				}
				if c > k {
					break;
				}
				let last = mp0[l][k - c].max(ms0[r][k - c]);
				mlen0[r - l] /= cp::cmp::max / Some(last);
			}
		}

		// score = a * l_0 + l_1
		let mut ans = vec![None; n];
		#[allow(clippy::needless_range_loop)]
		for len1 in 0..=n {
			for a in 1..=n {
				if let Some(len0) = mlen0[len1] {
					ans[a - 1] /= cp::cmp::max / Some(len1 + a * len0);
				}
			}
		}

		io.put_vec(ans.into_iter().map(|x| x.unwrap_or_default()))?;
	}

	Ok(())
}
