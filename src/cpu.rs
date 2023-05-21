use std::mem;

pub struct CPU {
    ram: [u8; 2048],
    /// program counter
    pc: u16,
    /// stack pointer
    sp: [u8; 256],
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
    b: bool,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            ram: [0; 2048],
            pc: 0,
            sp: [0; 256],
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
        }
    }
}

impl CPU {
    pub fn load_roam(filename: &str) {}

    pub fn cycle(&mut self) {}

    /// ADC #$44
    fn adc69(&mut self, operand: u8) {
        let (inum, overflowed1) = unsafe {
            mem::transmute::<u8, i8>(operand).overflowing_add(mem::transmute::<u8, i8>(self.carry))
        };
        let (_, overflowed2) = unsafe { inum.overflowing_add(mem::transmute::<u8, i8>(self.a)) };

        let (num, carried1) = operand.overflowing_add(self.carry);
        let (res, carried2) = num.overflowing_add(self.a);

        self.a = res;

        if carried1 || carried2 {
            self.carry = 1;
        } else {
            self.carry = 0;
        }

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
        let operand = self.ram[(operand + self.x) as usize];
        self.adc69(operand);
    }

    /// ADC $4400
    fn adc6d(&mut self, operand: u16) {
        let operand = self.ram[operand as usize];
        self.adc69(operand);
    }

    /// ADC $4400, X
    fn adc7d(&mut self, operand: u16) {
        let operand = self.ram[(operand + self.x as u16) as usize];
        self.adc69(operand);
    }

    /// ADC $4400,Y
    fn adc79(&mut self, operand: u16) {
        let operand = self.ram[(operand + self.y as u16) as usize];
        self.adc69(operand);
    }

    /// ADC ($44,X)
    fn adc61(&mut self, operand: u8) {
        let addr: u16 = self.ram[(operand + self.x) as usize] as u16
            | (self.ram[(operand + self.x + 1) as usize] as u16) << 8;
        let operand = self.ram[addr as usize];
        self.adc69(operand);
    }

    /// ADC ($44),Y
    fn adc71(&mut self, operand: u8) {
        let addr: u16 =
            self.ram[(operand) as usize] as u16 | (self.ram[(operand + 1) as usize] as u16) << 8;
        let operand = self.ram[(addr + self.y as u16) as usize];
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
        let operand = self.ram[(operand + self.x) as usize];
        self.and29(operand);
    }

    /// AND $4400
    fn and2d(&mut self, operand: u16) {
        let operand = self.ram[operand as usize];
        self.and29(operand);
    }

    /// AND $4400,X
    fn and3d(&mut self, operand: u16) {
        let operand = self.ram[(operand + self.x as u16) as usize];
        self.and29(operand);
    }

    /// AND $4400,Y
    fn and39(&mut self, operand: u16) {
        let operand = self.ram[(operand + self.y as u16) as usize];
        self.and29(operand);
    }

    /// AND ($44,X)
    fn and21(&mut self, operand: u8) {
        let addr: u16 = self.ram[(operand + self.x) as usize] as u16
            | (self.ram[(operand + self.x + 1) as usize] as u16) << 8;
        let operand = self.ram[addr as usize];
        self.and29(operand);
    }

    /// AND ($44),Y
    fn and31(&mut self, operand: u8) {
        let addr: u16 =
            self.ram[operand as usize] as u16 | (self.ram[(operand + 1) as usize] as u16) << 8;
        let operand = self.ram[(addr + self.y as u16) as usize];
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
        self.asl06(operand + self.x);
    }

    /// ASL $4400, X
    fn asl1e(&mut self, operand: u16) {
        self.asl0e(operand + self.x as u16);
    }

    /// BCC $44
    fn bcc90(&mut self, operand: u8) {
        if self.carry == 0 {
            self.pc += operand as u16;
        }
    }

    /// BCS $44
    fn bcsb0(&mut self, operand: u8) {
        if self.carry == 1 {
            self.pc += operand as u16;
        }
    }

    /// BEQ $44
    fn beqf0(&mut self, operand: u8) {
        if self.zero {
            self.pc += operand as u16;
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
            self.pc += operand as u16;
        }
    }

    /// BNE $44
    fn bned0(&mut self, operand: u8) {
        if !self.zero {
            self.pc += operand as u16;
        }
    }

    /// BPL $44
    fn bpl10(&mut self, operand: u8) {
        if !self.negative {
            self.pc += operand as u16
        }
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
}
