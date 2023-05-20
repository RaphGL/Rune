mod status {
    pub const NEGATIVE: u8 = 0b1000_0000;
    pub const OVERFLOW: u8 = 0b0100_0000;
    pub const DECIMAL: u8 = 0b0000_1000;
    pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
    pub const ZERO: u8 = 0b0000_0010;
    pub const CARRY: u8 = 0b0000_0001;
    pub const B: u8 = 0b0011_0000;
}

struct CPUFlags(u8);

impl CPUFlags {
    fn get_negative(&self) -> u8 {
        (self.0 & status::NEGATIVE) >> 7
    }
    fn set_negative(&mut self, state: u8) {
        self.0 = (state << 7) & status::NEGATIVE;
    }

    fn get_overflow(&self) -> u8 {
        (self.0 & status::OVERFLOW) >> 6
    }
    fn set_overflow(&mut self, state: u8) {
        self.0 = (state << 6) & status::OVERFLOW;
    }

    fn get_b(&self) -> u8 {
        (self.0 & status::B) >> 4
    }
    fn set_b(&mut self, state: u8) {
        self.0 = (state << 4) & status::B;
    }

    fn get_decimal(&self) -> u8 {
        (self.0 & status::DECIMAL) >> 3
    }
    fn set_decimal(&mut self, state: u8) {
        self.0 = (state << 3) & status::DECIMAL;
    }

    fn get_interrupt(&self) -> u8 {
        (self.0 & status::INTERRUPT_DISABLE) >> 2
    }
    fn set_interrupt(&mut self, state: u8) {
        self.0 = (state << 2) & status::INTERRUPT_DISABLE;
    }

    fn get_zero(&self) -> u8 {
        (self.0 & status::ZERO) >> 1
    }
    fn set_zero(&mut self, state: u8) {
        self.0 = (state << 1) & status::ZERO;
    }

    fn get_carry(&self) -> u8 {
        self.0 & status::CARRY
    }
    fn set_carry(&mut self, state: u8) {
        self.0 = state & status::CARRY;
    }
}

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
    /// processor status
    p: CPUFlags,
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
            p: CPUFlags(0),
        }
    }
}

impl CPU {
    pub fn load_roam(filename: &str) {}

    pub fn cycle(&mut self) {}

    // ADC instructions
    fn adc69(&mut self, operand: u8) {
        let (num, carried) = operand.overflowing_add(self.p.get_carry());
        self.a += num;
        if carried {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc65(&mut self, operand: u8) {
        let (num, overflowed) = self.ram[operand as usize].overflowing_add(self.p.get_carry());
        self.a += num;
        if overflowed {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc75(&mut self, operand: u8) {
        let (num, overflowed1) = self.ram[operand as usize].overflowing_add(self.x);
        let (num, overflowed2) = num.overflowing_add(self.p.get_carry());
        self.a += num;
        if overflowed1 || overflowed2 {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc6d(&mut self, operand: u16) {
        let (num, overflowed) = self.ram[operand as usize].overflowing_add(self.p.get_carry());
        self.a += num;
        if overflowed {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc7d(&mut self, operand: u16) {
        let (num, overflowed1) = self.ram[operand as usize].overflowing_add(self.x);
        let (num, overflowed2) = num.overflowing_add(self.p.get_carry());
        self.a += num;
        if overflowed1 || overflowed2 {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc79(&mut self, operand: u16) {
        let (num, overflowed1) = self.ram[operand as usize].overflowing_add(self.y);
        let (num, overflowed2) = num.overflowing_add(self.p.get_carry());
        self.a += num;
        if overflowed1 || overflowed2 {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc61(&mut self, operand: u8) {
        let operand = operand as usize;
        let num = self.ram[operand] + self.x;
        let (num, overflowed) = self.ram[num as usize].overflowing_add(self.x);
        self.a += num;
        if overflowed {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }

    fn adc71(&mut self, operand: u8) {
        let operand = operand as usize;
        let num = self.ram[operand] & (self.ram[operand + 1] << 8);
        let (num, overflowed) = num.overflowing_add(self.y);
        self.a += num;
        if overflowed {
            self.p.set_carry(1);
        }

        if self.a == 0 {
            self.p.set_zero(1);
        }

        if (self.a >> 7) == 1 {
            self.p.set_negative(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_flags() {
        let mut p = CPU::default().p;
        assert!(p.get_carry() == 0);
        p.set_carry(1);
        assert!(p.get_carry() == 1);

        assert!(p.get_b() == 0b00);
        p.set_b(0b11);
        assert!(p.get_b() == 0b11);

        assert!(p.get_zero() == 0);
        p.set_zero(1);
        assert!(p.get_zero() == 1);

        assert!(p.get_decimal() == 0);
        p.set_decimal(1);
        assert!(p.get_decimal() == 1);

        assert!(p.get_negative() == 0);
        p.set_negative(1);
        assert!(p.get_negative() == 1);

        assert!(p.get_overflow() == 0);
        p.set_overflow(1);
        assert!(p.get_overflow() == 1);

        assert!(p.get_interrupt() == 0);
        p.set_interrupt(1);
        assert!(p.get_interrupt() == 1);
    }

    #[test]
    fn test_adc_opcodes() {
        let mut cpu = CPU::default();
        cpu.adc69(0x44);
        assert!(cpu.a == 0x44);

        cpu.ram[0x44] = 0x69;
        cpu.adc65(0x44);
        assert!(cpu.a == 0x69 + 0x44);

        cpu.x = 0x21;
        cpu.ram[0x20] = 0x40;
        cpu.a = 0;
        cpu.adc75(0x20);
        assert!(cpu.a == 0x61);

        cpu.ram[0x07ff] = 90;
        cpu.adc6d(0x07ff);
        assert!(cpu.a == 0x61 + 90);

        cpu.a = 0;
        cpu.ram[0x7fe] = 0xff;
        cpu.x = 1;
        cpu.adc7d(0x7fe);
        assert!(cpu.a == 0);
        // TODO add carry and overflow flags to adc
        assert!(cpu.p.get_carry() == 1);
    }
}
