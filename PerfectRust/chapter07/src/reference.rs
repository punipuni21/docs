// #[allow(dead_code)]
// pub fn reference_1() {
//     let x = Vec::<i32>::new(); //immutable vector
//     let y = &mut x; // mutable value cannot be borrowed from an immutable vector
//     y.push(1);
//     y.push(2);
//     y.push(3);
//     println!("y: {:?}", y);
// }

#[allow(dead_code)]
pub fn reference_2() {
    let mut x = Vec::<i32>::new(); //mmutable vector
    let y = &mut x;
    y.push(1);
    y.push(2);
    y.push(3);
    println!("y: {:?}", y);
}

#[allow(dead_code)]
pub fn reference_3() {
    //if mutable reference is allowed, missing value may be substituted.
    let mut x = Vec::<i32>::new(); //mmutable vector
    let y = &mut x;
    let z = &mut x; // murtable reference must be unique
    y.push(100);
    y.push(200);
    println!("{:?}", x);
}
