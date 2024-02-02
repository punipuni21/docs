#[allow(dead_code)]
#[derive(Debug)]
struct Customer {
    id: i32,
    name: String,
    address: String,
    email: String,
}

#[allow(dead_code)]
pub fn generate_1() {
    let customer = Customer {
        id: 100,
        name: String::from("John"),
        address: String::from("123 Main St"),
        email: String::from("hoge@sample.com"),
    };

    println!("id:{}", customer.id);
    println!("name:{}", customer.name);
    println!("address:{}", customer.address);
    println!("email:{}", customer.email);
}

impl Customer {
    fn new(id: i32, name: String, address: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }
}

#[allow(dead_code)]
pub fn use_debug() {
    let customer = Customer::new(
        100,
        String::from("John"),
        String::from("123 Main St"),
        String::from("hoge@sample.com"),
    );
    println!("customer: {:?}", customer);
}
