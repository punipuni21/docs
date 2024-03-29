#[allow(dead_code)]
#[derive(Debug, Clone)]
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

    #[allow(dead_code)]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    #[allow(dead_code)]
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Drop for Customer {
    fn drop(&mut self) {
        println!("drop instance {}", self.name)
    }
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            address: String::from(""),
            email: String::from(""),
        }
    }
}

//TODO TryFrom trait
//TODO From trait

#[allow(dead_code)]
pub fn use_constant() {
    println!("ID_MIN:{}", Customer::ID_MIN);
    println!("ID_MAX:{}", Customer::ID_MAX);
}

#[allow(dead_code)]
pub fn use_method() {
    let mut customer = Customer::new(
        100,
        String::from("John"),
        String::from("123 Main St"),
        String::from("hoge@sample.com"),
    );
    customer.set_name(String::from("Smith"));
    println!("name:{}", customer.get_name());
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

#[allow(dead_code)]
pub fn use_clone() {
    let customer = Customer::new(
        100,
        String::from("John"),
        String::from("123 Main St"),
        String::from("hoge@sample.com"),
    );
    println!("clone of customer: {:?}", customer.clone());
}

#[allow(dead_code)]
pub fn use_drop() {
    let customer_1 = Customer::new(
        100,
        String::from("John"),
        String::from("123 Main St"),
        String::from("hoge@sample.com"),
    );
    let mut costomer_2 = customer_1.clone();
    costomer_2.set_name(String::from("Smith"));
}

#[allow(dead_code)]
pub fn use_default() {
    let customer = Customer::default();
    println!("customer: {:?}", customer);
}
