use serde::Desirialize;

#[derive(Debug, Defualt, Desirialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u32,
}
