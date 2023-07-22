fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut io = cp::io::default();
    let t: u8 = io.read()?;
    for _ in 0..t {
        let n: u8 = io.read()?;
        let mut mi = 0;
        let mut m = 0;
        for i in 0..n {
            let (a, b) = io.read2::<u8, u8>()?;
            if a <= 10 && b > m {
                mi = i;
                m = b;
            }
        }
        io.print(mi + 1)?;
    }

    Ok(())
}
