use std::collections::HashMap;

struct Cpu {
    pc: u16,
    cycles: u32,
    opcodes: HashMap<u8, Opcode>,
}

#[derive(Copy,Clone)]
struct Opcode {
    size: u16,
    cycles: u32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            cycles: 0,
            opcodes: HashMap::from([
                (0x00, Opcode::new(1, 7)),
                (0x01, Opcode::new(2, 6))
            ]),
        }
    }

    fn tick(&mut self) {
        let address = self.pc as u8;
        //let opcode = &self.opcodes[&address];
        let opcode = self.opcodes[&address];

        self.step(&opcode);
    }
    fn step(&mut self, opcode: &Opcode) {

    }

}



impl Opcode {
    fn new(size: u16, cycles: u32) -> Opcode {
        Opcode { size, cycles }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    cpu.tick();
}
