#[allow(dead_code)] //関数が利用されていない場合，ワーニングを出さない
pub fn symbol() {
    let result = 5 + 10;
    println!("5 + 10 = {}", result);
}

#[allow(dead_code)]
pub fn overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}

#[allow(dead_code)]
pub fn ignore_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x.wrapping_add(y);
    println!("{} + {} = {}", x, y, result); //44
}
