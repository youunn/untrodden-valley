fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::io_test::default();
	let t: usize = io.get()?;
	let n: usize = io.get()?;
	io.put(t + n)?;

	Ok(())
}
