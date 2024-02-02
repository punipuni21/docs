struct Coodinates(usize, usize);
#[allow(dead_code)]
pub fn generate_tuple() {
    let c = Coodinates(10, 20);
    println!("x:{}", c.0);
    println!("y:{}", c.1);
}

use std::borrow::Borrow;
struct OneState;
#[allow(dead_code)]
pub fn generate_unit() {
    let o = OneState;
    println!("{:p}", o.borrow());
}
