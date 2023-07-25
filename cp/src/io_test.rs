use std::{
	error::Error,
	fmt::Display,
	io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
	str::{FromStr, SplitWhitespace},
};

pub struct IO<'a> {
	scan: StdinLock<'a>,
	out: BufWriter<StdoutLock<'a>>,
	buf: String,
	split: SplitWhitespace<'a>,
}

impl<'a> IO<'a> {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			scan: stdin().lock(),
			out: BufWriter::new(stdout().lock()),
			buf: String::new(),
			split: "".split_whitespace(),
		}
	}

	pub fn read<T>(&mut self) -> Result<T, Box<dyn Error>>
	where
		T: FromStr,
		<T as FromStr>::Err: Error + 'static,
	{
		loop {
			if let Some(s) = self.split.next() {
				return Ok(s.parse()?);
			}
			self.buf.clear();
			self.scan.read_line(&mut self.buf)?;
			self.split = unsafe { &*(self as *mut IO<'_>) }.buf.split_whitespace();
		}
	}

	pub fn print<T: Display>(&mut self, value: T) -> Result<(), Box<dyn Error>> {
		writeln!(self.out, "{}", value)?;
		Ok(())
	}
}
