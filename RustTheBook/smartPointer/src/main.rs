enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

#[allow(unused_variables)]
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //Boxは間接参照とヒープメモリ確保を提供する

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
