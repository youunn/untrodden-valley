fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: u8 = io.get()?;
		let odd = io
			.get_vec::<u8, Vec<_>>(n as usize)?
			.into_iter()
			.filter(|a| a % 2 == 1)
			.count();

		if odd % 2 == 1 {
			io.put("NO")?;
		} else {
			io.put("YES")?;
		}
	}

	Ok(())
}
