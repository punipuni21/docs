use restaurant::eat_at_restaurant;
use std::collections::HashMap; //他のクレートの公開されているものを使う場合はフルパスで記載する
fn main() {
    eat_at_restaurant();

    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::io::{self, Write};
//上と下は同じ
// use std::io;
// use std::io::Write;
