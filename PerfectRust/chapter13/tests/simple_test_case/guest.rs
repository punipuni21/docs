#[cfg(test)]
mod simple_test {
    use chapter13::target::{Guest, SampleError};
    use simple_test_case::test_case;

    #[test_case(0, false, 0; "case01 arg:0 campaign:false expected:0")]
    #[test_case(0, true, 0; "case02 arg:0 campaign:true expected:0")]
    #[test_case(12, true, 450; "case08 arg:12 campaign:true expected:450")]
    #[test]
    fn calc_fee_test01_08(age: u32, campaign: bool, expected: u32) {
        let guest = Guest::new(age, campaign);
        assert_eq!(guest.calc_fee().unwrap(), expected);
    }

    #[test_case(121, false; "case09 arg:121 camplain:false")]
    #[test]
    fn calc_fee_test09_10(age: u32, campaign: bool) {
        let expected = SampleError::Msg(String::from("Invalid age"));
        let guest = Guest::new(age, campaign);
        assert_eq!(guest.calc_fee().unwrap_err(), expected);
    }
}
