use std::{fmt::Debug, ops::Add};

pub struct SegmentTree<T>
where
	T: Add<Output = T> + Default + Clone + Copy + Debug,
{
	tree: Vec<T>,
	len: usize,
}

impl<T> SegmentTree<T>
where
	T: Add<Output = T> + Default + Clone + Copy + Debug,
{
	pub fn new(v: Vec<T>, n: usize) -> Self {
		let mut tree = vec![T::default(); n];
		tree.extend(v);

		for i in (1..n).rev() {
			tree[i] = tree[2 * i] + tree[2 * i + 1];
		}

		Self { tree, len: n }
	}

	pub fn update(&mut self, mut index: usize, value: T) {
		index += self.len;
		self.tree[index] = value;
		index /= 2;

		while index != 0 {
			self.tree[index] = self.tree[2 * index] + self.tree[2 * index + 1];
			index /= 2;
		}
	}

	pub fn query(&self, mut l: usize, mut r: usize) -> T {
		let mut ans = T::default();
		l += self.len;
		r += self.len;
		while l < r {
			if l % 2 == 1 {
				ans = ans + self.tree[l];
				l += 1;
			}
			if r % 2 == 1 {
				r -= 1;
				ans = ans + self.tree[r];
			}
			l /= 2;
			r /= 2;
		}
		ans
	}

	pub fn query_all(&self) -> T {
		self.tree[1]
	}
}
