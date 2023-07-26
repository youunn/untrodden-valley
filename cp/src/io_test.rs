use std::{
	error::Error,
	fmt::Display,
	io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
	ptr::NonNull,
	str::{from_utf8_unchecked, FromStr, SplitWhitespace},
};

pub struct Buf<'a> {
	// never mutate the buffer directly
	buf: Vec<u8>,
	iter: SplitWhitespace<'a>,
}

impl<'a> Buf<'a> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn modify<F>(&mut self, f: F) -> Result<(), Box<dyn Error>>
	where
		F: FnOnce(&mut Vec<u8>) -> Result<(), Box<dyn Error>>,
	{
		f(&mut self.buf)?;
		let ptr = NonNull::from(&self.buf);
		let buf = unsafe { from_utf8_unchecked(ptr.as_ref()) };
		self.iter = buf.split_whitespace();
		Ok(())
	}
}

impl<'a> Default for Buf<'a> {
	fn default() -> Self {
		Self {
			buf: Default::default(),
			iter: "".split_whitespace(),
		}
	}
}

impl<'a> Iterator for Buf<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}
}

pub struct GeneralIO<'a, I, O> {
	scan: I,
	out: O,
	buf: Buf<'a>,
}

impl<I: BufRead, O: Write> GeneralIO<'_, I, O> {
	fn new(scan: I, out: O) -> Self {
		Self {
			scan,
			out,
			buf: Buf::new(),
		}
	}

	pub fn get<T>(&mut self) -> Result<T, Box<dyn Error>>
	where
		T: FromStr,
		<T as FromStr>::Err: Error + 'static,
	{
		loop {
			if let Some(s) = self.buf.next() {
				return Ok(s.parse()?);
			}
			self.buf.modify(|buf| {
				buf.clear();
				self.scan.read_until(b'\n', buf)?;
				Ok(())
			})?;
		}
	}

	pub fn put<T: Display>(&mut self, value: T) -> Result<(), Box<dyn Error>> {
		writeln!(self.out, "{}", value)?;
		Ok(())
	}
}

type IO<'a> = GeneralIO<'a, StdinLock<'a>, BufWriter<StdoutLock<'a>>>;

impl Default for IO<'_> {
	fn default() -> Self {
		Self::new(stdin().lock(), BufWriter::new(stdout().lock()))
	}
}

pub fn default<'a>() -> IO<'a> {
	IO::default()
}
