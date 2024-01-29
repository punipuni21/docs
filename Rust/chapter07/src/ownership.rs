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

#[allow(dead_code)]
pub fn ownership_3() {
    //Clone use heap memory, so this is called deep copy, which is different from shallow copy
    //Clone does not have ownership, which means the original variable is still valid.
    let x = String::from("ABC");
    println!("x: {:?}", x);
    let y = x.clone(); // y is a clone of x.
    println!("x: {:?}", x);
    println!("y: {:?}", y);
}
