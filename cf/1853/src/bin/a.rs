fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::io::default();
	let t: usize = io.read()?;
	't: for _ in 0..t {
		let n: usize = io.read()?;
		let v: Vec<usize> = io.read_vec(n)?;
		let mut ans = usize::MAX;
		for x in v.windows(2) {
			if x[0] > x[1] {
				io.print(0)?;
				continue 't;
			}
			ans = std::cmp::min(ans, (x[1] - x[0]) / 2);
		}
		io.print(ans + 1)?;
	}

	Ok(())
}
