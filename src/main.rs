extern crate raymarch;

fn main() {
    let mut scene = raymarch::Scene::new();
    assert!(raymarch::render().is_ok());
}
