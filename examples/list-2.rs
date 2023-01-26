use std::rc::Rc;
fn main() {
    // 创建Rc，持有一个值5
    let five = Rc::new(5);
    println!("five ref count = {}", Rc::strong_count(&five));
    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);
    println!("five weak ref count = {}", Rc::weak_count(&five));
    // Weak引用的资源依然存在，取到值5
    let strong_five = weak_five.upgrade().unwrap();
    assert_eq!(*strong_five, 5);
    println!("strong_five ref count = {}", Rc::strong_count(&strong_five));
    println!("strong_five weak ref count = {}", Rc::weak_count(&strong_five));

    // 手动释放资源`five`
    println!("drop five");
    drop(five);
    println!("strong_five ref count = {}", Rc::strong_count(&strong_five));
    println!("strong_five weak ref count = {}", Rc::weak_count(&strong_five));
    println!("drop strong_five");
    drop(strong_five);

    // Weak引用的资源已不存在，因此返回None
    let next_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(next_five, None);
}
