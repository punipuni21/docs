use anyhow::Result;

pub trait Calculator {
    fn calc(&self) -> Result<u64>;
}
