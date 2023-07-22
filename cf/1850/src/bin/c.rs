fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();

    // 被意料之外的人关心了。有一说一，有点开心

    let t: usize = io.read()?;
    for _ in 0..t {
        let mut s = Vec::with_capacity(8);
        for _ in 0..8 {
            for c in io.read_bytes(8)? {
                if c != b'.' {
                    s.push(c);
                }
            }
        }
        io.print_str(&s)?;
    }

    Ok(())
}
