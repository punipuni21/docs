static TAX_RATE: f32 = 0.10;
static mut TOTAL_VALUE: i32 = 0;

#[allow(dead_code)]
pub fn calc_amount(price: i32) -> i32 {
    price + (price as f32 * TAX_RATE) as i32
}

#[allow(dead_code)]
pub fn calc_total(value: i32) {
    unsafe {
        TOTAL_VALUE += value;
        println!("TOTAL_VALUE = {}", TOTAL_VALUE);
    }
}
