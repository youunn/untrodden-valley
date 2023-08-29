use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut io = cp::cio::default();
	let n: usize = io.get()?;
	let mut l = VecDeque::<usize>::new();
	let mut r = VecDeque::<usize>::new();
	let mut a = VecDeque::<usize>::new();
	for i in (1..=n).rev() {
		if i % 2 == 1 {
			a.push_back(i);
			l.extend(a.iter());
		} else {
			a.push_front(i);
			r.extend(a.iter().rev());
		}
	}
	dbg!(&l, &r);
	l.extend(r.iter().rev());
	if n % 2 == 1 {
		let len = l.len();
		l.swap(len - 2, len - 1);
	} else {
		l.swap(0, 1);
	}
	io.put_vec(l.into_iter())?;

	Ok(())
}
