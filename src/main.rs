mod design_patterns;
pub mod problems;
use design_patterns::prototype::{Circle, Prototype};

fn main() {
    let circle1 = Circle { radius: 5.0 };
    let circle2 = circle1.clone_box();

    println!("Circle1: {:?}", circle1);
    println!("Circle2: {:?}", circle2);
}
