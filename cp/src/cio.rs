use std::{
	error::Error,
	fmt::{Arguments, Display},
	io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
	ptr::NonNull,
	str::{from_utf8_unchecked, FromStr, SplitWhitespace},
};

pub struct Buf<'a> {
	buf: Vec<u8>,
	iter: SplitWhitespace<'a>,
}

impl<'a> Buf<'a> {
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

pub struct Scan<'a, I> {
	scan: I,
	buf: Buf<'a>,
}

impl<'a, I: BufRead> Iterator for Scan<'a, I> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(s) = self.buf.next() {
				return Some(s);
			}
			let res = self.buf.modify(|buf| {
				buf.clear();
				self.scan.read_until(b'\n', buf)?;
				Ok(())
			});
			if res.is_err() {
				return None;
			}
		}
	}
}

pub struct GeneralIO<'a, I, O> {
	scan: Scan<'a, I>,
	out: O,
}

impl<I: BufRead, O: Write> GeneralIO<'_, I, O> {
	fn new(scan: I, out: O) -> Self {
		Self {
			scan: Scan {
				scan,
				buf: Buf::default(),
			},
			out,
		}
	}

	pub fn get<T>(&mut self) -> Result<T, Box<dyn Error>>
	where
		T: FromStr,
		<T as FromStr>::Err: Error + 'static,
	{
		Ok(self.scan.next().ok_or("No next element")?.parse()?)
	}

	pub fn get_vec<T, V>(&mut self, n: usize) -> Result<V, Box<dyn Error>>
	where
		T: FromStr,
		<T>::Err: Error + 'static,
		V: FromIterator<T>,
	{
		Ok(self.scan
			.by_ref()
			.take(n)
			.map(|s| s.parse::<T>())
			.collect::<Result<V, _>>()?)
	}

	pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
		self.out.write_fmt(fmt)
	}

	pub fn put<T: Display>(&mut self, value: T) -> Result<(), Box<dyn Error>> {
		writeln!(self.out, "{}", value)?;
		Ok(())
	}

	pub fn put_vec<T: Display>(
		&mut self,
		mut values: impl Iterator<Item = T>,
	) -> Result<(), Box<dyn Error>> {
		if let Some(v) = values.next() {
			write!(self.out, "{}", v)?;
			for v in values {
				write!(self.out, " {}", v)?;
			}
		}
		writeln!(self.out)?;
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
