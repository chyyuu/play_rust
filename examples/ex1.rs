use std::rc::Rc;
use std::cell::Cell;

fn x() {
    println!("x");
}

fn f2(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x(); // might happen
    }
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x(); // never happens
    }
}

fn main() {
    println!("Hello，world");
    let a = Rc::new([1, 2, 3]);
    let  b = [1,2,3];
    let mut c = [1,2,3];
    f(& b[1],&mut c[2]);
    let d =Cell::new(5);
    let e = Cell::new(6);
    f2(&d,&d);
    println!("a addr = {:?} {}",a.as_ptr(),a[1]);
    //assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
}
