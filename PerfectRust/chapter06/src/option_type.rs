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

#[allow(dead_code)]
pub fn method_4() {
    let x = 10;
    let y = 5;
    let r = match div(x, y).ok_or("cannot divide".to_owned()) {
        Ok(result) => format!("{} / {} = {}", x, y, result),
        Err(err) => err,
    };
    println!("ok_or: {}", r);
    let x = 10;
    let y = 0;
    let r = match div(x, y).ok_or_else(|| "cannot divide".to_owned()) {
        Ok(result) => format!("{} / {} = {}", x, y, result),
        Err(err) => err,
    };
    println!("ok_or_else: {}", r);
}

#[allow(dead_code)]
pub fn method_5() -> Option<String> {
    let x = 10;
    let y = 0;
    let result: i32 = div(x, y)?; //? means if None, return None
    Some(format!("{} / {} = {}", x, y, result))
}
