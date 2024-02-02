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
    const ID_MIN: u32 = 1;
    const ID_MAX: u32 = 10000;

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
pub fn use_constant() {
    println!("ID_MIN:{}", Customer::ID_MIN);
    println!("ID_MAX:{}", Customer::ID_MAX);
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
