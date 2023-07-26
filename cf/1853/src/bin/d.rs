fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let v: Vec<usize> = io.get_vec(n)?;

		io.put(v[0])?;
	}

	Ok(())
}
