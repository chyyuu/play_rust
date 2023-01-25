//https://course.rs/advance/lifetime/advance.html#%E4%B8%80%E4%B8%AA%E5%A4%8D%E6%9D%82%E7%9A%84%E4%BE%8B%E5%AD%90
struct Interface<'a> {
    manager: &'a Manager<'a>,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a self) -> Interface<'a> {
        Interface {
            manager: &self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };
    {
        list.get_interface().noop();
        //drop(list);
    }
    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
