mod rc_pointer;
use crate::rc_pointer::List::{Cons, Nil};
use std::rc::Rc;

#[allow(unused_variables)]
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
