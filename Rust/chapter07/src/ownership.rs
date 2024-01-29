// #[allow(dead_code)]
// pub fn ownership() {
//     let x = String::from("ABC"); //String doesn't implement Copy trait, so x is moved to y
//     println!("x: {:?}", x);
//     let y = x;
//     println!("x: {:?}", x);
//     println!("y: {:?}", y);
// }

#[allow(dead_code)]
pub fn ownership_2() {
    let x = String::from("ABC");
    println!("x: {:?}", x);
    let y = &x; // y is a reference to x
    println!("x: {:?}", x);
    println!("y: {:?}", y);
}
