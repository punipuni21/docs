#[derive(Debug, Clone)]
Customer<T> {
  id: T,
  name: String,
  address: String,
  email: String
}

impl<T> Customer<T>{
  #[allow(dead_code)]
  fn new(id: T, name: String, address: String, email: String)-> Self{
    Self{
      id,
      name,
      address,
      email
    }
  }
}