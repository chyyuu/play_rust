#![allow(unused)]
fn main() {
    struct Test {
        a: u32,
        b: u32,
    }

    impl Test {
        fn increase_a(&mut self) {
            self.a += 1;
        }

        fn increase(&mut self) {
            let b = &mut self.b;
            self.increase_a();
            *b += 1;
        }
    }
}
