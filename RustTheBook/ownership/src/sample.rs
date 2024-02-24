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

#[allow(dead_code)]
pub fn sample7() {
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1はここでスコープを抜ける

    let _r2 = &mut s;
}

#[allow(dead_code)]
pub fn sample8() {
    let mut s = String::from("hello");

    //不変な参照と可変な参照を同時に持つことはできない
    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; //問題あり

    println!("{}, {}", r1, r2);
}

// #[allow(dead_code)]
// pub fn sample9() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s //sの参照を返すが，sはスコープを抜けるとdrop関数が呼び出されるため，参照先がなくなる
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
