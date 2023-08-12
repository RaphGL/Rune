pub mod ram {
    pub const START: usize = 0x0;
    pub const END: usize = 0x07FF;

    pub mod zero_page {
        pub const START: usize = 0x0;
        pub const END: usize = 0x00FF;
    }

    pub mod stack {
        pub const START: usize = 0x0100;
        pub const END: usize = 0x01FF;
    }
}

pub mod cpu {
    pub mod nmi {
        pub const START: usize = 0xFFFA;
        pub const END: usize = 0xFFFB;
    }

    pub mod reset {
        pub const START: usize = 0xFFFC;
        pub const END: usize = 0xFFFD;
    }

    pub mod irq_brk {
        pub const START: usize = 0xFFFE;
        pub const END: usize = 0xFFFF;
    }
}

pub mod ppu {
    pub const START: usize = 0x2000;
    pub const END: usize = 0x2007;
}

pub mod apu_io_registers {
    pub const START: usize = 0x4000;
    pub const END: usize = 0x4017;
}

pub mod cartrige {
    pub const START: usize = 0x4020;
    pub const END: usize = 0xFFFF;
}
