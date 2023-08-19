use std::{
	error::Error,
	fmt::Display,
	io::{stdin, stdout, BufRead, BufWriter, StdinLock, StdoutLock, Write},
	ptr::NonNull,
	str::{from_utf8_unchecked, FromStr, SplitWhitespace},
};

pub struct Buf<'a> {
	buf: Vec<u8>,
	iter: SplitWhitespace<'a>,
}

impl<'a> Buf<'a> {
	pub fn modify<F>(&mut self, f: F) -> std::io::Result<()>
	where
		F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
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
			res.ok()?;
		}
	}
}

pub struct GeneralIO<'a, I, O> {
	pub scan: Scan<'a, I>,
	pub out: O,
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

	pub fn get<T>(&mut self) -> Result<T, <T as FromStr>::Err>
	where
		T: FromStr,
		<T as FromStr>::Err: Error + 'static,
	{
		self.scan.next().unwrap_or("").parse()
	}

	pub fn get_vec<T, V>(&mut self, n: usize) -> Result<V, <T as FromStr>::Err>
	where
		T: FromStr,
		<T>::Err: Error + 'static,
		V: FromIterator<T>,
	{
		self.scan
			.by_ref()
			.take(n)
			.map(|s| s.parse::<T>())
			.collect::<Result<V, _>>()
	}

	pub fn put<T: Display>(&mut self, value: T) -> std::io::Result<()> {
		writeln!(self.out, "{}", value)
	}

	pub fn put_vec<T: Display>(
		&mut self,
		mut values: impl Iterator<Item = T>,
	) -> std::io::Result<()> {
		if let Some(v) = values.next() {
			write!(self.out, "{}", v)?;
			for v in values {
				write!(self.out, " {}", v)?;
			}
		}
		writeln!(self.out)
	}
}

impl<I: BufRead, O: Write> Write for GeneralIO<'_, I, O> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.out.write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.out.flush()
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
