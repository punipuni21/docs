use std::env::current_dir;
use std::fs::File;

#[allow(dead_code)]
pub fn use_expect() {
    let file_path = current_dir()
        .map(|path_buf| path_buf.join("\\files\\param.txt"))
        .map_err(|error| panic!("Failed to get current directory: {}", error))
        .unwrap();
    let file = File::open(file_path);
    if file.is_err() {
        panic!("{},{:?}", "Failed to open file", file.err().unwrap());
    }
}
