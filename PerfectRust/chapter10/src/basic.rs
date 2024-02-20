use anyhow::Result;

pub struct Rectangle {
    width: u64,
    height: u64,
}

pub trait Calculator {
    fn calc(&self) -> Result<u64>;
}

impl Calculator for Rectangle {
    fn calc(&self) -> Result<u64> {
        Ok(self.width * self.height)
    }
}

#[allow(dead_code)]
pub fn use_rectangle() {
    let r = Rectangle {
        width: 10,
        height: 20,
    };
    let result = r.calc();
    println!("area = {}", result.unwrap());
}
