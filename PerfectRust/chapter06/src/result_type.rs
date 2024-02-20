#[allow(dead_code)]
pub fn value_setting() {
    let mut x: Result<i32, String>;
    x = Ok(10);
    println!("x= {:?}", x);
    x = Err(String::from("error"));
    println!("x= {:?}", x);
}
