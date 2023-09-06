use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

#[derive(Clone, Copy)]
pub struct MutNumber {
    pub value: u64,
    modulus: u64,
}

impl MutNumber {
    pub fn new(value: u64, modulus: u64) -> Self {
        Self {
            value: value % modulus,
            modulus,
        }
    }

    fn pow(mut self, mut b: u64) -> Self {
        let mut ans = Self {
            value: 1,
            modulus: self.modulus,
        };
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
        self.pow(self.modulus - 2)
    }
}

impl Add for MutNumber {
    type Output = MutNumber;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value, self.modulus)
    }
}
impl Sub for MutNumber {
    type Output = MutNumber;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut lhs = self.value % self.modulus;
        let rhs = rhs.value % self.modulus;
        if lhs < rhs {
            lhs += self.modulus;
        }
        Self::new(lhs - rhs, self.modulus)
    }
}
impl Mul for MutNumber {
    type Output = MutNumber;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value, self.modulus)
    }
}
impl Div for MutNumber {
    type Output = MutNumber;
    fn div(self, rhs: Self) -> Self::Output {
        Mul::mul(self, rhs.inv())
    }
}

impl AddAssign for MutNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl SubAssign for MutNumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl MulAssign for MutNumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl DivAssign for MutNumber {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl BitXor for MutNumber {
    type Output = MutNumber;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.pow(rhs.value)
    }
}
impl BitXorAssign for MutNumber {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}
