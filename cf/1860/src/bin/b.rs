fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let m: usize = io.get()?;
		let k: usize = io.get()?;
		let a1: usize = io.get()?;
		let ak: usize = io.get()?;

		let (eak, ea1) = (m / k, m % k);

		let ans = if ak >= eak {
			if a1 >= ea1 {
				0
			} else {
				ea1 - a1
			}
		} else {
			let em = ea1 + k * (eak - ak);
			if a1 >= em {
				0
			} else {
				let rm = em - a1;
				let (d, r) = (rm / k, rm % k);
				dbg!(rm, d, r);
				if r == 0 {
					d
				} else if a1 >= k - r {
					d + 1
				} else {
					d + r
				}
			}
		};
		io.put(ans)?;
	}

	Ok(())
}
