#[allow(dead_code)]
pub fn compound_assign_method(mut x: i32, y: i32) {
    use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    x.add_assign(y);
    println!("x + y = {}", x);
    x.sub_assign(y);
    println!("x - y = {}", x);
    x.mul_assign(y);
    println!("x * y = {}", x);
    x.div_assign(y);
    println!("x / y = {}", x);
    x.rem_assign(y);
    println!("x % y = {}", x);
}
