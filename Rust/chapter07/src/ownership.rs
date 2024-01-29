#[allow(dead_code)]
pub fn ownership() {
    let x = String::from("ABC"); //String doesn't implement Copy trait, so x is moved to y
    println!("x: {}", x);
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);
}
