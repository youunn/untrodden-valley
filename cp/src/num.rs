use std::ops::{
	Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

#[derive(Clone, Copy)]
pub struct Number<const M: u64>(pub u64);

impl<const M: u64> Number<M> {
	pub fn new(x: u64) -> Self {
		Self(x % M)
	}

	fn pow(mut self, mut b: u64) -> Self {
		let mut ans = Self(1);
		while b > 0 {
			if b % 2 == 1 {
				ans *= self;
			}
			self *= self;
			b /= 2;
		}
		ans
	}

	fn inv(self) -> Self {
		self.pow(M - 2)
	}
}

impl<const M: u64> From<u64> for Number<M> {
	fn from(x: u64) -> Self {
		Self::new(x)
	}
}

impl<const M: u64> Add for Number<M> {
	type Output = Number<M>;
	fn add(self, rhs: Self) -> Self::Output {
		Self((self.0 + rhs.0) % M)
	}
}
impl<const M: u64> Sub for Number<M> {
	type Output = Number<M>;
	fn sub(self, rhs: Self) -> Self::Output {
		let mut lhs = self.0 % M;
		let rhs = rhs.0 % M;
		if lhs < rhs {
			lhs += M;
		}
		Self((lhs - rhs) % M)
	}
}
impl<const M: u64> Mul for Number<M> {
	type Output = Number<M>;
	fn mul(self, rhs: Self) -> Self::Output {
		Self((self.0 * rhs.0) % M)
	}
}
impl<const M: u64> Div for Number<M> {
	type Output = Number<M>;
	fn div(self, rhs: Self) -> Self::Output {
		Mul::mul(self, rhs.inv())
	}
}

impl<const M: u64> AddAssign for Number<M> {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs
	}
}
impl<const M: u64> SubAssign for Number<M> {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}
impl<const M: u64> MulAssign for Number<M> {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs
	}
}
impl<const M: u64> DivAssign for Number<M> {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs
	}
}

impl<const M: u64> BitXor for Number<M> {
	type Output = Number<M>;
	fn bitxor(self, rhs: Self) -> Self::Output {
		self.pow(rhs.0)
	}
}
impl<const M: u64> BitXorAssign for Number<M> {
	fn bitxor_assign(&mut self, rhs: Self) {
		*self = *self ^ rhs
	}
}

impl<const M: u64> Add<u64> for Number<M> {
	type Output = Number<M>;
	fn add(self, rhs: u64) -> Self::Output {
		Self((self.0 + rhs) % M)
	}
}
impl<const M: u64> Sub<u64> for Number<M> {
	type Output = Number<M>;
	fn sub(self, rhs: u64) -> Self::Output {
		let mut lhs = self.0 % M;
		let rhs = rhs % M;
		if lhs < rhs {
			lhs += M;
		}
		Self((lhs - rhs) % M)
	}
}
impl<const M: u64> Mul<u64> for Number<M> {
	type Output = Number<M>;
	fn mul(self, rhs: u64) -> Self::Output {
		Self((self.0 * rhs) % M)
	}
}
impl<const M: u64> Div<u64> for Number<M> {
	type Output = Number<M>;
	fn div(self, rhs: u64) -> Self::Output {
		Div::div(self, Self::new(rhs))
	}
}

impl<const M: u64> AddAssign<u64> for Number<M> {
	fn add_assign(&mut self, rhs: u64) {
		*self = *self + rhs
	}
}
impl<const M: u64> SubAssign<u64> for Number<M> {
	fn sub_assign(&mut self, rhs: u64) {
		*self = *self - rhs
	}
}
impl<const M: u64> MulAssign<u64> for Number<M> {
	fn mul_assign(&mut self, rhs: u64) {
		*self = *self * rhs
	}
}
impl<const M: u64> DivAssign<u64> for Number<M> {
	fn div_assign(&mut self, rhs: u64) {
		*self = *self / rhs
	}
}

impl<const M: u64> BitXor<u64> for Number<M> {
	type Output = Number<M>;
	fn bitxor(self, rhs: u64) -> Self::Output {
		self.pow(rhs)
	}
}
impl<const M: u64> BitXorAssign<u64> for Number<M> {
	fn bitxor_assign(&mut self, rhs: u64) {
		*self = *self ^ rhs
	}
}
