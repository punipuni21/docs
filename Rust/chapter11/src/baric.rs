#[derive(Debug, PartialEq, Eq)]
pub enum ValueConversion {
    Int,
    Float,
}

#[allow(dead_code)]
fn parse_01(value: String, conv: ValueConversion) {
    if conv == ValueConversion::Int {
        println!("{:?}", value.parse::<i32>());
    } else {
        println!("{:?}", value.parse::<f32>());
    }
}

#[allow(dead_code)]
pub fn use_parse_01() {
    parse_01(String::from("123"), ValueConversion::Int);
    parse_01(String::from("123"), ValueConversion::Float);
    parse_01(String::from("ABC"), ValueConversion::Int);
    parse_01(String::from("ABC"), ValueConversion::Float);
}
