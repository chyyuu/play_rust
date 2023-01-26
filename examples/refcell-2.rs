#![allow(unused)]
//error demo
// fn main() {
//     pub trait Messenger {
//         fn send(&self, msg: String);
//     }

//     // --------------------------
//     struct MsgQueue {
//         msg_cache: Vec<String>,
//     }

//     impl Messenger for MsgQueue {
//         fn send(&self, msg: String) {
//             self.msg_cache.push(msg)
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;
pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn main() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());
    
    let s = Rc::new(RefCell::new("mymultitest".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", on yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
