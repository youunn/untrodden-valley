fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::io_test::IO::new();
	let t: usize = io.read()?;
	let n: usize = io.read()?;
	io.print(t + n)?;

	Ok(())
}
