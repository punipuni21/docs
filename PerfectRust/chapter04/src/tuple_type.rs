#[allow(dead_code)]
pub fn dealare() {
    let x: (i32, &str, bool) = (1, "hello", true);

    let (a, b, c) = (1, "hello", true);

    let (l, _, m) = x;

    println!("{:?}", x);
    println!("a={:?}, b={:?}, c={:?}", a, b, c);
    println!("l={:?}, m={:?}", l, m);
}

#[allow(dead_code)]
pub fn calc(value: (i32, i32)) {
    println!("{} + {} = {}", value.0, value.1, value.0 + value.1);
    println!("{} - {} = {}", value.0, value.1, value.0 - value.1);
}

#[allow(dead_code)]
pub fn methods() {
    let a: (i32, i32, i32) = (1, 2, 3);
    println!("clone():{:?}", a.clone());
    println!("eq():{}", a.eq(&(1, 2, 3)));
    println!("cmp():{:?}", a.cmp(&(2, 2, 3)));
}
