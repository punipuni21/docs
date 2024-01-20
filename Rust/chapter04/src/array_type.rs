#[allow(dead_code)]
pub fn multidimensional() {
    let array_a = [[0; 5]; 3];
    for sub_array in array_a {
        println!("{:?}", sub_array);
    }

    let array_b = [[[10; 5]; 3]; 2];
    for sub_array in array_b {
        println!("{:?}", sub_array);
    }
}
