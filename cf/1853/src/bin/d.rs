fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::io::default();
	let t: usize = io.read()?;
	for _ in 0..t {
		let n: usize = io.read()?;
		let v: Vec<usize> = io.read_vec(n)?;

		io.print(v[0])?;
	}

	Ok(())
}
