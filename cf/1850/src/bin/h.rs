fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();
    let t: usize = io.read()?;
    't: for _ in 0..t {
        let (n, m): (usize, usize) = io.read2()?;
        let mut v = Vec::with_capacity(m);
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let (mut a, mut b, d) = io.read3::<usize, usize, i64>()?;
            a -= 1;
            b -= 1;

            v.push((a, b, d));
            g[a].push((b, d));
            g[b].push((a, -d));
        }

        let mut s = vec![None; n];
        let mut visited = vec![false; n];
        let mut stack = Vec::with_capacity(n);
        for i in 0..n {
            if !visited[i] {
                stack.push(i);
                s[i] = Some(0);
            }
            while let Some(i) = stack.pop() {
                if !visited[i] {
                    visited[i] = true;
                }
                for &(j, d) in &g[i] {
                    if !visited[j] {
                        s[j] = Some(s[i].unwrap_or_default() + d);
                        stack.push(j);
                    }
                }
            }
        }

        for (a, b, d) in v {
            if let (Some(sa), Some(sb)) = (s[a], s[b]) {
                if sa + d == sb {
                    continue;
                }
            }
            io.print("NO")?;
            continue 't;
        }

        io.print("YES")?;
    }

    Ok(())
}
