#[derive(Default)]
pub struct Vector3T<T> {
    x: T,
    y: T,
    z: T,
}
pub type Vector3 = Vector3T<f64>;

impl<T> Vector3T<T> {
    pub fn new(v1: T, v2: T, v3: T) -> Self {
        Self {
            x: v1,
            y: v2,
            z: v3,
        }
    }
}

impl<T, U> From<U> for Vector3T<T>
where
    T: From<U>,
{
    fn from(v: U) -> Self {
        Self {
            x: T::from(v),
            y: T::from(v),
            z: T::from(v),
        }
    }
}
