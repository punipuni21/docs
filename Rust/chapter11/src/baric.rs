use std::env::current_dir;
use std::fs::File;
use std::io::ErrorKind;
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

#[allow(dead_code)]
pub fn use_error_kind() {
    let file_path = current_dir()
        .map(|path_buf| path_buf.join("\\files\\sample.txt"))
        .map_err(|error| panic!("{}", error))
        .unwrap();

    let error = File::open(file_path).err().unwrap();

    let result = match error.kind() {
        ErrorKind::NotFound => "File not found",
        ErrorKind::PermissionDenied => "Permission denied",
        _ => "Other error",
    };
    println!("{}", result);
}
