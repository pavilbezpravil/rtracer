extern crate rtracer;

use rtracer::{Vec3};

fn main() {
    let vec = Vec3::new(1., 2., 3.);

    let ser = serde_json::to_string(&vec).unwrap();

    println!("{}", ser);
}
