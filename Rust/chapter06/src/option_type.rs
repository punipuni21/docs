#[allow(dead_code)]
fn div(value1: i32, value2: i32) -> Option<i32> {
    if value2 == 0 {
        return None;
    }
    let r = (value1 / value2) as i32;
    Some(r)
}

#[allow(dead_code)]
pub fn use_div() {
    let x = 10;
    let y = 5;
    let r = match div(x, y) {
        None => "cannot divide".to_owned(),
        Some(result) => format!("{} / {} = {}", x, y, result),
    };
    println!("{}", r);
}
