use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // a : 5, nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a ref count= {}", Rc::strong_count(&a));
    println!("a point-to = {:?}", a.tail());

    // b: 10, a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("after create b ，a ref count = {}", Rc::strong_count(&a));
    println!("b ref count  = {}", Rc::strong_count(&b));
    println!("b point-to = {:?}", b.tail());

    // a --> b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    // now   a->[5, b]  b->[10, a] a and b are cycle
    println!("after change a, b ref count = {}", Rc::strong_count(&b));
    println!("after change a，a ref count = {}", Rc::strong_count(&a));

    // uncomment the next line to see that we have a cycle and stack overflow
    //println!("a next item = {:?}", a.tail());
}
