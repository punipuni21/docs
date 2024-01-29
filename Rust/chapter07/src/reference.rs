#[allow(dead_code)]
pub fn reference_1() {
    let x = Vec::<i32>::new(); //immutable vector
    let y = &mut x; // mutable value cannot be borrowed from an immutable vector
    y.push(1);
    y.push(2);
    y.push(3);
    println!("y: {:?}", y);
}
