#![allow(unused)]
fn main() {
    struct Test {
        a: u32,
        b: u32,
    }

    impl Test {
        fn increase_a(a:&mut u32) {
            *a += 1;
        }

        fn increase(&mut self) {
            let b = &mut self.b;
            Test::increase_a(&mut self.a);
            *b += 1;
        }
    }
}
