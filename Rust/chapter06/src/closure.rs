#[allow(dead_code)]
pub fn closure_sum() {
    let sum = |values: &Vec<i32>| {
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
