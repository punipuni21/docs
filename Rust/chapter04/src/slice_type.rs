#[allow(dead_code)]
pub fn get() {
    let str_array: [&str; 5] = ["one", "two", "three", "four", "five"];
    let slice1: &[&str] = &str_array[3..=4];
    let slice2: &[&str] = &str_array[1..5];
    let slice3: &[&str] = &str_array[..];
    println!("slice1:{:?}", slice1);
    println!("slice2:{:?}", slice2);
    println!("slice3:{:?}", slice3);
}
