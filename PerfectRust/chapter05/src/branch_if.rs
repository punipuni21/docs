#[allow(dead_code)]
pub fn branch_if_let() {
    let num = 10;
    let result = if num == 1 {
        "num is 1"
    } else if num == 2 {
        "num is 2"
    } else {
        "num is not 1 or 2"
    };
    println!("result: {}", result);
}
