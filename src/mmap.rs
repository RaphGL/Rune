mod ram {
    const START: usize = 0x0;
    const END: usize = 0x07FF;

    mod zero_page {
        const START: usize = 0x0;
        const END: usize = 0x00FF;
    }

    mod stack {
        const START: usize = 0x0100;
        const END: usize = 0x01FF;
    }
}

mod cpu {
    mod nmi {
        const START: usize = 0xFFFA;
        const END: usize = 0xFFFB;
    }

    mod reset {
        const START: usize = 0xFFFC;
        const END: usize = 0xFFFD;
    }

    mod irq_brk {
        const START: usize = 0xFFFE;
        const END: usize = 0xFFFF;
    }
}

mod ppu {
    const START: usize = 0x2000;
    const END: usize = 0x2007;
}

mod apu_io_registers {
    const START: usize = 0x4000;
    const END: usize = 0x4017;
}

mod cartrige {
    const START: usize = 0x4020;
    const END: usize = 0xFFFF;
}
