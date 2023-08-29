fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let x: i32 = io.get()?;
		let y: i32 = io.get()?;
		let n: usize = io.get()?;

		let mut rev = Vec::new();
		let mut c = y;

		for i in 1..n {
			rev.push(c);
			c -= i as i32;
		}
		rev.push(x);

		if c < x {
			io.put("-1")?;
		} else {
			io.put_iter(rev.into_iter().rev())?;
		}
	}

	Ok(())
}
