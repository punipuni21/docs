#[allow(dead_code)]
pub fn closure_sum() {
    let sum = |values: &Vec<i32>| -> i32 {
        //can declare argument in ||
        //can implement process in {}
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    let values = vec![1, 2, 3, 4, 5];
    println!("sum: {}", sum(&values));
}

#[allow(dead_code)]
pub fn move_1() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = || {
        //closure can use values in outer scope
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    println!("sum: {}", sum());
    println!("values: {:?}", values);
}

// #[allow(dead_code)]
// pub fn move_2() {
//     let mut values = vec![1, 2, 3, 4, 5];
//     let sum = move || {
//         //ownership is moved when 'move' keyword is used
//         let mut sum = 0;
//         for value in values.iter() {
//             sum += value;
//         }
//         sum
//     };
//     println!("sum: {}", sum());
//     println!("values: {:?}", values);
// }

use std::ops::Fn;

#[allow(dead_code)]
pub fn impl_1(values: Vec<i32>) -> impl Fn() -> i32 {
    move || {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    }
}

#[allow(dead_code)]
pub fn where_1<F>(f: F)
where
    F: Fn() -> i32,
{
    let sum = f();
    println!("sum: {}", sum);
}

#[allow(dead_code)]
pub fn use_impl_where_1() {
    let values = vec![1, 2, 3, 4, 5];
    let f = impl_1(values);
    where_1(f);
}

#[allow(dead_code)]
pub fn impl_2() -> impl Fn(Vec<i32>) -> i32 {
    move |values: Vec<i32>| {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    }
}

#[allow(dead_code)]
pub fn where_2<F>(f: F)
where
    F: Fn(Vec<i32>) -> i32,
{
    let values = vec![1, 2, 3, 4, 5];
    let sum = f(values);
    println!("sum: {}", sum);
}

#[allow(dead_code)]
pub fn use_impl_where_2() {
    let f = impl_2();
    where_2(f);
}
