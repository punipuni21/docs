use crate::generics::entities::Product;
use crate::generics::service::ReadService;
use crate::generics::traits::{CsvReader, JsonReader};
use crate::generics::traits_impl::{CsvReaderImpl, JsonReaderImpl};

#[allow(dead_code)]
pub fn use_generics_method() {
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.csv");
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.json");

    let csv_method = CsvReaderImpl::<Product>::default();
    let json_method = JsonReaderImpl::<Product>::default();

    let csv_result = csv_method.read(csv_path).unwrap();
    let json_result = json_method.read(json_path).unwrap();

    println!("<<CSV>>");
    for result in csv_result {
        println!("{:?}", result);
    }
    println!("<<JSON>>");
    for result in json_result {
        println!("{:?}", result);
    }
}

#[allow(dead_code)]
pub fn use_service_method() {
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.csv");
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.json");

    let service = ReadService::<Product>::new();

    let csv_result = service.csv_reader(csv_path).unwrap();
    let json_result = service.json_reader(json_path).unwrap();

    println!("<<CSV>>");
    for result in csv_result {
        println!("{:?}", result);
    }
    println!("<<JSON>>");
    for result in json_result {
        println!("{:?}", result);
    }
}
