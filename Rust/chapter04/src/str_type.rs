#[allow(dead_code)]
pub fn declar() {
    let str = "Hello Rust";
    println!("value = {:?}, ptr = {:p}, len = {}", str, str, str.len()); //slice is created at static memory
}

#[allow(dead_code)]
pub fn invalid_str_binding() {
    let str_a = "Hello ";
    let str_b = "Rust.";
    println!(
        "str_a:value = {:?}, ptr = {:p}, len = {}",
        str_a,
        str_a,
        str_a.len()
    );
    println!(
        "str_b:value = {:?}, ptr = {:p}, len = {}",
        str_b,
        str_b,
        str_b.len()
    );
    let str_c = str_a + str_b; // ref binding is not allowed
    println!(
        "str_c:value = {:?}, ptr = {:p}, len = {}",
        str_c,
        str_c,
        str_c.len()
    );
}
