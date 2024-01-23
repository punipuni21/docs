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

#[allow(dead_code)]
pub fn mut_declare() {
    let mut x: i32 = 10;
    let mut y: &str = "ABC";
    let x_ptr: *mut i32 = &mut x;
    let y_ptr: *mut &str = &mut y;

    unsafe {
        println!("x_ptr: {}", *x_ptr);
        println!("y_ptr: {}", *y_ptr);

        *x_ptr += 100;
        let str_val = "usage of pointer".to_string();
        *y_ptr = &str_val;

        println!("x_ptr: {}", *x_ptr);
        println!("y_ptr: {}", *y_ptr);
    }
}
