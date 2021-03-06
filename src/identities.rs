pub trait Zero: Sized {
    fn zero() -> Self;
}
pub trait One: Sized {
    fn one() -> Self;
}

macro_rules! zero_impl {
    ($t: ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
        }
    };
}
macro_rules! one_impl {
    ($t: ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
        }
    };
}

zero_impl!(usize, 0usize);
zero_impl!(u8, 0u8);
zero_impl!(u16, 0u16);
zero_impl!(u32, 0u32);
zero_impl!(u64, 0u64);
zero_impl!(isize, 0isize);
zero_impl!(i8, 0i8);
zero_impl!(i16, 0i16);
zero_impl!(i32, 0i32);
zero_impl!(i64, 0i64);
zero_impl!(f32, 0.0f32);
zero_impl!(f64, 0.0f64);
zero_impl!(na::Point3<f64>, na::Point3::<f64>::new(0.0, 0.0, 0.0));
zero_impl!(na::Vector3<f64>, na::Vector3::<f64>::new(0.0, 0.0, 0.0));

one_impl!(usize, 1usize);
one_impl!(u8, 1u8);
one_impl!(u16, 1u16);
one_impl!(u32, 1u32);
one_impl!(u64, 1u64);
one_impl!(isize, 1isize);
one_impl!(i8, 1i8);
one_impl!(i16, 1i16);
one_impl!(i32, 1i32);
one_impl!(i64, 1i64);
one_impl!(f32, 1.0f32);
one_impl!(f64, 1.0f64);
