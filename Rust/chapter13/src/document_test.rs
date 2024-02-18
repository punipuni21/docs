use crate::target::Guest;

/// ```
/// use chapter13::document_test::calc_fee_case_01;
/// let result = calc_fee_case_01();
/// assert_eq!(500 , result);
/// ```

#[allow(dead_code)]
pub fn calc_fee_case_01() -> u32 {
    let guest = Guest::new(10, false);
    let result = guest.clone().calc_fee().unwrap();
    result
}
