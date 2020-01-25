extern crate raymarch;

use raymarch::*;

fn main() {
    raymarch::render().unwrap();
    let mut interp = spline::LinearInterp::<f64>::new();
    interp.set(0.0, 0.0).set(0.5, 20.0).set(1.0, 10.0);
    println!("VALS: {:?}", interp.values);
    for t in 0..100 {
        println!(
            "  {}:{}",
            t as f64 / 100.0,
            interp.interp(t as f64 / 100.0f64)
        );
    }
}
