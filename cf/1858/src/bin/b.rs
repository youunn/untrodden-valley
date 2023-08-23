use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let m: usize = io.get()?;
		let d: usize = io.get()?;
		let mut v: Vec<usize> = io.get_vec(m)?;
		v.push(n + 1);

		let mut sum = 0;
		if v[0] != 1 {
			sum += (v[0] - 1 + d - 1) / d;
		}
		for i in 1..=m {
			sum += (v[i] - v[i - 1] - 1) / d
		}

		let mut ans = usize::MAX;
		let mut cnt = 0;

		{
			#[allow(clippy::redundant_locals)]
			let mut sum = sum;
			sum -= (v[1] - v[0] - 1) / d;
			if v[0] != 1 {
				sum -= (v[0] - 1 + d - 1) / d;
			}
			sum += (v[1] - 1 + d - 1) / d;
			sum += m - 1;

			#[allow(clippy::comparison_chain)]
			if sum < ans {
				ans = sum;
				cnt = 1;
			} else if sum == ans {
				cnt += 1;
			}
		}

		for i in 1..m {
			#[allow(clippy::redundant_locals)]
			let mut sum = sum;
			sum -= (v[i] - v[i - 1] - 1) / d;
			sum -= (v[i + 1] - v[i] - 1) / d;
			sum += (v[i + 1] - v[i - 1] - 1) / d;
			sum += m - 1;

			#[allow(clippy::comparison_chain)]
			if sum < ans {
				ans = sum;
				cnt = 1;
			} else if sum == ans {
				cnt += 1;
			}
		}

		writeln!(io, "{} {}", ans, cnt)?;
	}

	Ok(())
}
