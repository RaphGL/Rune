mod cpu;
mod mmap;
mod ines;

fn main() {
    let mut cpu = cpu::CPU::default();
    println!("{:?}", ines::InesFile::new("./test.nes"));
}
