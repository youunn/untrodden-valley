fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let k: usize = io.get()?;
		let s: String = io.get()?;
		let mut s = s.into_bytes();

		if k % 2 == 0 {
			s.sort();
			io.put(unsafe { String::from_utf8_unchecked(s) })?;
			continue;
		}

		let mut odd: Vec<_> =
			s.iter().enumerate()
				.filter(|s| s.0 % 2 == 0)
				.map(|s| s.1)
				.collect();
		odd.sort();
		let mut even: Vec<_> =
			s.iter().enumerate()
				.filter(|s| s.0 % 2 == 1)
				.map(|s| s.1)
				.collect();
		even.sort();

		let mut s = Vec::with_capacity(n);
		let last = *odd[odd.len() - 1];
		for (o, e) in odd.into_iter().zip(even) {
			s.push(*o);
			s.push(*e);
		}
		if n % 2 == 1 {
			s.push(last);
		}

		io.put(unsafe { String::from_utf8_unchecked(s) })?;
	}

	Ok(())
}
