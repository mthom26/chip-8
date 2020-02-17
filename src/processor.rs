const RAM: usize = 4096;
const VRAM: usize = 2048;

pub struct Processor {
    registers: [u8; 16],
    idxr: usize,
    pc: usize,

    ram: [u8; RAM],
    vram: [bool; VRAM],

    stack: [u16; 16],
    sp: usize,

    keys: [bool; 16],
}

impl Processor {
    pub fn initialize() -> Processor {
        Processor {
            registers: [0; 16],
            idxr: 0,
            pc: 0x200,
            ram: [0; RAM],
            vram: [false; VRAM],
            stack: [0; 16],
            sp: 0,
            keys: [false; 16],
        }
    }

    pub fn run_cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let byte1 = self.ram[self.pc] as u16;
        let byte2 = self.ram[self.pc + 1] as u16;

        let opcode = byte1 << 8 | byte2;
        self.pc += 2;

        opcode
    }

    fn execute_opcode(&mut self, opcode: u16) {
        match opcode {
            0xa000..=0xafff => {
                // Set idxr to address
                let i = (opcode & 0x0fff) as usize;
                self.idxr = i;
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::Processor;

    #[test]
    fn set_idxr() {
        let mut cpu = Processor::initialize();
        cpu.ram[0x200] = 0xa1;
        cpu.ram[0x201] = 0x23;

        cpu.run_cycle();
        assert_eq!(cpu.idxr, 0x123);
    }
}
