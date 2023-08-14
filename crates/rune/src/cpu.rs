use crate::mmap;
use std::mem;
use std::time::Instant;

pub struct CPU<'prg_rom> {
    /// 0x0 - 0x7ff and mirrors from 0x800 to 0x1fff
    ram: [u8; 2048],
    prg_rom: Option<&'prg_rom Vec<u8>>,
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u8,
    /// accumulator
    a: u8,
    /// register x
    x: u8,
    /// register y
    y: u8,

    // processor status flags
    carry: u8,
    negative: bool,
    overflow: bool,
    decimal: bool,
    interrupt_disable: bool,
    zero: bool,
    /// break command
    b: bool,
}

impl Default for CPU<'_> {
    fn default() -> Self {
        let mut cpu = CPU {
            ram: [0; 2048],
            prg_rom: None,
            pc: 0,
            sp: 0xff,
            a: 0,
            x: 0,
            y: 0,
            negative: false,
            overflow: false,
            decimal: false,
            interrupt_disable: false,
            zero: false,
            carry: 0,
            b: false,
        };

        cpu.set_status(0x34);
        cpu
    }
}

impl CPU<'_> {
    pub fn load_rom(&mut self, prg_rom: Vec<u8>) {
        use mmap::cartrige;
        // loads program
        let mut j = 0;
        for i in cartrige::START..cartrige::END {
            self.ram[i] = prg_rom[j];
            j += 1;
        }

        // loads sprites
    }

    pub fn cycle(&mut self) {
        const SECS_PER_CYCLE: f32 = 1.0 / 21441960.0;
        let start = Instant::now();
        let cycle_time: u8;

        let rom = if let Some(ref rom) = self.prg_rom {
            rom
        } else {
            return;
        };

        let op_u8 = rom[self.pc as usize + 1];
        let op_u16;

        match rom[self.pc as usize] {
            // ADC
            0x69 => {
                self.adc69(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0x65 => {
                self.adc65(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0x75 => {
                self.adc75(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0x6D => {
                self.adc6d(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x7D => {
                self.adc7d(op_u16);
                self.pc += 2;
                cycle_time = 4;
                // TODO: impl cross page time
            }
            0x79 => {
                self.adc79(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x61 => {
                self.adc61(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0x71 => {
                self.adc71(op_u8);
                self.pc += 1;
                cycle_time = 5;
            }

            // AND
            0x29 => {
                self.and29(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0x25 => {
                self.and25(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0x35 => {
                self.and35(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0x2D => {
                self.and2d(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x3D => {
                // TODO: page cross
                self.and3d(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x39 => {
                // TODO: page cross
                self.and39(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x21 => {
                self.and21(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0x31 => {
                // TODO: page cross
                self.and31(op_u8);
                self.pc += 1;
                cycle_time = 5;
            }

            // ASL
            0x0a => {
                self.asl0a();
                cycle_time = 2;
            }
            0x06 => {
                self.asl06(op_u8);
                self.pc += 1;
                cycle_time = 5;
            }
            0x16 => {
                self.asl16(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0x0e => {
                self.asl0e(op_u16);
                self.pc += 2;
                cycle_time = 6;
            }
            0x1e => {
                self.asl1e(op_u16);
                self.pc += 2;
                cycle_time = 7;
            }

            // BCC
            0x90 => {
                // TODO: branch and new page timings
                self.bcc90(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BCS
            0xB0 => {
                // TODO: branch and new page timings
                self.bcsb0(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BEQ
            0xF0 => {
                // TODO: branch and new page timings
                self.beqf0(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BIT
            0x24 => {
                self.bit24(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0x2C => {
                self.bit2c(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }

            // BMI
            0x30 => {
                // TODO: branch and new page timings
                self.bmi30(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BNE
            0xD0 => {
                // TODO: branch and new page timings
                self.bned0(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BPL
            0x10 => {
                // TODO: branch and new page timings
                self.bpl10(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BRK
            0x00 => {
                self.brk00();
                cycle_time = 7;
            }

            // BVC
            0x50 => {
                // TODO: branch and new page timing
                self.bvc50(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // BVS
            0x70 => {
                // TODO: branch and new page timing
                self.bvs70(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // CLC
            0x18 => {
                self.clc18();
                cycle_time = 2;
            }

            // CLD
            0xD8 => {
                self.cldd8();
                cycle_time = 2;
            }

            // CLI
            0x58 => {
                self.cli58();
                cycle_time = 2;
            }

            // CLV
            0xB8 => {
                self.clvb8();
                cycle_time = 2;
            }

            // CMP
            0xC9 => {
                self.cmpc9(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0xC5 => {
                self.cmpc5(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0xD5 => {
                self.cmpd5(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0xCD => {
                self.cmpcd(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0xDD => {
                // TODO: page cross timing
                self.cmpdd(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0xD9 => {
                // TODO: page cross timing
                self.cmpd9(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0xC1 => {
                self.cmpc1(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0xD1 => {
                self.cmpd1(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }

            // CPX
            0xE0 => {
                self.cpxe0(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0xE4 => {
                self.cpxe4(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0xEC => {
                self.cpxec(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }

            // CPY
            0xC0 => {
                self.cpyc0(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0xC4 => {
                self.cpyc4(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0xCC => {
                self.cpycc(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }

            // DEC
            0xC6 => {
                self.decc6(op_u8);
                self.pc += 1;
                cycle_time = 5;
            }
            0xD6 => {
                self.decd6(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0xCE => {
                self.decce(op_u16);
                self.pc += 2;
                cycle_time = 6;
            }
            0xDE => {
                self.decde(op_u16);
                self.pc += 2;
                cycle_time = 7;
            }

            // DEX
            0xCA => {
                self.dexca();
                cycle_time = 2;
            }

            // DEY
            0x88 => {
                self.dey88();
                cycle_time = 2;
            }

            // EOR
            0x49 => {
                self.eor49(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0x45 => {
                self.eor45(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0x55 => {
                self.eor55(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0x4D => {
                self.eor4d(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x5D => {
                // TODO: page cross timing
                self.eor5d(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x59 => {
                // TODO: page cross timing
                self.eor59(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0x41 => {
                self.eor41(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0x51 => {
                // TODO: page cross timing
                self.eor51(op_u8);
                self.pc += 1;
                cycle_time = 5;
            }

            // INC
            0xE6 => {
                self.ince6(op_u8);
                self.pc += 1;
                cycle_time = 5;
            }
            0xF6 => {
                self.incf6(op_u8);
                self.pc += 1;
                cycle_time = 6;
            }
            0xEE => {
                self.incee(op_u16);
                self.pc += 2;
                cycle_time = 6;
            }
            0xFE => {
                self.incfe(op_u16);
                self.pc += 2;
                cycle_time = 7;
            }

            // INX
            0xE8 => {
                self.inxe8();
                cycle_time = 2;
            }

            // INY
            0xC8 => {
                self.inyc8();
                cycle_time = 2;
            }

            // JMP
            0x4C => {
                self.jmp4c(op_u16);
                self.pc += 2;
                cycle_time = 3;
            }
            0x6C => {
                self.jmp6c(op_u16);
                cycle_time = 5;
            }

            // JSR
            0x20 => {
                self.jsr20(op_u16);
                self.pc += 2;
                cycle_time = 6;
            }

            // LDA
            0xA9 => {
                self.ldaa9(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0xA5 => {
                self.ldaa5(op_u8);
                self.pc += 1;
                cycle_time = 3;

            }
            0xB5 => {
                self.ldab5(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0xAD => {
                self.ldaad(op_u16);
                self.pc += 2;
                cycle_time = 4;

            }
            0xBD => {
                // TODO: page cross timing
                self.ldabd(op_u16);
                self.pc += 2;
                cycle_time = 4;

            }
            0xB9 => {
                // TODO: page cross timing
                self.ldab9(op_u16);
                self.pc += 2;
                cycle_time = 4;

            }
            0xA1 => {
                self.ldaa1(op_u8);
                self.pc += 1;
                cycle_time = 6;

            }
            0xB1 => {
                // TODO: page cross timing
                self.ldab1(op_u8);
                self.pc += 1;
                cycle_time = 5;

            }
            
            // LDX
            0xA2 => {
                self.ldxa2(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0xA6 => {
                self.ldxa6(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0xB6 => {
                self.ldxb6(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0xAE => {
                self.ldxae(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0xBE => {
                // TODO: page cross timing
                self.ldxbe(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }

            // LDY
            0xA0 => {
                self.ldya0(op_u8);
                self.pc += 1;
                cycle_time = 2;
            }
            0xA4 => {
                self.ldya4(op_u8);
                self.pc += 1;
                cycle_time = 3;
            }
            0xB4 => {
                self.ldyb4(op_u8);
                self.pc += 1;
                cycle_time = 4;
            }
            0xAC => {
                self.ldyac(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
            0xBC => {
                // TODO: page cross timing
                self.ldybc(op_u16);
                self.pc += 2;
                cycle_time = 4;
            }
        }

        // synchronizes to clockspeed
        for _ in 0..cycle_time {
            while SECS_PER_CYCLE > start.elapsed().as_secs_f32() {}
            let start = Instant::now();
        }
        self.pc += 1;
    }

    /// encodes the status flags into a single byte
    fn get_status(&self) -> u8 {
        let mut status_code: u8 = 0;
        if self.negative {
            status_code |= 0b1000_0000;
        }

        if self.overflow {
            status_code |= 0b0100_0000;
        }

        if self.b {
            status_code |= 0b0011_0000;
        } else {
            status_code |= 0b0010_0000;
        }

        if self.decimal {
            status_code |= 0b0000_1000;
        }

        if self.interrupt_disable {
            status_code |= 0b0000_0100;
        }

        if self.zero {
            status_code |= 0b0000_0010;
        }

        if self.carry == 1 {
            status_code |= 0b0000_0001;
        }

        status_code
    }

    /// decodes status flag byte into their corresponding struct fields
    fn set_status(&mut self, status_byte: u8) {
        self.negative = status_byte & 0b1000_0000 == 0b1000_0000;
        self.overflow = status_byte & 0b0100_0000 == 0b0100_0000;
        self.b = status_byte & 0b0001_0000 == 0b0001_0000;
        self.decimal = status_byte & 0b0000_1000 == 0b0000_1000;
        self.interrupt_disable = status_byte & 0b0000_0100 == 0b0000_0100;
        self.zero = status_byte & 0b0000_0010 == 0b0000_0010;
        self.carry = status_byte & 0b0000_0001;
    }

    fn get_indirect_addr(&mut self, operand: u8) -> u16 {
        let addr1: u16 = (self.ram[(operand.wrapping_add(1)) as usize] as u16) << 8;
        let addr2: u16 = self.ram[operand as usize] as u16;
        addr1 | addr2
    }

    fn push_to_stack(&mut self, val: u8) {
        self.ram[mmap::ram::stack::START + self.sp as usize] = val;
        self.sp -= 1;
    }

    fn pop_from_stack(&mut self) -> u8 {
        self.sp += 1;
        self.ram[mmap::ram::stack::START + self.sp as usize]
    }

    // --- INSTRUCTIONS ---

    /// ADC #$44
    fn adc69(&mut self, operand: u8) {
        let (inum, overflowed1) = unsafe {
            mem::transmute::<u8, i8>(operand).overflowing_add(mem::transmute::<u8, i8>(self.carry))
        };
        let (_, overflowed2) = unsafe { inum.overflowing_add(mem::transmute::<u8, i8>(self.a)) };

        let (num, carried1) = operand.overflowing_add(self.carry);
        let (res, carried2) = num.overflowing_add(self.a);

        self.a = res;

        self.carry = if carried1 || carried2 { 1 } else { 0 };
        self.overflow = overflowed1 || overflowed2;
        self.zero = self.a == 0;
        self.negative = (self.a >> 7) == 1;
    }

    /// ADC $44
    fn adc65(&mut self, operand: u8) {
        let operand = self.ram[operand as usize];
        self.adc69(operand);
    }

    /// ADC $44, X
    fn adc75(&mut self, operand: u8) {
        let operand = self.ram[(operand.wrapping_add(self.x)) as usize];
        self.adc69(operand);
    }

    /// ADC $4400
    fn adc6d(&mut self, operand: u16) {
        let operand = self.ram[operand as usize];
        self.adc69(operand);
    }

    /// ADC $4400, X
    fn adc7d(&mut self, operand: u16) {
        let operand = self.ram[(operand.wrapping_add(self.x as u16)) as usize];
        self.adc69(operand);
    }

    /// ADC $4400,Y
    fn adc79(&mut self, operand: u16) {
        let operand = self.ram[(operand.wrapping_add(self.y as u16)) as usize];
        self.adc69(operand);
    }

    /// ADC ($44,X)
    fn adc61(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand.wrapping_add(self.x));
        self.adc69(self.ram[addr as usize]);
    }

    /// ADC ($44),Y
    fn adc71(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand);
        let operand = self.ram[(addr.wrapping_add(self.y as u16)) as usize];
        self.adc69(operand);
    }

    /// AND #$44
    fn and29(&mut self, operand: u8) {
        self.a &= operand;
        self.zero = self.a == 0;
        self.negative = self.a >> 7 == 1;
    }

    /// AND $44
    fn and25(&mut self, operand: u8) {
        let operand = self.ram[operand as usize];
        self.and29(operand);
    }

    /// AND $44,X
    fn and35(&mut self, operand: u8) {
        let operand = self.ram[(operand.wrapping_add(self.x)) as usize];
        self.and29(operand);
    }

    /// AND $4400
    fn and2d(&mut self, operand: u16) {
        let operand = self.ram[operand as usize];
        self.and29(operand);
    }

    /// AND $4400,X
    fn and3d(&mut self, operand: u16) {
        let operand = self.ram[(operand.wrapping_add(self.x as u16)) as usize];
        self.and29(operand);
    }

    /// AND $4400,Y
    fn and39(&mut self, operand: u16) {
        let operand = self.ram[(operand.wrapping_add(self.y as u16)) as usize];
        self.and29(operand);
    }

    /// AND ($44,X)
    fn and21(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand.wrapping_add(self.x));
        let operand = self.ram[addr as usize];
        self.and29(operand);
    }

    /// AND ($44),Y
    fn and31(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand);
        let operand = self.ram[(addr.wrapping_add(self.y as u16)) as usize];
        self.and29(operand);
    }

    /// ASL A
    fn asl0a(&mut self) {
        self.carry = (self.a & 0b1000_0000) >> 7;
        self.a <<= 1;
        self.a &= 0b1111_1110;

        self.zero = self.a == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// ASL $4400
    fn asl0e(&mut self, operand: u16) {
        let operand = operand as usize;
        self.carry = (self.ram[operand] & 0b1000_0000) >> 7;
        self.ram[operand] <<= 1;
        self.ram[operand] &= 0b1111_1110;

        self.zero = self.ram[operand] == 0;
        self.negative = (self.ram[operand] & 0b1000_0000) == 0b1000_0000;
    }

    /// ASL $44
    fn asl06(&mut self, operand: u8) {
        self.asl0e(operand as u16);
    }

    /// ASL $44,X
    fn asl16(&mut self, operand: u8) {
        self.asl06(operand.wrapping_add(self.x));
    }

    /// ASL $4400, X
    fn asl1e(&mut self, operand: u16) {
        self.asl0e(operand.wrapping_add(self.x as u16));
    }

    /// BCC $44
    fn bcc90(&mut self, operand: u8) {
        if self.carry == 0 {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// BCS $44
    fn bcsb0(&mut self, operand: u8) {
        if self.carry == 1 {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// BEQ $44
    fn beqf0(&mut self, operand: u8) {
        if self.zero {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// helper function for BIT instructions
    fn bit_helper(&mut self, operand: u16) {
        let value = self.ram[operand as usize] & self.a;

        self.zero = value == 0;
        self.negative = value & 0b1000_0000 == 0b1000_0000;
        self.overflow = value & 0b0100_0000 == 0b0100_0000;
    }

    /// BIT $44
    fn bit24(&mut self, operand: u8) {
        self.bit_helper(operand as u16);
    }

    /// BIT $4400
    fn bit2c(&mut self, operand: u16) {
        self.bit_helper(operand);
    }

    /// BMI $44
    fn bmi30(&mut self, operand: u8) {
        if self.negative {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// BNE $44
    fn bned0(&mut self, operand: u8) {
        if !self.zero {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// BPL $44
    fn bpl10(&mut self, operand: u8) {
        if !self.negative {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// BRK
    fn brk00(&mut self) {
        let pc_lsb = (self.pc & 0x00FF) as u8;
        let pc_msb = ((self.pc & 0xFF00) >> 8) as u8;
        self.push_to_stack(pc_msb);
        self.push_to_stack(pc_lsb);
        self.push_to_stack(self.get_status());

        let irq: u16 = self.ram[mmap::cpu::irq_brk::START] as u16
            | (self.ram[mmap::cpu::irq_brk::END] as u16) << 8;

        self.pc = irq;
        self.b = true;
    }

    /// BVC $44
    fn bvc50(&mut self, operand: u8) {
        if !self.overflow {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// BVS $44
    fn bvs70(&mut self, operand: u8) {
        if self.overflow {
            self.pc = self.pc.wrapping_add(operand as u16);
        }
    }

    /// CLC
    fn clc18(&mut self) {
        self.carry = 0;
    }

    /// CLD
    fn cldd8(&mut self) {
        self.decimal = false;
    }

    /// CLI
    fn cli58(&mut self) {
        self.interrupt_disable = false;
    }

    /// CLV
    fn clvb8(&mut self) {
        self.overflow = false;
    }

    fn cmp_helper(&mut self, operand: u16) {
        let acc = self.a as u16;
        let res = acc.wrapping_sub(operand);
        self.carry = if acc >= operand { 1 } else { 0 };

        self.zero = acc == operand;
        self.negative = (res & 0b1000_0000) == 0b1000_0000;
        self.carry = if acc >= operand { 1 } else { 0 };
    }

    /// CMP #$44
    fn cmpc9(&mut self, operand: u8) {
        self.cmp_helper(operand as u16);
    }

    /// CMP $44
    fn cmpc5(&mut self, operand: u8) {
        self.cmpc9(self.ram[operand as usize]);
    }

    /// CMP $44,X
    fn cmpd5(&mut self, operand: u8) {
        let operand = self.ram[(operand.wrapping_add(self.x)) as usize];
        self.cmpc9(operand);
    }

    /// CMP $4400
    fn cmpcd(&mut self, operand: u16) {
        self.cmpc9(self.ram[operand as usize]);
    }

    /// CMP $4400,X
    fn cmpdd(&mut self, operand: u16) {
        let operand = self.ram[(operand.wrapping_add(self.x as u16)) as usize];
        self.cmpc9(operand);
    }

    /// CMP $4400,Y
    fn cmpd9(&mut self, operand: u16) {
        let operand = self.ram[(operand.wrapping_add(self.y as u16)) as usize];
        self.cmpc9(operand);
    }

    /// CMP ($44,X)
    fn cmpc1(&mut self, operand: u8) {
        let operand = self.get_indirect_addr(operand.wrapping_add(self.x));
        self.cmp_helper(operand);
    }

    /// CMP ($44),Y
    fn cmpd1(&mut self, operand: u8) {
        let operand = self.get_indirect_addr(operand);
        self.cmp_helper(operand.wrapping_add(self.y as u16));
    }

    /// CPX #$44
    fn cpxe0(&mut self, operand: u8) {
        self.carry = if self.x >= operand { 1 } else { 0 };
        self.zero = self.x == operand;
        let res = self.x.wrapping_sub(operand);
        self.negative = (res & 0b1000_0000) == 0b1000_0000;
    }

    /// CPX $44
    fn cpxe4(&mut self, operand: u8) {
        self.cpxe0(self.ram[operand as usize]);
    }

    /// CPX $4400
    fn cpxec(&mut self, operand: u16) {
        self.cpxe0(self.ram[operand as usize]);
    }

    /// CPY #$44
    fn cpyc0(&mut self, operand: u8) {
        self.carry = if self.y >= operand { 1 } else { 0 };
        self.zero = self.y == operand;
        let res = self.y.wrapping_sub(operand);
        self.negative = (res & 0b1000_0000) == 0b1000_0000;
    }

    /// CPY $44
    fn cpyc4(&mut self, operand: u8) {
        self.cpyc0(self.ram[operand as usize]);
    }

    /// CPY $4400
    fn cpycc(&mut self, operand: u16) {
        self.cpyc0(self.ram[operand as usize]);
    }

    /// DEC $44
    fn decc6(&mut self, operand: u8) {
        self.ram[operand as usize] -= 1;

        let res = self.ram[operand as usize];
        if res == 0 {
            self.zero = true;
        } else {
            self.zero = false;
        }

        self.negative = (res & 0b1000_0000) == 0b1000_0000;
    }

    /// DEC $44,X
    fn decd6(&mut self, operand: u8) {
        self.decc6(operand.wrapping_add(self.x));
    }

    /// DEC $4400
    fn decce(&mut self, operand: u16) {
        self.ram[operand as usize] -= 1;

        let res = self.ram[operand as usize];
        if res == 0 {
            self.zero = true;
        } else {
            self.zero = false;
        }

        self.negative = (res & 0b1000_0000) == 0b1000_0000;
    }

    /// DEC $4400,X
    fn decde(&mut self, operand: u16) {
        self.decce(operand.wrapping_add(self.x as u16));
    }

    /// DEX
    fn dexca(&mut self) {
        self.x -= 1;
        self.zero = self.x == 0;
        self.negative = (self.x & 0b1000_0000) == 0b1000_0000;
    }

    /// DEY
    fn dey88(&mut self) {
        self.y -= 1;
        self.zero = self.y == 0;
        self.negative = (self.y & 0b1000_0000) == 0b1000_0000;
    }

    /// EOR #$44
    fn eor49(&mut self, operand: u8) {
        self.a ^= operand;
        self.zero = self.a == 0;
        self.negative = (0b1000_0000 & self.a) == 0b1000_0000;
    }

    /// EOR $44
    fn eor45(&mut self, operand: u8) {
        self.eor49(self.ram[operand as usize]);
    }

    /// EOR $44,X
    fn eor55(&mut self, operand: u8) {
        self.eor49(self.ram[(operand.wrapping_add(self.x)) as usize]);
    }

    /// EOR $4400
    fn eor4d(&mut self, operand: u16) {
        self.eor49(self.ram[operand as usize]);
    }

    /// EOR $4400,X
    fn eor5d(&mut self, operand: u16) {
        self.eor49(self.ram[(operand.wrapping_add(self.x as u16)) as usize]);
    }

    /// EOR $4400,Y
    fn eor59(&mut self, operand: u16) {
        self.eor49(self.ram[(operand.wrapping_add(self.y as u16)) as usize]);
    }

    /// EOR ($44,X)
    fn eor41(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand.wrapping_add(self.x));
        self.eor49(self.ram[addr as usize]);
    }

    /// EOR ($44),Y
    fn eor51(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand);
        self.eor49(self.ram[(addr.wrapping_add(self.y as u16)) as usize]);
    }

    /// INC $4400
    fn incee(&mut self, operand: u16) {
        self.ram[operand as usize] = self.ram[operand as usize].wrapping_add(1);
        let res = self.ram[operand as usize];
        self.zero = res == 0;
        self.negative = (0b1000_0000 & res) == 0b1000_0000;
    }

    /// INC $44
    fn ince6(&mut self, operand: u8) {
        self.incee(operand as u16);
    }

    /// INC $44,X
    fn incf6(&mut self, operand: u8) {
        self.ince6(operand.wrapping_add(self.x));
    }

    /// INC $4400,X
    fn incfe(&mut self, operand: u16) {
        self.incee(operand.wrapping_add(self.x as u16));
    }

    /// INX
    fn inxe8(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.zero = self.x == 0;
        self.negative = (0b1000_0000 & self.x) == 0b1000_0000;
    }

    /// INY
    fn inyc8(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.zero = self.y == 0;
        self.negative = (0b1000_0000 & self.y) == 0b1000_0000;
    }

    /// JMP $5597
    fn jmp4c(&mut self, operand: u16) {
        self.pc = operand;
    }

    /// JMP ($5597)
    fn jmp6c(&mut self, operand: u16) {
        let addr2: u16 = self.ram[operand as usize] as u16;

        // introducing paging bug
        let operand = if operand & 0x00FF == 0x00FF {
            operand & 0xFF00
        } else {
            operand
        };

        let addr1: u16 = (self.ram[(operand.wrapping_add(1)) as usize] as u16) << 8;
        self.pc = addr1 | addr2;
    }

    /// JSR $5597
    fn jsr20(&mut self, operand: u16) {
        self.pc -= 1;
        let lsb = self.pc & 0x00FF;
        let msb = (self.pc & 0xFF00) >> 8;
        self.push_to_stack(msb as u8);
        self.push_to_stack(lsb as u8);
        self.pc = operand;
    }

    /// LDA #$44
    fn ldaa9(&mut self, operand: u8) {
        self.a = operand;
        self.zero = operand == 0;
        self.negative = (0b1000_0000 & operand) == 0b1000_0000;
    }

    /// LDA $44
    fn ldaa5(&mut self, operand: u8) {
        self.ldaa9(self.ram[operand as usize]);
    }

    /// LDA $44,X
    fn ldab5(&mut self, operand: u8) {
        self.ldaa9(self.ram[(operand.wrapping_add(self.x)) as usize]);
    }

    /// LDA $4400
    fn ldaad(&mut self, operand: u16) {
        self.ldaa9(self.ram[operand as usize]);
    }

    /// LDA $4400,X
    fn ldabd(&mut self, operand: u16) {
        self.ldaa9(self.ram[operand.wrapping_add(self.x as u16) as usize]);
    }

    /// LDA $4400,Y
    fn ldab9(&mut self, operand: u16) {
        self.ldaa9(self.ram[operand.wrapping_add(self.y as u16) as usize]);
    }

    /// LDA ($44,X)
    fn ldaa1(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand.wrapping_add(self.x));
        self.ldaa9(self.ram[addr as usize]);
    }

    /// LDA ($44),Y
    fn ldab1(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand).wrapping_add(self.y as u16);
        self.ldaa9(self.ram[addr as usize]);
    }

    /// LDX #$44
    fn ldxa2(&mut self, operand: u8) {
        self.x = operand;

        self.zero = operand == 0;
        self.negative = (0b1000_0000 & operand) == 0b1000_0000;
    }

    /// LDX $44
    fn ldxa6(&mut self, operand: u8) {
        self.ldxa2(self.ram[operand as usize]);
    }

    /// LDX $44,Y
    fn ldxb6(&mut self, operand: u8) {
        self.ldxa2(self.ram[(operand.wrapping_add(self.y)) as usize]);
    }

    /// LDX $4400
    fn ldxae(&mut self, operand: u16) {
        self.ldxa2(self.ram[operand as usize]);
    }

    /// LDX $4400,Y
    fn ldxbe(&mut self, operand: u16) {
        self.ldxa2(self.ram[operand.wrapping_add(self.y as u16) as usize]);
    }

    /// LDY #$44
    fn ldya0(&mut self, operand: u8) {
        self.y = operand;

        self.zero = operand == 0;
        self.negative = (0b1000_0000 & operand) == 0b1000_0000;
    }

    /// LDY $44
    fn ldya4(&mut self, operand: u8) {
        self.ldya0(self.ram[operand as usize]);
    }

    /// LDY $44,X
    fn ldyb4(&mut self, operand: u8) {
        self.ldya0(self.ram[(operand.wrapping_add(self.x)) as usize]);
    }

    /// LDY $4400
    fn ldyac(&mut self, operand: u16) {
        self.ldya0(self.ram[operand as usize]);
    }

    /// LDY $4400,X
    fn ldybc(&mut self, operand: u16) {
        self.ldya0(self.ram[operand.wrapping_add(self.x as u16) as usize]);
    }

    /// LSR A
    fn lsr4a(&mut self) {
        self.carry = self.a & 0b0000_0001;
        self.a = (self.a & 0b0111_1111) >> 1;
        self.zero = self.a == 0;
        self.negative = false;
    }

    /// LSR $4400
    fn lsr4e(&mut self, operand: u16) {
        let operand = operand as usize;
        self.carry = self.ram[operand] & 0b0000_0001;
        self.ram[operand] = (self.ram[operand] & 0b0111_1111) >> 1;
        self.zero = self.ram[operand] == 0;
        self.negative = false;
    }

    /// LSR $44
    fn lsr46(&mut self, operand: u8) {
        self.lsr4e(operand as u16);
    }

    /// LSR $44,X
    fn lsr56(&mut self, operand: u8) {
        self.lsr46(operand + self.x);
    }

    /// LSR $4400,X
    fn lsr5e(&mut self, operand: u16) {
        self.lsr4e(operand + self.x as u16);
    }

    /// ORA #$44
    fn ora09(&mut self, operand: u8) {
        self.a |= operand;
        self.zero = self.a == 0;
        self.negative = (operand & 0b1000_0000) == 0b1000_0000;
    }

    /// ORA $44
    fn ora05(&mut self, operand: u8) {
        self.ora09(self.ram[operand as usize]);
    }

    /// ORA $44,X
    fn ora15(&mut self, operand: u8) {
        self.ora09(self.ram[(operand + self.x) as usize]);
    }

    /// ORA $4400
    fn ora0d(&mut self, operand: u16) {
        self.ora09(self.ram[operand as usize]);
    }

    /// ORA $4400,X
    fn ora1d(&mut self, operand: u16) {
        self.ora09(self.ram[(operand + self.x as u16) as usize]);
    }

    /// ORA $4400,Y
    fn ora19(&mut self, operand: u16) {
        self.ora09(self.ram[(operand + self.y as u16) as usize]);
    }

    /// ORA ($44,X)
    fn ora01(&mut self, operand: u8) {
        let operand = self.get_indirect_addr(operand + self.x);
        self.ora09(self.ram[operand as usize]);
    }

    /// ORA ($44),Y
    fn ora11(&mut self, operand: u8) {
        let operand = self.get_indirect_addr(operand) + self.y as u16;
        self.ora09(self.ram[operand as usize]);
    }

    /// PHA
    fn pha48(&mut self) {
        self.push_to_stack(self.a);
    }

    /// PHP
    fn php08(&mut self) {
        self.push_to_stack(self.get_status());
    }

    /// PLA
    fn pla68(&mut self) {
        self.a = self.pop_from_stack();
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// PLP
    fn plp28(&mut self) {
        let status = self.pop_from_stack();
        self.set_status(status);
    }

    /// ROL A
    fn rol2a(&mut self) {
        let old = self.a;
        self.a = (self.a << 1) | self.carry;

        self.carry = old >> 7;
        self.zero = self.a == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// ROL $4400
    fn rol2e(&mut self, operand: u16) {
        let operand = operand as usize;
        let old = self.ram[operand];
        self.ram[operand] = (self.ram[operand] << 1) | self.carry;

        self.carry = old >> 7;
        self.zero = self.ram[operand] == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// ROL $44
    fn rol26(&mut self, operand: u8) {
        self.rol2e(operand as u16);
    }

    /// ROL $44,X
    fn rol36(&mut self, operand: u8) {
        self.rol26(operand + self.x);
    }

    /// ROL $4400,X
    fn rol3e(&mut self, operand: u16) {
        self.rol2e(operand + self.x as u16);
    }

    /// ROR A
    fn ror6a(&mut self) {
        let old = self.a;
        self.a = (self.a >> 1) | self.carry;

        self.carry = old >> 7;
        self.zero = self.a == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// ROR $4400
    fn ror6e(&mut self, operand: u16) {
        let operand = operand as usize;
        let old = self.ram[operand];
        self.ram[operand] = (self.ram[operand] >> 1) | self.carry;

        self.carry = old >> 7;
        self.zero = self.ram[operand] == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// ROR $44
    fn ror66(&mut self, operand: u8) {
        self.ror6e(operand as u16);
    }

    /// ROR $44,X
    fn ror76(&mut self, operand: u8) {
        self.ror66(operand + self.x);
    }

    /// ROR $4400,X
    fn ror7e(&mut self, operand: u16) {
        self.ror6e(operand + self.x as u16);
    }

    /// RTI
    fn rti40(&mut self) {
        let status = self.pop_from_stack();
        let pc: u16 = (self.pop_from_stack() as u16) | ((self.pop_from_stack() as u16) << 8);
        self.set_status(status);
        self.pc = pc;
    }

    /// RTS
    fn rts60(&mut self) {
        let pc = self.pop_from_stack() as u16 | ((self.pop_from_stack() as u16) << 8);
        self.pc = pc;
    }

    /// SBC #$44
    fn sbce9(&mut self, operand: u8) {
        let (acc, overflowed1) = self.a.overflowing_sub(operand);
        let (acc, overflowed2) = acc.overflowing_sub(!self.carry & 0b0000_0001);

        self.a = acc;
        self.zero = self.a == 0;
        self.negative = (acc & 0b1000_0000) == 0b1000_0000;
        self.overflow = overflowed1 || overflowed2;

        if overflowed1 || overflowed2 {
            self.carry = 0;
        }
    }

    /// SBC $44
    fn sbce5(&mut self, operand: u8) {
        self.sbce9(self.ram[operand as usize]);
    }

    /// SBC $44,X
    fn sbcf5(&mut self, operand: u8) {
        self.sbce9(self.ram[(operand + self.x) as usize]);
    }

    /// SBC $4400
    fn sbced(&mut self, operand: u16) {
        self.sbce9(self.ram[operand as usize]);
    }

    /// SBC $4400,X
    fn sbcfd(&mut self, operand: u16) {
        self.sbce9(self.ram[(operand + self.x as u16) as usize]);
    }

    /// SBC $4400,Y
    fn sbcf9(&mut self, operand: u16) {
        self.sbce9(self.ram[(operand + self.y as u16) as usize]);
    }

    /// SBC ($44,X)
    fn sbce1(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand + self.x);
        self.sbce9(self.ram[addr as usize]);
    }

    /// SBC ($44),Y
    fn sbcf1(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand);
        self.sbce9(self.ram[(addr + self.y as u16) as usize]);
    }

    /// SEC
    fn sec38(&mut self) {
        self.carry = 1;
    }

    /// SEI
    fn sei78(&mut self) {
        self.interrupt_disable = true;
    }

    /// STA $4400
    fn sta8d(&mut self, operand: u16) {
        self.ram[operand as usize] = self.a;
    }

    /// STA $44
    fn sta85(&mut self, operand: u8) {
        self.sta8d(operand as u16);
    }

    /// STA $44,X
    fn sta95(&mut self, operand: u8) {
        self.sta85(operand + self.x);
    }

    /// STA $4400,X
    fn sta9d(&mut self, operand: u16) {
        self.sta8d(operand + self.x as u16);
    }

    /// STA $4400,Y
    fn sta99(&mut self, operand: u16) {
        self.sta8d(operand + self.y as u16);
    }

    /// STA ($44,X)
    fn sta81(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand + self.x);
        self.sta8d(addr);
    }

    /// STA ($44),Y
    fn sta91(&mut self, operand: u8) {
        let addr = self.get_indirect_addr(operand);
        self.sta8d(addr + self.y as u16);
    }

    /// STX $4400
    fn stx8e(&mut self, operand: u16) {
        self.ram[operand as usize] = self.x;
    }

    /// STX $44
    fn stx86(&mut self, operand: u8) {
        self.stx8e(operand as u16);
    }

    /// STX $44,Y
    fn stx96(&mut self, operand: u8) {
        self.stx86(operand + self.y);
    }

    /// STY $4400
    fn sty8c(&mut self, operand: u16) {
        self.ram[operand as usize] = self.y;
    }

    /// STY $44
    fn sty84(&mut self, operand: u8) {
        self.stx8e(operand as u16);
    }

    /// STY $44,X
    fn sty94(&mut self, operand: u8) {
        self.stx86(operand + self.x);
    }

    /// TAX
    fn taxaa(&mut self) {
        self.x = self.a;
        self.zero = self.x == 0;
        self.negative = (self.x & 0b1000_0000) == 0b1000_0000;
    }

    /// TAY
    fn taya8(&mut self) {
        self.y = self.a;
        self.zero = self.y == 0;
        self.negative = (self.y & 0b1000_0000) == 0b1000_0000;
    }

    /// TSX
    fn tsxba(&mut self) {
        self.x = self.sp;
        self.zero = self.x == 0;
        self.negative = (self.x & 0b1000_0000) == 0b1000_0000;
    }

    /// TXA
    fn txa8a(&mut self) {
        self.a = self.x;
        self.zero = self.a == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }

    /// TXS
    fn txs9a(&mut self) {
        self.sp = self.x;
    }

    /// TYA
    fn tya98(&mut self) {
        self.a = self.y;
        self.zero = self.a == 0;
        self.negative = (self.a & 0b1000_0000) == 0b1000_0000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adc_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 255;
        cpu.adc69(1);
        assert!(cpu.a == 0);
        assert!(cpu.carry == 1);
        assert!(cpu.zero == true);
        cpu.adc69(254);
        assert!(cpu.negative == true);
        cpu.a = unsafe { mem::transmute::<i8, u8>(i8::MAX) };
        cpu.adc69(1);
        assert!(cpu.overflow == true);

        // the following opcodes all use adc69 underneath so there's no need to test flags again
        cpu.ram[0x44] = 29;
        cpu.a = 3;
        cpu.adc65(0x44);
        assert!(cpu.a == 32);

        cpu.a = 4;
        cpu.ram[0xff] = 50;
        cpu.x = 0x2;
        cpu.adc75(0xfd);
        assert!(cpu.a == 54);

        cpu.ram[0x1ee] = 244;
        cpu.a = 2;
        cpu.adc6d(0x1ee);
        assert!(cpu.a == 246);

        cpu.ram[0x1ee] = 156;
        cpu.a = 2;
        cpu.x = 0xee;
        cpu.adc7d(0x100);
        assert!(cpu.a == 158);

        cpu.ram[0x7ee] = 70;
        cpu.a = 90;
        cpu.y = 0xe0;
        cpu.adc79(0x70e);
        assert!(cpu.a == 160);

        cpu.ram[0x45] = 0xab;
        cpu.ram[0x46] = 0x01;
        cpu.ram[0x01ab] = 222;
        cpu.a = 0;
        cpu.x = 1;
        cpu.adc61(0x44);
        assert!(cpu.a == 222);

        cpu.a = 0;
        cpu.ram[0xaa] = 0xca;
        cpu.ram[0xab] = 0x01;
        cpu.y = 3;
        cpu.ram[0x01cd] = 111;
        cpu.adc71(0xaa);
        assert!(cpu.a == 111);
    }

    #[test]
    fn and_opcodes() {
        let mut cpu = CPU::default();
        cpu.a = 0x0f;
        cpu.and29(0xf0);
        assert!(cpu.a == 0);
        assert!(cpu.zero == true);
        cpu.a = 0xfa;
        cpu.and29(0x0f);
        assert!(cpu.a == 0x0a);
        cpu.a = 0xff;
        cpu.and29(0xff);
        assert!(cpu.negative == true);

        // the following opcodes all use and29 underneath so there's no need to test flags again
        cpu.ram[0xaa] = 0xf0;
        cpu.a = 0xea;
        cpu.and25(0xaa);
        assert!(cpu.a == 0xe0);

        cpu.ram[40] = 0xff;
        cpu.a = 0xEE;
        cpu.x = 2;
        cpu.and35(38);
        assert!(cpu.a == 0xee);

        cpu.ram[2000] = 0x0f;
        cpu.a = 0xac;
        cpu.and2d(2000);
        assert!(cpu.a == 0xc);

        cpu.ram[2005] = 0x56;
        cpu.a = 0xf0;
        cpu.x = 5;
        cpu.and3d(2000);
        assert!(cpu.a == 0x50);

        cpu.ram[2046] = 0xa7;
        cpu.a = 0x0f;
        cpu.y = 10;
        cpu.and39(2036);
        assert!(cpu.a == 0x7);

        cpu.ram[10] = 0x0e;
        cpu.ram[11] = 0x1;
        cpu.ram[0x10e] = 0xcc;
        cpu.a = 0xf0;
        cpu.x = 3;
        cpu.and21(7);
        assert!(cpu.a == 0xc0);

        cpu.ram[10] = 0x0f;
        cpu.ram[11] = 0x02;
        cpu.ram[0x211] = 0xdd;
        cpu.a = 0x0f;
        cpu.y = 2;
        cpu.and31(10);
        assert!(cpu.a == 0xd);
    }

    #[test]
    fn asl_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 0b1011_1111;
        cpu.asl0a();
        assert!(cpu.a == 0b0111_1110);
        assert!(cpu.carry == 1);
        assert!(cpu.zero == false);
        assert!(cpu.negative == false);
        cpu.a = 0b1000_0000;
        cpu.asl0a();
        assert!(cpu.zero == true);
        cpu.a = 0b0100_0000;
        cpu.asl0a();
        assert!(cpu.negative == true);

        // has the same logic as asl0a so flag testing is skipped
        // also skips testing asl0e as underneath asl06 uses it to limit to 8 bits
        cpu.ram[150] = 0b1011_1111;
        cpu.asl06(150);
        assert!(cpu.ram[150] == 0b0111_1110);
        // also skips the other intructions because they all rely on the opcode above
    }

    #[test]
    fn branch_opcodes() {
        let mut cpu = CPU::default();

        cpu.carry = 0;
        cpu.bcc90(200);
        assert!(cpu.pc == 200);
        cpu.carry = 1;
        cpu.bcc90(50);
        assert!(cpu.pc != 250);

        cpu.pc = 0;
        cpu.carry = 1;
        cpu.bcsb0(200);
        assert!(cpu.pc == 200);
        cpu.carry = 0;
        cpu.bcsb0(50);
        assert!(cpu.pc != 250);

        cpu.pc = 0;
        cpu.zero = true;
        cpu.beqf0(100);
        assert!(cpu.pc == 100);
        cpu.zero = false;
        cpu.beqf0(100);
        assert!(cpu.pc != 200);

        cpu.pc = 0;
        cpu.negative = true;
        cpu.bmi30(100);
        assert!(cpu.pc == 100);
        cpu.negative = false;
        cpu.bmi30(100);
        assert!(cpu.pc != 200);

        cpu.pc = 0;
        cpu.zero = false;
        cpu.bned0(100);
        assert!(cpu.pc == 100);
        cpu.zero = true;
        cpu.bned0(100);
        assert!(cpu.pc != 200);

        // BPL $44
        cpu.pc = 0;
        cpu.negative = false;
        cpu.bpl10(100);
        assert!(cpu.pc == 100);
        cpu.negative = true;
        cpu.bpl10(100);
        assert!(cpu.pc != 200);

        cpu.pc = 0;
        cpu.overflow = false;
        cpu.bvc50(25);
        assert!(cpu.pc == 25);

        cpu.pc = 0;
        cpu.overflow = true;
        cpu.bvs70(100);
        assert!(cpu.pc == 100);
    }

    #[test]
    fn bit_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 0b1111_0000;
        cpu.ram[55] = 0b1100_1111;
        cpu.bit24(55);
        assert!(cpu.negative == true);
        assert!(cpu.overflow == true);
        cpu.a = 0b1111_0000;
        cpu.ram[55] = 0b0011_1111;
        cpu.bit24(55);
        assert!(cpu.negative == false);
        assert!(cpu.overflow == false);
        assert!(cpu.zero == false);
        cpu.a = 0b0000_1111;
        cpu.ram[55] = 0b1111_0000;
        cpu.bit24(55);
        assert!(cpu.zero == true);
        // no need to test bit2c, it has the same implementation
    }

    #[test]
    fn brk_opcode() {
        // TODO uncomment once PPU memory is implemented
        // let mut cpu = CPU::default();
        // cpu.ram[mmap::cpu::irq_brk::START] = 0xff;
        // cpu.ram[mmap::cpu::irq_brk::END] = 0x02;
        // cpu.brk00();
        // assert!(cpu.pc == 0xea);
    }

    #[test]
    fn clear_opcodes() {
        let mut cpu = CPU::default();
        cpu.carry = 1;
        cpu.clc18();
        assert!(cpu.carry == 0);

        cpu.decimal = true;
        cpu.cldd8();
        assert!(cpu.decimal == false);

        cpu.interrupt_disable = true;
        cpu.cli58();
        assert!(cpu.interrupt_disable == false);

        cpu.overflow = true;
        cpu.clvb8();
        assert!(cpu.overflow == false);
    }

    #[test]
    fn cmp_opcodes() {
        let mut cpu = CPU::default();
        cpu.a = 0xfe;
        cpu.cmpc9(0xfe);
        assert!(cpu.zero == true);
        cpu.a = 0xfe;
        cpu.cmpc9(0x10);
        assert!(cpu.carry == 1);
        assert!(cpu.negative == true);

        cpu.a = 0xfe;
        cpu.ram[0x80] = 0xfe;
        cpu.cmpc5(0x80);
        assert!(cpu.zero == true);

        cpu.a = 0xfe;
        cpu.ram[0x81] = 0xfe;
        cpu.x = 1;
        cpu.cmpd5(0x80);
        assert!(cpu.zero == true);

        cpu.a = 0xfe;
        cpu.ram[0x80] = 0xfe;
        cpu.cmpcd(0x80);
        assert!(cpu.zero == true);

        cpu.a = 0xfe;
        cpu.ram[0x701] = 0xfe;
        cpu.x = 1;
        cpu.cmpdd(0x700);
        assert!(cpu.zero == true);

        cpu.a = 0xfe;
        cpu.ram[0x701] = 0xfe;
        cpu.y = 1;
        cpu.cmpd9(0x700);
        assert!(cpu.zero == true);

        cpu.a = 0xfe;
        cpu.ram[240] = 0xfe;
        cpu.ram[241] = 0x0;
        cpu.ram[80] = 240;
        cpu.x = 1;
        cpu.cmpc1(79);

        // CMP ($44),Y
        cpu.a = 0xfe;
        cpu.ram[240] = 0xfe;
        cpu.ram[241] = 0x01;
        cpu.ram[0x1ff] = 0xfe;
        cpu.y = 1;
        cpu.cmpd1(240);
    }

    #[test]
    fn cpx_opcodes() {
        let mut cpu = CPU::default();
        cpu.x = 0xea;
        cpu.cpxe0(0xea);
        assert!(cpu.zero == true);

        cpu.x = 0xea;
        cpu.ram[200] = 0xea;
        cpu.cpxe4(200);
        assert!(cpu.zero == true);

        cpu.x = 0xea;
        cpu.ram[2023] = 0xea;
        cpu.cpxec(2023);
        assert!(cpu.zero == true);
    }

    #[test]
    fn cpy_opcodes() {
        let mut cpu = CPU::default();
        cpu.y = 0xea;
        cpu.cpyc0(0xea);
        assert!(cpu.zero == true);

        cpu.y = 0xea;
        cpu.ram[200] = 0xea;
        cpu.cpyc4(200);
        assert!(cpu.zero == true);

        cpu.y = 0xea;
        cpu.ram[2023] = 0xea;
        cpu.cpycc(2023);
        assert!(cpu.zero == true);
    }

    #[test]
    fn dec_opcodes() {
        let mut cpu = CPU::default();
        cpu.ram[0x80] = 2;
        cpu.decc6(0x80);
        assert!(cpu.ram[0x80] == 1);

        cpu.x = 1;
        cpu.ram[0x81] = 2;
        cpu.decd6(0x80);
        assert!(cpu.ram[0x81] == 1);

        cpu.ram[1400] = 5;
        cpu.decce(1400);
        assert!(cpu.ram[1400] == 4);

        cpu.ram[1401] = 5;
        cpu.decde(1400);
        assert!(cpu.ram[1401] == 4);

        cpu.x = 1;
        cpu.dexca();
        assert!(cpu.x == 0);

        cpu.y = 2;
        cpu.dey88();
        assert!(cpu.y == 1);
    }

    #[test]
    fn eor_opcodes() {
        let mut cpu = CPU::default();
        cpu.a = 100;
        cpu.eor49(50);
        assert!(cpu.a == 86);

        cpu.a = 100;
        cpu.ram[200] = 50;
        cpu.eor45(200);
        assert!(cpu.a == 86);

        cpu.a = 100;
        cpu.x = 2;
        cpu.ram[52] = 100;
        cpu.eor55(50);
        assert!(cpu.zero == true);
    }

    #[test]
    fn inc_opcodes() {
        let mut cpu = CPU::default();
        cpu.ram[122] = 10;
        cpu.ince6(122);
        assert!(cpu.ram[122] == 11);

        cpu.ram[122] = 255;
        cpu.ince6(122);
        assert!(cpu.ram[122] == 0);

        cpu.x = 20;
        cpu.inxe8();
        assert!(cpu.x == 21);

        cpu.y = 20;
        cpu.inyc8();
        assert!(cpu.y == 21);
    }

    #[test]
    fn jmp_opcodes() {
        let mut cpu = CPU::default();

        cpu.jmp4c(670);
        assert!(cpu.pc == 670);

        cpu.ram[700] = 0xff;
        cpu.ram[701] = 0x0a;
        cpu.jmp6c(700);
        assert!(cpu.pc == 0x0aff);

        cpu.ram[0x1ff] = 0xff;
        cpu.ram[0x200] = 0x0a;
        cpu.ram[0x100] = 0x01;
        cpu.jmp6c(0x1ff);
        assert!(cpu.pc != 0x0aff);
        assert!(cpu.pc != 0x01ff);

        cpu.jsr20(2300);
        assert!(cpu.pc == 2300);
    }

    #[test]
    fn lda_opcodes() {
        let mut cpu = CPU::default();
        cpu.ldaa9(0xff);
        assert!(cpu.a == 0xff);

        cpu.ram[0xff] = 0xfe;
        cpu.ldaa5(0xff);
        assert!(cpu.a == 0xfe);

        cpu.x = 2;
        cpu.ram[3] = 59;
        cpu.ldab5(1);
        assert!(cpu.a == 59);

        cpu.ram[2000] = 55;
        cpu.ldaad(2000);
        assert!(cpu.a == 55);

        cpu.x = 1;
        cpu.ram[2001] = 222;
        cpu.ldabd(2000);
        assert!(cpu.a == 222);

        cpu.y = 1;
        cpu.ram[2001] = 223;
        cpu.ldab9(2000);
        assert!(cpu.a == 223);

        cpu.ram[142] = 0xbb;
        cpu.ram[143] = 0x01;
        cpu.ram[0x01bb] = 0xee;
        cpu.x = 2;
        cpu.ldaa1(140);
        assert!(cpu.a == 0xee);

        cpu.ram[142] = 0xb3;
        cpu.ram[143] = 0x01;
        cpu.ram[0x01b5] = 0xaa;
        cpu.y = 2;
        cpu.ldab1(142);
        assert!(cpu.a == 0xaa);
    }

    #[test]
    fn ldx_opcodes() {
        let mut cpu = CPU::default();
        cpu.ldxa2(0xff);
        assert!(cpu.x == 0xff);

        cpu.ram[0xff] = 0xfe;
        cpu.ldxa6(0xff);
        assert!(cpu.x == 0xfe);

        cpu.y = 2;
        cpu.ram[3] = 59;
        cpu.ldxb6(1);
        assert!(cpu.x == 59);

        cpu.ram[2000] = 55;
        cpu.ldxae(2000);
        assert!(cpu.x == 55);

        cpu.y = 1;
        cpu.ram[2001] = 222;
        cpu.ldxbe(2000);
        assert!(cpu.x == 222);
    }

    #[test]
    fn ldy_opcodes() {
        let mut cpu = CPU::default();
        cpu.ldya0(0xff);
        assert!(cpu.y == 0xff);

        cpu.ram[0xff] = 0xfe;
        cpu.ldya4(0xff);
        assert!(cpu.y == 0xfe);

        cpu.x = 2;
        cpu.ram[3] = 59;
        cpu.ldyb4(1);
        assert!(cpu.y == 59);

        cpu.ram[2000] = 55;
        cpu.ldyac(2000);
        assert!(cpu.y == 55);

        cpu.x = 1;
        cpu.ram[2001] = 222;
        cpu.ldybc(2000);
        assert!(cpu.y == 222);
    }

    #[test]
    fn lsr_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 0b0101_0101;
        cpu.lsr4a();
        assert!(cpu.a == 0b0010_1010);

        cpu.ram[100] = 0b0101_0101;
        cpu.lsr4e(100);
        assert!(cpu.ram[100] == 0b0010_1010);
    }

    #[test]
    fn ora_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 0b1010_1010;
        cpu.ora09(0b0101_0101);
        assert!(cpu.a == 0xff);

        cpu.a = 0b1010_1010;
        cpu.ram[1600] = 0b0101_0101;
        cpu.ora0d(1600);
        assert!(cpu.a == 0xff);

        cpu.x = 2;
        cpu.ram[0xaa] = 0x10;
        cpu.ram[0xab] = 0x02;
        cpu.ram[0x0210] = 0b0101_0101;
        cpu.ora01(98);
        assert!(cpu.a == 0xff);

        cpu.x = 2;
        cpu.ram[0xaa] = 0x10;
        cpu.ram[0xab] = 0x02;
        cpu.ram[0x0212] = 0b0101_0101;
        cpu.ora11(100);
        assert!(cpu.a == 0xff);
    }

    #[test]
    fn pha_pla_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 43;
        cpu.pha48();
        cpu.a = 23;
        cpu.pla68();
        assert!(cpu.a == 43);
    }

    #[test]
    fn php_plp_opcodes() {
        let mut cpu = CPU::default();

        cpu.carry = 1;
        cpu.negative = false;
        cpu.overflow = true;
        cpu.decimal = false;
        cpu.interrupt_disable = true;
        cpu.zero = false;
        cpu.b = true;
        let status = cpu.get_status();
        cpu.php08();

        cpu.carry = 0;
        cpu.negative = true;
        cpu.overflow = false;
        cpu.decimal = true;
        cpu.interrupt_disable = true;
        cpu.zero = true;
        cpu.b = false;
        cpu.plp28();
        assert!(cpu.get_status() == status);
    }

    #[test]
    fn rol_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 0b0111_0101;
        cpu.carry = 1;
        cpu.rol2a();
        assert!(cpu.a == 0b1110_1011);

        cpu.ram[1200] = 0b0111_0101;
        cpu.carry = 1;
        cpu.rol2e(1200);
        assert!(cpu.ram[1200] == 0b1110_1011);

        cpu.ram[1202] = 0b0111_0101;
        cpu.carry = 1;
        cpu.x = 2;
        cpu.rol3e(1200);
        assert!(cpu.ram[1202] == 0b1110_1011);
    }

    #[test]
    fn ror_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 0b0111_0101;
        cpu.carry = 1;
        cpu.ror6a();
        assert!(cpu.a == 0b0011_1011);

        cpu.ram[1200] = 0b0111_0101;
        cpu.carry = 1;
        cpu.ror6e(1200);
        assert!(cpu.ram[1200] == 0b0011_1011);

        cpu.ram[1202] = 0b0111_0101;
        cpu.carry = 1;
        cpu.x = 2;
        cpu.ror7e(1200);
        assert!(cpu.ram[1202] == 0b0011_1011);
    }

    #[test]
    fn rti_opcode() {
        let mut cpu = CPU::default();

        cpu.pc = 545;
        cpu.jsr20(200);
        cpu.negative = true;
        cpu.push_to_stack(cpu.get_status());
        cpu.negative = false;
        cpu.pc = 111;
        cpu.rti40();
        assert!(cpu.pc == 544);
        assert!(cpu.negative == true);
    }

    #[test]
    fn rts_opcode() {
        let mut cpu = CPU::default();

        cpu.pc = 200;
        cpu.jsr20(1000);
        cpu.rts60();
        assert!(cpu.pc == 200 - 1);
    }

    #[test]
    fn sbc_opcodes() {
        let mut cpu = CPU::default();
        cpu.a = 200;
        cpu.carry = 1;
        cpu.sbce9(20);
        assert!(cpu.a == 180);

        cpu.a = 200;
        cpu.carry = 0;
        cpu.sbce9(20);
        assert!(cpu.a == 179);
    }

    #[test]
    fn sec_opcode() {
        let mut cpu = CPU::default();
        assert!(cpu.carry == 0);
        cpu.sec38();
        assert!(cpu.carry == 1);
    }

    #[test]
    fn sei_opcode() {
        let mut cpu = CPU::default();
        cpu.interrupt_disable = false;
        cpu.sei78();
        assert!(cpu.interrupt_disable == true);
    }

    #[test]
    fn sta_opcodes() {
        let mut cpu = CPU::default();
        cpu.a = 123;
        cpu.sta85(100);
        assert!(cpu.ram[100] == 123);
    }

    #[test]
    fn stx_opcodes() {
        let mut cpu = CPU::default();
        cpu.x = 145;
        cpu.stx8e(1233);
        assert!(cpu.ram[1233] == 145);
    }

    #[test]
    fn sty_opcodes() {
        let mut cpu = CPU::default();
        cpu.y = 233;
        cpu.sty8c(2000);
        assert!(cpu.ram[2000] == 233);
    }

    #[test]
    fn txx_opcodes() {
        let mut cpu = CPU::default();

        cpu.a = 50;
        cpu.taxaa();
        assert!(cpu.x == cpu.a);

        cpu.a = 90;
        cpu.taya8();
        assert!(cpu.y == cpu.a);

        cpu.sp = 200;
        cpu.tsxba();
        assert!(cpu.x == cpu.sp);

        cpu.x = 155;
        cpu.txa8a();
        assert!(cpu.x == cpu.a);

        cpu.x = 111;
        cpu.txs9a();
        assert!(cpu.x == cpu.sp);

        cpu.y = 222;
        cpu.tya98();
        assert!(cpu.y == cpu.a);
    }
}
