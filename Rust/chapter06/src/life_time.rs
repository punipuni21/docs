#[allow(dead_code)]
pub fn life_time_1() {
    let x = vec![1, 2, 3];

    let a = String::from("ABC");

    let y = &x;
    let b = &a;

    println!("y = {:?}", y); // end of life time of y
    println!("b = {}", b);
    println!("b = {}", b); // end of life time of b
    println!("end program");
}

#[allow(dead_code)]
pub fn life_time_2() {
    let a = String::from("ABC");

    let b = &a; // end of life time of a
    let c = b; // end of life time of b

    println!("c = {}", c); // end of life time of c
    println!("end program");
}

// #[allow(dead_code)]
// pub fn life_time_3() -> &String {
//     let x = String::from("ABC");
//     &x // end of life time of x, so this is error
// }

#[allow(dead_code)]
// fn compare(value1: &String, value2: &String) -> &String {
//     // missing lifetime specifier
//     if (value1.len() > value2.len()) {
//         value1
//     } else {
//         value2
//     }
// }

// #[allow(dead_code)]
// pub fn life_time_4() {
//     let a = String::from("ABC");
//     let b = String::from("DEF");
//     let r = compare(&a, &b);
//     println!("r = {:?}", r);
// }
