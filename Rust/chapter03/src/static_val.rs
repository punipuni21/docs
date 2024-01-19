static TAX_RATE: f32 = 0.10;

#[allow(dead_code)]
pub fn calc_amount(price: i32) -> i32 {
    price + (price as f32 * TAX_RATE) as i32
}
