const RAM: usize = 4096;
const VRAM: usize = 2048;

pub struct Processor {
    registers: [u8; 16],
    idxr: u16,
    pc: usize,

    ram: [u8; RAM],
    vram: [bool; VRAM],

    stack: [usize; 16],
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
        let nibbles = decode_opcode(opcode);
        self.execute_opcode(opcode, nibbles);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let byte1 = self.ram[self.pc] as u16;
        let byte2 = self.ram[self.pc + 1] as u16;

        byte1 << 8 | byte2
    }

    fn execute_opcode(&mut self, opcode: u16, nibbles: (u8, u8, u8, u8)) {
        let (op_major, x, y, op_minor) = nibbles;

        match op_major {
            0x00 => match op_minor {
                0x00 => self.op_00e0(),
                0x0e => self.op_00ee(),
                _ => unreachable!(),
            },
            0x01 => self.op_1nnn(opcode),
            0x02 => self.op_2nnn(opcode),
            0x0a => self.op_annn(opcode),
            _ => unimplemented!(),
        }
    }

    // Clear screen
    fn op_00e0(&mut self) {
        // TODO
    }

    // Return
    fn op_00ee(&mut self) {
        // TODO
    }

    // Jump to address at `nnn`
    fn op_1nnn(&mut self, opcode: u16) {
        self.pc = (opcode & 0x0fff) as usize;
    }

    // Call subroutine at `nnn`
    fn op_2nnn(&mut self, opcode: u16) {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = (opcode & 0x0fff) as usize;
    }

    // Set idxr to address `nnn`
    fn op_annn(&mut self, opcode: u16) {
        self.idxr = opcode & 0x0fff;
        self.pc += 2;
    }
}

// An opcode is two bytes long (four nibbles).
//
// /------- byte 1 -------\  /------- byte 2 -------\
// /----n1----||----n2----\  /----n3----||----n4----\
// / op_major ||     x    \  /    y     || op_minor \
fn decode_opcode(opcode: u16) -> (u8, u8, u8, u8) {
    let op_major = ((opcode & 0xf000) >> 12) as u8;
    let x = ((opcode & 0x0f00) >> 8) as u8;
    let y = ((opcode & 0x00f0) >> 4) as u8;
    let op_minor = (opcode & 0x000f) as u8;

    (op_major, x, y, op_minor)
}

#[cfg(test)]
mod tests {
    use crate::processor::Processor;

    #[test]
    fn op_1nnn() {
        let mut cpu = Processor::initialize();
        cpu.ram[0x200] = 0x1a;
        cpu.ram[0x201] = 0xaa;

        cpu.run_cycle();
        assert_eq!(cpu.pc, 0xaaa);
    }

    #[test]
    fn op_2nnn() {
        let mut cpu = Processor::initialize();
        cpu.ram[0x200] = 0x25;
        cpu.ram[0x201] = 0x55;

        cpu.run_cycle();
        assert_eq!(cpu.pc, 0x555);
        assert_eq!(cpu.stack[0], 0x202);
    }

    #[test]
    fn op_annn() {
        let mut cpu = Processor::initialize();
        cpu.ram[0x200] = 0xa1;
        cpu.ram[0x201] = 0x23;

        cpu.run_cycle();
        assert_eq!(cpu.idxr, 0x123);
    }
}
