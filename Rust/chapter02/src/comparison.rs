#[allow(dead_code)]
pub fn methods(x: i32, y: i32) {
    // use std::cmp::{PartialEq, PartialOrd};//mod.rsで既にimportされているので不要
    println!("x == y: {}", x.eq(&y));
    println!("x != y: {}", x.ne(&y));
    println!("x > y: {}", x.gt(&y));
}
