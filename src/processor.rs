#![allow(dead_code)]

const RAM: usize = 4096;
const VRAM: usize = 2048;

pub struct Processor {
    v: [u8; 16],
    idxr: u16,
    pc: usize,

    ram: [u8; RAM],
    vram: [bool; VRAM],

    stack: [usize; 16],
    sp: usize,

    keys: [bool; 16],

    delay_timer: u8,
    sound_timer: u8,
}

impl Processor {
    pub fn initialize() -> Processor {
        Processor {
            v: [0; 16],
            idxr: 0,
            pc: 0x200,
            ram: [0; RAM],
            vram: [false; VRAM],
            stack: [0; 16],
            sp: 0,
            keys: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
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

    fn execute_opcode(&mut self, opcode: u16, nibbles: (u8, usize, usize, u8)) {
        let (op_major, x, y, op_minor) = nibbles;

        match op_major {
            0x00 => match op_minor {
                0x00 => self.op_00e0(),
                0x0e => self.op_00ee(),
                _ => unreachable!(),
            },
            0x01 => self.op_1nnn(opcode),
            0x02 => self.op_2nnn(opcode),
            0x03 => self.op_3xnn(x, opcode),
            0x04 => self.op_4xnn(x, opcode),
            0x05 => self.op_5xy0(x, y),
            0x06 => self.op_6xnn(x, opcode),
            0x07 => self.op_7xnn(x, opcode),
            0x08 => match op_minor {
                0x00 => self.op_8xy0(x, y),
                0x01 => self.op_8xy1(x, y),
                0x02 => self.op_8xy2(x, y),
                0x03 => self.op_8xy3(x, y),
                0x04 => self.op_8xy4(x, y),
                0x05 => self.op_8xy5(x, y),
                0x06 => self.op_8xy6(x),
                0x07 => self.op_8xy7(x, y),
                0x0e => self.op_8xye(x),
                _ => unreachable!(),
            },
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

    // Skips the next instruction if VX equals NN
    fn op_3xnn(&mut self, x: usize, opcode: u16) {
        let nn = (opcode & 0x00ff) as u8;
        match self.v[x] == nn {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    // Skips the next instruction if VX doesn't equal NN
    fn op_4xnn(&mut self, x: usize, opcode: u16) {
        let nn = (opcode & 0x00ff) as u8;
        match self.v[x] == nn {
            true => self.pc += 2,
            false => self.pc += 4,
        }
    }

    // Skips the next instruction if VX equals VY
    fn op_5xy0(&mut self, x: usize, y: usize) {
        match self.v[x] == self.v[y] {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    // Sets VX to NN
    fn op_6xnn(&mut self, x: usize, opcode: u16) {
        let nn = (opcode & 0x00ff) as u8;
        self.v[x] = nn;
        self.pc += 2;
    }

    // Adds NN to VX
    fn op_7xnn(&mut self, x: usize, opcode: u16) {
        let nn = (opcode & 0x00ff) as u8;
        self.v[x] += nn;
        self.pc += 2;
    }

    // Sets VX to the value of VY
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += 2;
    }

    // Sets VX to VX or VY
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.pc += 2;
    }

    // Sets VX to VX and VY
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.pc += 2;
    }

    // Sets VX to VX xor VY
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.pc += 2;
    }

    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let val = vx + vy;

        self.v[x] = val as u8;
        self.v[0x0f] = match val {
            0x00..=0xff => 0,
            _ => 1, // Set carry flag
        };
        self.pc += 2;
    }

    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0f] = match self.v[x] > self.v[y] {
            true => 0,
            false => 1,
        };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += 2;
    }

    // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
    fn op_8xy6(&mut self, x: usize) {
        self.v[0x0f] = self.v[x] & 0x01;
        self.v[x] >>= 1;
        self.pc += 2;
    }

    // Sets VX to VY minus VX
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0f] = match self.v[x] <= self.v[y] {
            true => 0,
            false => 1,
        };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += 2;
    }

    // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
    fn op_8xye(&mut self, x: usize) {
        self.v[0x0f] = self.v[x] & 0b10000000;
        self.v[0x0f] >>= 7;
        self.v[x] <<= 1;
        self.pc += 2;
    }

    // Skips the next instruction if VX doesn't equal VY
    fn op_9xy0(&mut self, x: usize, y: usize) {
        match self.v[x] != self.v[y] {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    // Set idxr to address `nnn`
    fn op_annn(&mut self, opcode: u16) {
        self.idxr = opcode & 0x0fff;
        self.pc += 2;
    }

    // Jumps to the address NNN plus V0
    fn op_bnnn(&mut self, opcode: u16) {
        let mut nnn = opcode & 0x0fff;
        nnn += self.v[0x00] as u16;
        self.pc = nnn as usize;
    }

    // Sets VX to the result of a bitwise and operation on a random number and NN
    fn op_cxnn(&mut self, x: usize) {
        // TODO
    }

    // Draw sprite
    fn op_dxyn(&mut self, x: usize, y: usize) {
        // TODO
    }

    // Skips the next instruction if the key stored in VX is pressed
    fn op_ex9e(&mut self, x: usize) {
        // TODO
    }

    // Skips the next instruction if the key stored in VX isn't pressed
    fn op_exa1(&mut self, x: usize) {
        // TODO
    }

    // Sets VX to the value of the delay timer
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += 2;
    }

    // A key press is awaited, and then stored in VX
    fn op_fx0a(&mut self, x: usize) {
        // TODO
    }

    // Sets the delay timer to VX
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc += 2;
    }

    // Sets the sound timer to VX
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc += 2;
    }

    // Adds VX to I
    fn op_fx1e(&mut self, x: usize) {
        self.idxr += self.v[x] as u16;
        match self.idxr {
            0x000..=0xfff => self.v[0x0f] = 0,
            _ => self.v[0x0f] = 1,
        }
        self.pc += 2;
    }

    // Sets I to the location of the sprite for the character in VX
    fn op_fx29(&mut self, x: usize) {
        // TODO
    }

    // Stores the binary-coded decimal representation of VX, with the most
    // significant of three digits at the address in I, the middle digit at I
    // plus 1, and the least significant digit at I plus 2
    fn op_fx33(&mut self, x: usize) {
        // TODO
    }

    // Stores V0 to VX (including VX) in memory starting at address I
    fn op_fx55(&mut self, x: usize) {
        // TODO
    }

    // Fills V0 to VX (including VX) with values from memory starting at address I
    fn op_fx65(&mut self, x: usize) {
        // TODO
    }
}

// An opcode is two bytes long (four nibbles).
//
// /------- byte 1 -------\  /------- byte 2 -------\
// /----n1----||----n2----\  /----n3----||----n4----\
// / op_major ||     x    \  /    y     || op_minor \
fn decode_opcode(opcode: u16) -> (u8, usize, usize, u8) {
    let op_major = ((opcode & 0xf000) >> 12) as u8;
    let x = ((opcode & 0x0f00) >> 8) as usize;
    let y = ((opcode & 0x00f0) >> 4) as usize;
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
