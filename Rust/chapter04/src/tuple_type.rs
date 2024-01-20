#[allow(dead_code)]
pub fn dealare() {
    let x: (i32, &str, bool) = (1, "hello", true);

    let (a, b, c) = (1, "hello", true);

    let (l, _, m) = x;

    println!("{:?}", x);
    println!("a={:?}, b={:?}, c={:?}", a, b, c);
    println!("l={:?}, m={:?}", l, m);
}
