// 当结构体的引用穿越函数边界时，我们要格外小心，因为编译器只会对函数签名进行检查，并不关心内部到底用了结构体的哪个字段，当签名都使用了结构体时，会立即报错。
// 而智能指针由于隐式解引用Deref的存在，导致了两次Deref时都让结构体穿越了函数边界Deref::deref，结果造成了重复借用的错误。
// 解决办法就是提前对智能指针进行手动解引用，然后对内部的值进行借用后，再行使用。
#![allow(unused)]
fn main() {
    use std::cell::RefCell;
    use std::io::Write;

    struct Data {
        string: String,
    }

    struct S {
        data: Data,
        writer: Vec<u8>,
    }

    fn write(s: RefCell<S>) {
        let mut mut_s = s.borrow_mut();
        let str = &mut_s.data.string;
        mut_s.writer.write(str.as_bytes());
    }
}