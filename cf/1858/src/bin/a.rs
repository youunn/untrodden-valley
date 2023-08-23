fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let a: usize = io.get()?;
		let b: usize = io.get()?;
		let c: usize = io.get()?;
		let a = if c % 2 == 1 { a + 1 } else { a };
		if a > b {
			io.put("First")?;
		} else {
			io.put("Second")?;
		}
	}

	Ok(())
}
