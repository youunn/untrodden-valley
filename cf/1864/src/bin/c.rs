fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let mut x: usize = io.get()?;
		let mut ans = Vec::new();
		ans.push(x);

		let mut p;
		let mut i = 0;
		loop {
			if (x >> i) & 1 == 0 {
				i += 1;
				continue;
			}
			if x == 1 << i {
				p = i;
				break;
			}
			x -= 1 << i;
			ans.push(x);
			i += 1;
		}

		while p > 0 {
			x -= 1 << (p - 1);
			ans.push(x);
			p -= 1;
		}

		io.put(ans.len())?;
		io.put_iter(ans)?;
	}

	Ok(())
}
