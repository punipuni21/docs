#[allow(dead_code)]
pub fn declare() {
    let x: i32 = 10;
    let y: &str = "ABC";
    let x_ptr: *const i32 = &x;
    let y_ptr: *const &str = &y;

    //display value
    unsafe {
        println!("x_ptr: {}", *x_ptr);
        println!("y_ptr: {}", *y_ptr);
    }

    //address
    println!("x_ptr: {:p}", x_ptr);
    println!("y_ptr: {:p}", y_ptr);
}
