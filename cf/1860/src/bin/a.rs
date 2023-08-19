fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let s: String = io.get()?;
		if s == "()" {
			io.put("NO")?;
			continue;
		}
		let len = s.len();
		io.put("YES")?;
		if s.contains(")(") {
			for _ in 0..len {
				write!(io, "(")?;
			}
			for _ in 0..len {
				write!(io, ")")?;
			}
		} else {
			for _ in 0..len {
				write!(io, "()")?;
			}
		}
		writeln!(io)?;
	}

	Ok(())
}
