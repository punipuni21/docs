struct Coodinates(usize, usize);
#[allow(dead_code)]
pub fn generate_tuple() {
    let c = Coodinates(10, 20);
    println!("x:{}", c.0);
    println!("y:{}", c.1);
}
