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

#[allow(dead_code)]
pub fn move_2() {
    let mut values = vec![1, 2, 3, 4, 5];
    let sum = move || {
        //ownership is moved when 'move' keyword is used
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    println!("sum: {}", sum());
    println!("values: {:?}", values);
}
