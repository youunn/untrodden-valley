fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::io::default();

	const N: usize = 30;
	let mut f = Vec::with_capacity(N);
	f.push(0);
	f.push(1);
	for i in 0..N - 2 {
		f.push(f[i] + f[i + 1]);
	}
	// 0 1 1 2 3 5 8

	let t: usize = io.read()?;
	for _t in 0..t {
		let (n, k): (i64, i64) = io.read2()?;

		if k as usize > N || f[k as usize - 1] > n {
			io.print(0)?;
			continue;
		}

		let k = k as usize;

		// f_{k-2} \cdot x + f_{k-1} \cdot y = n
		// ax + by = n
		// gcd(a, b) = 1 // 更相减损
		// |f_{k-2} \cdot f_{k+1} - f_{k-1} \cdot f_{k}| = 1
		// |n \cdot f_{k-2} \cdot f_{k+1} - n \cdot f_{k-1} \cdot f_{k}| = n
		let a = f[k - 2];
		let b = f[k - 1];
		let ans = if k % 2 == 1 {
			// f_{k-2} \cdot f_{k+1} - f_{k-1} \cdot f_{k} = 1
			// n \cdot f_{k-2} \cdot f_{k+1} - n \cdot f_{k-1} \cdot f_{k} = n
			let x = n * f[k + 1];
			let y = -(n * f[k]);
			// x = x_0 + c \cdot b
			// y = y_0 - c \cdot a
			// x \geq 0 \Rightarrow c \geq -frac x b
			// y \geq x \Rightarrow c \leq frac {y - x} / {b + a}
			let r1 = if (y - x) % (b + a) == 0 { 1 } else { 0 };
			// let r2 = if x % b == 0 { 1 } else { 0 };
			let r2 = 0;
			(y - x) / (b + a) + x / b + r1 + r2
		} else {
			// f_{k-2} \cdot f_{k+1} - f_{k-1} \cdot f_{k} = -1
			// n \cdot f_{k-2} \cdot f_{k+1} - n \cdot f_{k-1} \cdot f_{k} = -n
			let x = -(n * f[k + 1]);
			let y = n * f[k];
			// x = x_0 + c \cdot b
			// y = y_0 - c \cdot a
			// x \geq 0 \Rightarrow c \geq -frac x b
			// y \geq x \Rightarrow c \leq frac {y - x} / {b + a}
			// let r1 = if (y - x) % (b + a) == 0 { 1 } else { 0 };
			let r1 = 0;
			let r2 = if x % b == 0 { 1 } else { 0 };
			(y - x) / (b + a) + x / b + r1 + r2
		};

		io.print(ans.max(0))?;
	}

	Ok(())
}
