#![feature(generators, generator_trait)]

use std::pin::Pin;
use std::ops::{Generator, GeneratorState};


fn main() {
    let mut gen = || {
        for i in 0..10 {
            yield i;
        }
        
        return ();
    };
    
    loop {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {}", y),
            GeneratorState::Complete(r) => {
                println!("Complete: {:?}", r);
                break;
            }
        }
    }
}