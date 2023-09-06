use std::ops::{Div, DivAssign};

#[allow(non_camel_case_types)]
pub struct max;
#[allow(non_camel_case_types)]
pub struct max_partial<T: Ord>(T);
impl<T: Ord> Div<T> for max_partial<T> {
    type Output = T;
    fn div(self, rhs: T) -> Self::Output {
        self.0.max(rhs)
    }
}
impl<T: Ord> Div<T> for max {
    type Output = max_partial<T>;
    fn div(self, rhs: T) -> Self::Output {
        max_partial(rhs)
    }
}

#[allow(non_camel_case_types)]
pub struct min;
#[allow(non_camel_case_types)]
pub struct min_partial<T: Ord>(T);
impl<T: Ord> Div<T> for min_partial<T> {
    type Output = T;
    fn div(self, rhs: T) -> Self::Output {
        self.0.min(rhs)
    }
}
impl<T: Ord> Div<T> for min {
    type Output = min_partial<T>;
    fn div(self, rhs: T) -> Self::Output {
        min_partial(rhs)
    }
}

macro_rules! cmp_impl {
    ($t:ty) => {
        impl Div<max> for $t {
            type Output = max_partial<$t>;
            fn div(self, _: max) -> Self::Output {
                max_partial(self)
            }
        }
        impl DivAssign<max_partial<$t>> for $t {
            fn div_assign(&mut self, rhs: max_partial<$t>) {
                *self = (*self).max(rhs.0)
            }
        }
        impl Div<min> for $t {
            type Output = min_partial<$t>;
            fn div(self, _: min) -> Self::Output {
                min_partial(self)
            }
        }
        impl DivAssign<min_partial<$t>> for $t {
            fn div_assign(&mut self, rhs: min_partial<$t>) {
                *self = (*self).min(rhs.0)
            }
        }
    };
    ($($t:ty),+) => {
        $(cmp_impl!($t);)+
    };
}

macro_rules! cmp_enum_impl {
    ($t:ty) => {
        cmp_impl!($t);
        cmp_impl!(Option<$t>);
    };
    ($($t:ty),+) => {
        $(cmp_enum_impl!($t);)+
    };
}

cmp_enum_impl!(u8, u16, u32, u64, u128, usize);
cmp_enum_impl!(i8, i16, i32, i64, i128, isize);
