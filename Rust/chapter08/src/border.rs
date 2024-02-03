use std::marker::Copy;
use std::ops::Add;
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Customer<T> {
    id: T,
    name: String,
    address: String,
    email: String,
}

impl<T> Customer<T>
where
    T: Add + Copy,
{
    #[allow(dead_code)]
    fn new(id: T, name: String, address: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }

    #[allow(dead_code)]
    fn change_id(&mut self, value: T) {
        self.id = value;
    }
}

// #[allow(dead_code)]
// pub fn use_new() {
//     let customer = Customer::<String>::new(
//         100,
//         String::from("John"),
//         String::from("123 Main St"),
//         String::from("hoge@sample.com"),
//     ); // compile error
//     println!("{:?}", customer);
// }
