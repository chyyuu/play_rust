fn bar(writer: &mut Writer) {
    baz(&mut writer.indent());
    writer.write("world");
    writer.indent().indent().write("chyyuu");
}

fn baz(writer: &mut Writer) {
    writer.write("hello");
}

pub struct Writer<'a> {
    target: &'a mut String,
    indent: usize,
}

impl<'a> Writer<'a> {
    //fn indent(&'b mut self) -> Writer<'b> {
    fn indent(&mut self) -> Writer{    
        Writer {
            target: self.target,
            indent: self.indent + 1,
        }
    }

    fn write(&mut self, s: &str) {
        for _ in 0..self.indent {
            self.target.push(' ');
        }
        self.target.push_str(s);
        self.target.push('\n');
    }
}

fn main() {
    let mut s = String::new();
    let mut writer = Writer {
        target: &mut s,
        indent: 2,
    };
    bar(&mut writer);
    println!("{}", s);
}
