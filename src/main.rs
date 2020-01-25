extern crate raymarch;

use raymarch::*;

fn main() {
    let mut objs: Vec<Box<dyn Sdf + Sync>> = Vec::new();
    objs.push(Box::new(object::Sphere::new(1.0f64)));
    raymarch::render(raymarch::RenderArgs::builder().objects(objs).build()).unwrap();
}
