fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let t: usize = io.get()?;
	for _ in 0..t {
		let n: usize = io.get()?;
		let mut mat = Vec::<Vec<_>>::with_capacity(n);
		for _ in 0..n {
			let s: String = io.get()?;
			mat.push(s.into_bytes().into_iter().map(|x| x - b'0').collect());
		}

		let mut q = vec![vec![0; n]; n];

		// 一个大三角形可以分为两个小三角形减去重叠的部分再加上顶点

		let mut ans = 0;
		for i in 0..n {
			for j in 0..n {
				let x = mat[i][j] ^ q[i][j];
				if x == 1 {
					ans += 1;
				}
				if i < n - 1 {
					let b = mat[i][j];
					mat[i + 1][j] ^= b;
					let mut flag = true;
					if j >= 1 {
						q[i + 1][j - 1] ^= b;
						flag = !flag;
					}
					if j < n - 1 {
						q[i + 1][j + 1] ^= b;
						flag = !flag;
					}
					if flag && i < n - 2 {
						q[i + 2][j] ^= b;
					}
				}
			}
		}

		io.put(ans)?;
	}

	Ok(())
}
