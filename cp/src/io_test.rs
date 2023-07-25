use std::{
	error::Error,
	fmt::Display,
	io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
	str::FromStr,
};

pub struct GeneralIO<I, O> {
	scan: I,
	out: O,
	buf: String,
}

impl<I: BufRead, O: Write> GeneralIO<I, O> {
	fn new(scan: I, out: O) -> Self {
		Self {
			scan,
			out,
			buf: String::new(),
		}
	}

	pub fn get<T>(&mut self) -> Result<T, Box<dyn Error>>
	where
		T: FromStr,
		<T as FromStr>::Err: Error + 'static,
	{
		loop {
			if let Some(s) = self.buf.split_whitespace().next() {
				return Ok(s.parse()?);
			}
			self.buf.clear();
			self.scan.read_line(&mut self.buf)?;
		}
	}

	pub fn put<T: Display>(&mut self, value: T) -> Result<(), Box<dyn Error>> {
		writeln!(self.out, "{}", value)?;
		Ok(())
	}
}

pub type IO<'a> = GeneralIO<StdinLock<'a>, BufWriter<StdoutLock<'a>>>;

impl Default for IO<'_> {
	fn default() -> Self {
		Self::new(stdin().lock(), BufWriter::new(stdout().lock()))
	}
}
