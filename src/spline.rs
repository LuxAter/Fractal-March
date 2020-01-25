use crate::identities::Zero;
pub use log::{debug, error, info, trace, warn};
use std::ops::{Add, Mul};

pub trait InterpolateConstructor {
    type Value;
    fn new() -> Self;
    fn set(&mut self, time: f64, val: Self::Value) -> &mut Self;
}
pub trait Interpolate {
    type Value;
    fn interp(&self, time: f64) -> Self::Value;
}

macro_rules! interpolate_impl {
    ($t: ty) => {
        impl<T: Add + Mul + Copy + Zero + Mul<f64, Output = T>> InterpolateConstructor for $t {
            type Value = T;
            fn new() -> Self {
                Self { values: vec![] }
            }
            fn set(&mut self, time: f64, val: T) -> &mut Self {
                let mut insert_index: Option<usize> = None;
                for (i, (t, x)) in self.values.iter_mut().enumerate() {
                    if *t == time {
                        *x = val;
                        break;
                    } else if *t > time {
                        insert_index = Some(i);
                        break;
                    }
                }
                if insert_index.is_some() {
                    self.values.insert(insert_index.unwrap(), (time, val));
                } else {
                    self.values.push((time, val));
                }
                return self;
            }
        }
    };
}

pub struct LinearInterp<T>
where
    T: Add + Mul + Copy + Zero + Mul<f64, Output = T>,
{
    pub values: Vec<(f64, T)>,
}
interpolate_impl!(LinearInterp<T>);
impl<T: Add + Mul + Copy + Zero + Mul<f64, Output = T>> Interpolate for LinearInterp<T> {
    type Value = T;
    fn interp(&self, time: f64) -> Self::Value {
        let mut pos: usize = 0usize;
        if self.values.len() == 0 {
            return T::zero();
        } else if self.values.len() == 1 {
            return self.values.first().unwrap().1;
        } else if time <= self.values.first().unwrap().0 {
            return self.values.first().unwrap().1;
        } else if time >= self.values.last().unwrap().0 {
            return self.values.last().unwrap().1;
        }
        for (i, (t, _x)) in self.values.iter().enumerate() {
            if *t > time {
                pos = i;
                break;
            }
        }
        let dt = (time - self.values[pos - 1].0) / (self.values[pos].0 - self.values[pos - 1].0);
        return self.values[pos - 1].1 * (1.0f64 - dt) + self.values[pos].1 * dt;
    }
}
