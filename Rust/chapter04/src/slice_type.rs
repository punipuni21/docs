use std::ops::Range;

#[allow(dead_code)]
pub fn get() {
    let str_array: [&str; 5] = ["one", "two", "three", "four", "five"];
    let slice1: &[&str] = &str_array[3..=4];
    let slice2: &[&str] = &str_array[1..5];
    let slice3: &[&str] = &str_array[..];
    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
    println!("slice3: {:?}", slice3);
}

#[allow(dead_code)]
pub fn range() {
    let int_array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let range: Range<usize> = std::ops::Range { start: 1, end: 3 };
    println!("range: {:?}", range);
    assert_eq!((1..3), range); //range output type is Range<usize>, which is expressed as tuple (start, end)
    let slice: &[i32] = &int_array[range];
    println!("slice: {:?}", slice);
}

#[allow(dead_code)]
pub fn invalid_multibyte_slice() {
    let company_name = "株式会社Rust"; //multibyte string use 3 bytes per character
    let slice = &company_name[1..3]; //invalid slice
    println!("ref range: {:?}, size={}", slice, slice.len()); //this program will panic
}

#[allow(dead_code)]
pub fn valid_multibyte_slice() {
    let company_name = "株式会社Rust";
    let slice = &company_name[..12];
    println!("ref range: {:?}, size={}", slice, slice.len());
    let slice = &company_name[12..];
    println!("ref range: {:?}, size={}", slice, slice.len());
}
