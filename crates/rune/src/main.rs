mod cpu;
mod mmap;

use rune_ines::InesFile;

fn main() {
    let mut cpu = cpu::CPU::default();
    let rom = InesFile::open("./test.nes");
    cpu.load_rom(rom.prg_rom);

    loop {
        cpu.cycle();
    }
}
