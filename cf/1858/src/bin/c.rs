fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let mut v = Vec::with_capacity(n);

		for i in (1..=n).step_by(2) {
			let mut j = i;
			while j <= n {
				v.push(j);
				j *= 2;
			}
		}

		io.put_vec(v.into_iter())?;
	}

	Ok(())
}
