// extern crate nalgebra as na;
// pub struct Transform {
//     __scale: na::Vector3<f32>,
//     __translate: na::Vector3<f32>,
//     __rotate: na::Quaternion<f32>,
// }
// pub struct Object {
//     transformm: Transform,
//     dist: fn(na::Vector3<f32>) -> f32,
// }
// pub struct Camera {
//     fov: f32,
//     transform: Transform,
// }
// pub struct Scene {
//     camera: Camera,
//     objects: Vec<Object>,
// }
// impl Transform {
//     pub fn new() -> Self {
//         Self {
//             __scale: na::Vector3::<f32>::new(1.0, 1.0, 1.0),
//             __translate: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
//             __rotate: na::Quaternion::<f32>::new(0.0, 0.0, 0.0, 1.0),
//         }
//     }
//     pub fn translate(self, xyz: na::Vector3<f32>) -> Self {
//         self.__translate = xyz;
//         return self;
//     }
//     pub fn scale(self, xyz: na::Vector3<f32>) -> Self {
//         self.__scale = xyz;
//         return self;
//     }
//     pub fn rotate(self, angle: f32, axis: na::Vector3<f32>) -> Self {}
// }
// impl Camera {
//     pub fn new() -> Self {
//         Self {
//             fov: std::f32::consts::PI / 2f32,
//             transform: Transform::new(),
//         }
//     }
// }
// impl Scene {
//     pub fn new() -> Self {
//         Self {
//             camera: Camera::new(),
//             objects: vec![],
//         }
//     }
// }
