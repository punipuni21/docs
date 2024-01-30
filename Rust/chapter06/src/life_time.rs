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
