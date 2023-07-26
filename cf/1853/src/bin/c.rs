fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let n: usize = io.get()?;
	let v: Vec<usize> = io.take(n)?;
	io.put(v.into_iter().sum::<usize>())?;

	Ok(())
}
