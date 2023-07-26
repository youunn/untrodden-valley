fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n = io.get::<usize>()?;
		let k = io.get::<usize>()?;
		let v = io.get_vec::<usize, Vec<_>>(n)?;

		let mut ans = 1;
		if v[0] > 1 {
			io.put(ans)?;
			continue;
		}
		
		let mut i = 0;
		for _ in 0..k {
			ans += i as u64;
			while i < n && v[i] as u64 <= ans {
				i += 1;
				ans += 1;
			}
		}

		io.put(ans)?;
	}

	Ok(())
}
