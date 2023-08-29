fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();

	{
		let n: usize = io.get()?;
		let q: usize = io.get()?;
		let v = io.get_vec::<_, Vec<usize>>(n)?;

		for _ in 0..q {
			let l: usize = io.get()?;
			let r: usize = io.get()?;
			io.put(v[0] + l + r)?;
		}
	}

	Ok(())
}
