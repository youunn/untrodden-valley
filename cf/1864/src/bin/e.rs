type N = cp::num::Number<998_244_353>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	const P: usize = 30;
	for _ in 0..t {
		let n: usize = io.get()?;
		let mut v: Vec<u64> = io.get_vec(n)?;
		v.sort();
		let total = N::new(n as u64 * n as u64);
		let mut ans = total;

		for _ in 0..P {
			let mut l = 0;
			let mut r = 0;
			while l < n {
				let mut c1 = 0u64;
				let mut c2 = 0u64;
				while r < n && v[l] >> 1 == v[r] >> 1 {
					if v[r] % 2 == 1 {
						c1 += 1;
					} else {
						c2 += 1;
					}
					r += 1;
				}
				ans += c1 * (c1 + c2);
				l = r;
			}
			for x in v.iter_mut() {
				*x >>= 1;
			}
		}

		ans /= total;
		io.put(ans.0)?;
	}

	Ok(())
}
