#[allow(dead_code)]
pub fn instantiate() {
    let s1 = String::new(); // new a empty string
    println!(
        "new()={:?} len={}, capacity={}",
        &s1,
        &s1.len(),
        &s1.capacity()
    );
    let s2 = String::from("Hello Rust."); // create instance from designated string
    println!(
        "from()={:?} len={}, capacity={}",
        &s2,
        &s2.len(),
        &s2.capacity()
    );
    let s3 = String::with_capacity(5); // create instance of capacity 5
    println!(
        "with_capacity()={:?} len={}, capacity={}",
        &s3,
        &s3.len(),
        &s3.capacity()
    );
}

#[allow(dead_code)]
pub fn add() {
    let mut s = String::from("abc");
    s.push('d');
    println!("push()={:?}", &s);
    s.insert(1, 'd'); //FIXME search insert implementation
    println!("insert()={:?}", &s);
    let mut s = String::from("abc");
    s.push_str("d");
    println!("push_str()={:?}", &s);
    s.insert_str(1, "d");
    println!("insert_str()={:?}", &s);
}
