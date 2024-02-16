use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum SampleError {
    Msg(String),
}
impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::Msg(msg) => write!(f, "{}", msg),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Guest {
    age: u32,
    campaign: bool,
}
impl Display for Guest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "age:{} campaign:{}", self.age, self.campaign)
    }
}
impl Guest {
    pub fn new(_age: u32, _campaign: bool) -> Self {
        Self
    }

    pub fn calc_fee(self) -> Result<u32, SampleResult> {
        let fee = match self.age {
            0..4 => 0,
            5..12 => 500,
            13..17 => 700,
            18..64 => 1000,
            65..120 => 600,
            _ => return Err(SampleError::Msg("Invalid age".to_string())),
        };
        Ok(fee)
    }

    fn calc_campaign_fee(&self, mut fee: u32) -> u32 {
        if self.campaign && fee != 0 {
            fee = fee * 90 / 100;
        }
        fee
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
