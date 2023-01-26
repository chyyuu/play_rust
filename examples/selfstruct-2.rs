
#![allow(unused)]
struct SelfRef<'a> {
    value: String,

    // 该引用指向上面的value
    pointer_to_value: &'a str,
}
fn main(){
    let s = "aaa".to_string();
    let v = SelfRef {
        value: s.clone(),
        pointer_to_value: &s
    };
}

