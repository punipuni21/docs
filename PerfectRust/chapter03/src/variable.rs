#[allow(dead_code)]
pub fn shadowing() {
    let value1: i32 = 100;
    println!("value1 = {}", value1);
    let value1: i32 = 200;
    println!("value1 = {}", value1);
    let value2: f32 = 100.1;
    println!("value2 = {}", value2);
    let value2: i32 = 200;
    println!("value2 = {}", value2);
}
