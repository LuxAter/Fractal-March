use crate::transform::*;
pub trait Sdf {
    fn dist(&self, p: na::Point3<f64>) -> f64;
}
macro_rules! transformable_impl {
    ($t: ty) => {
        impl Transformable for $t {
            fn rotate_vec(&mut self, angle: f64, axis: &na::Vector3<f64>) -> &mut Self {
                self.trans.rotate_vec(angle, axis);
                return self;
            }
            fn scale_vec(&mut self, scalar: &na::Vector3<f64>) -> &mut Self {
                self.trans.scale_vec(scalar);
                return self;
            }
            fn translate_vec(&mut self, translate: &na::Vector3<f64>) -> &mut Self {
                self.trans.translate_vec(translate);
                return self;
            }
        }
    };
}

#[derive(Debug, TypedBuilder)]
pub struct Sphere {
    radius: f64,
    #[builder(default)]
    trans: Transform,
}
impl Sphere {
    pub fn new(radius: f64) -> Self {
        Self {
            radius: radius,
            trans: Transform::default(),
        }
    }
}

transformable_impl!(Sphere);
impl Sdf for Sphere {
    fn dist(&self, p: na::Point3<f64>) -> f64 {
        return (self.trans * p).coords.norm() - self.radius;
    }
}
