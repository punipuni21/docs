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
        Self {
            age: _age,
            campaign: _campaign,
        }
    }

    #[allow(dead_code)]
    pub fn calc_fee(self) -> Result<u32, SampleError> {
        let fee = match self.age {
            0..=4 => 0,
            5..=12 => 500,
            13..=17 => 700,
            18..=64 => 1000,
            65..=120 => 600,
            _ => return Err(SampleError::Msg("Invalid age".to_string())),
        };
        Ok(self.calc_campaign_fee(fee))
    }

    #[allow(dead_code)]
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

    #[ignore = "This test is ignored"]
    #[test]
    fn calc_fee01() {
        let guest = Guest::new(10, false);
        let result = guest.clone().calc_fee().unwrap();
        assert!(result == 500, "{}", &guest);
    }

    #[test]
    fn calc_fee_case_campaign_01() {
        let guest = Guest::new(10, true);
        let result = guest.clone().calc_fee().unwrap();
        assert!(result == 450, "{}", &guest)
    }

    #[test]
    fn calc_fee_case_02() {
        let guest = Guest::new(15, false);
        let result = guest.clone().calc_fee().unwrap();
        assert_eq!(700, result, "{}", &guest)
    }

    #[test]
    fn calc_fee_case_campaign_02() {
        let guest = Guest::new(15, true);
        let result = guest.clone().calc_fee().unwrap();
        assert_eq!(630, result, "{}", &guest)
    }

    #[test]
    fn calc_fee_case_03() {
        let guest = Guest::new(18, false);
        let result = guest.clone().calc_fee().unwrap();
        assert_ne!(700, result, "{}", &guest)
    }

    #[test]
    fn calc_fee_case_campaign_03() {
        let guest = Guest::new(15, true);
        let result = guest.clone().calc_fee().unwrap();
        assert_eq!(630, result, "{}", &guest)
    }

    #[test]
    fn calc_fee_case_wrong_age() {
        let guest = Guest::new(125, false);
        let result = guest.clone().calc_fee().unwrap_err();
        let expected_err = SampleError::Msg("Invalid age".to_string());
        assert_eq!(expected_err, result, "{}", &guest)
    }

    #[test]
    fn calc_campaign_fee_case_01() {
        let guest = Guest::new(0, true);
        let result = guest.calc_campaign_fee(1000);
        assert_eq!(900, result, "{}", &guest)
    }
}
