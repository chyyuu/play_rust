use std::cell::{Cell, RefCell};
fn main() {
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}, {}", one, two,c.get());

    //uncomment below lines, will get the compiling error
    // let d = Cell::new(String::from("asdf"));
    // let t1=d.get();
    // d.set("mytest".to_string());
    // let t2=d.get();
    // println!("{},{}", t1,t2);

    let s = RefCell::new(String::from("hello, world"));
    //let s1 = s.borrow();
    let s2 = s.borrow_mut();

    //println!("{},{}", s1, s2);
    println!("{}", s2);
}
