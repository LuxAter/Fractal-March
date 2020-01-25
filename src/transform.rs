use std::ops::Mul;
extern crate nalgebra as na;

pub trait Transformable {
    fn rotate_vec(&mut self, angle: f64, axis: &na::Vector3<f64>) -> &mut Self;
    fn scale_vec(&mut self, scalar: &na::Vector3<f64>) -> &mut Self;
    fn translate_vec(&mut self, translate: &na::Vector3<f64>) -> &mut Self;
    fn rotate(&mut self, angle: f64, x: f64, y: f64, z: f64) -> &mut Self {
        return self.rotate_vec(angle, &na::Vector3::<f64>::new(x, y, z));
    }
    fn rotate_x(&mut self, angle: f64) -> &mut Self {
        return self.rotate_vec(angle, &na::Vector3::<f64>::x());
    }
    fn rotate_y(&mut self, angle: f64) -> &mut Self {
        return self.rotate_vec(angle, &na::Vector3::<f64>::x());
    }
    fn rotate_z(&mut self, angle: f64) -> &mut Self {
        return self.rotate_vec(angle, &na::Vector3::<f64>::x());
    }
    fn scale_unif(&mut self, scalar: f64) -> &mut Self {
        return self.scale_vec(&na::Vector3::<f64>::new(scalar, scalar, scalar));
    }
    fn scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        return self.scale_vec(&na::Vector3::<f64>::new(x, y, z));
    }
    fn translate(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        return self.translate_vec(&na::Vector3::<f64>::new(x, y, z));
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    trans: na::Matrix4<f64>,
    inv: na::Matrix4<f64>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            trans: na::Matrix4::<f64>::identity(),
            inv: na::Matrix4::<f64>::identity(),
        }
    }
}

impl std::default::Default for Transform {
    fn default() -> Self {
        Self {
            trans: na::Matrix4::<f64>::identity(),
            inv: na::Matrix4::<f64>::identity(),
        }
    }
}

impl Transformable for Transform {
    fn rotate_vec(&mut self, angle: f64, axis: &na::Vector3<f64>) -> &mut Self {
        self.trans = self.trans
            * na::Rotation3::from_axis_angle(&na::Unit::new_normalize(*axis), angle)
                .to_homogeneous();
        self.inv = self.inv
            * na::Rotation3::from_axis_angle(&na::Unit::new_normalize(*axis), -angle)
                .to_homogeneous();
        return self;
    }
    fn scale_vec(&mut self, scalar: &na::Vector3<f64>) -> &mut Self {
        self.trans.prepend_nonuniform_scaling(scalar);
        self.inv.prepend_nonuniform_scaling(&na::Vector3::new(
            1.0 / scalar[0],
            1.0 / scalar[1],
            1.0 / scalar[2],
        ));
        return self;
    }
    fn translate_vec(&mut self, translate: &na::Vector3<f64>) -> &mut Self {
        self.trans = self.trans * na::Translation3::from(*translate).to_homogeneous();
        self.inv = self.inv * na::Translation3::from(-translate).to_homogeneous();
        return self;
    }
}

impl Mul<na::Vector3<f64>> for Transform {
    type Output = na::Vector3<f64>;
    fn mul(self, vec: na::Vector3<f64>) -> na::Vector3<f64> {
        return self.trans.transform_vector(&vec);
    }
}
impl Mul<na::Point3<f64>> for Transform {
    type Output = na::Point3<f64>;
    fn mul(self, pt: na::Point3<f64>) -> na::Point3<f64> {
        return self.trans.transform_point(&pt);
    }
}
