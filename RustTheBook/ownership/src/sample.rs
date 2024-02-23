#[allow(dead_code)]
pub fn sample1() {
    {
        let s = String::from("hello");
        println!("{:?}", s);
    }
    // println!("{:?}", s); スコープを抜ける時点でdrop関数を呼び出す．OSにメモリを返却している
}

#[allow(dead_code)]
pub fn sample2() {
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);// s1はs1に移動した後無効化されている．スコープを抜けてもdrop関数は呼び出されない
}

#[allow(dead_code)]
pub fn sample3() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

#[allow(dead_code)]
pub fn sample4() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}
