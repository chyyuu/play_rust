#![feature(core_intrinsics)]
#[warn(unused_variables)]

fn main() {
    let r = &1;
    //print_type_name_of(r);
    let &_a = r;
    let ref _b =1;
    let _a = *r;
    let x = &false;
    print_type_name_of(x);

    let &x = &false;
    print_type_name_of(x);

    let ref x = &false;
    print_type_name_of(x);
}
fn print_type_name_of<T>(_: T) {
    println!("{}", { std::intrinsics::type_name::<T>() })
}