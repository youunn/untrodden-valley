fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let v: Vec<usize> = io.get_vec(n)?;
		let mut ans = 0usize;
		let mut cm = usize::MAX;
		let mut wm = usize::MAX;
		for p in v {
			if p < cm {
				// lose at minimum
				cm = p;
			} else if p < wm {
				// win at the next minimum
				// greater than minimum
				// after minimum
				wm = p;
				ans += 1;
			} else {
				// lose
			}
		}

		io.put(ans)?;
	}

	Ok(())
}
