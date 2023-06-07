mod cpu;
mod ines;
mod mmap;

use std::time::Instant;

fn main() {
    let mut cpu = cpu::CPU::default();
    let rom = ines::InesFile::open("./test.nes");

    const SECS_PER_CYCLE: f32 = 1.0 / 21441960.0;
    loop {
        let start = Instant::now();
        // synchronizes to clockspeed
        while SECS_PER_CYCLE > start.elapsed().as_secs_f32() {}
    }
}
