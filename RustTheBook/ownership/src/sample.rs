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
    //関数の引数に参照をとることを借用という
    s.len()
} //sスコープ外になるが，所有権を持っていないためdrop関数は呼び出されない．

#[allow(dead_code)]
pub fn sample5() {
    let mut s = String::from("hello");

    change(&mut s);
}

#[allow(dead_code)]
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//可変な参照は一つしか持てない．この制約により，データ競合を防ぐことができる
// #[allow(dead_code)]
// pub fn sample6() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;
//     //cannot borrow `s` as mutable more than once at a time

//     println!("{}, {}", r1, r2);
// }
