mod static_val;
mod variable;

fn main() {
    // variable::shadowing();
    // println!("calc_amount(100) = {}", static_val::calc_amount(100));
    static_val::calc_total(100);
}
