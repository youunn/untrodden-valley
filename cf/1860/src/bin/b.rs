fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let m: isize = io.get()?;
		let k: isize = io.get()?;
		let a1: isize = io.get()?;
		let ak: isize = io.get()?;

		let (eak, ea1) = (m / k, m % k);

		let f1 = (ea1 - a1).max(0);
		let r1 = (a1 - ea1).max(0);
		let fk = (eak - ak).max(0);
		let rk = (r1 / k).min(fk);
		let ans = f1 + fk - rk;
		io.put(ans)?;
	}

	Ok(())
}
