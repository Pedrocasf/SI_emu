use crate::i8080_core;
use i8080_core::CPU;
pub struct SI{
    cpu:CPU,
    pub mem:[u8;0x4000]
}
impl SI{
    pub fn new()->SI{
        let mut si = SI{
            cpu:CPU::new(0,0),
            mem:[0;0x4000]
        }
        let rom_h = fs::read("./roms/invaders.h");
        si.mem[0..0x800].copy_from_slice(rom_h);
        let rom_g = fs::read("./roms/invaders.g");
        si.mem[0x800..0x1000].copy_from_slice(rom_g);
        let rom_f = fs::read("./roms/invaders.f");
        si.mem[0x1000..0x1800].copy_from_slice(rom_f);
        let rom_e = fs::read("./roms/invaders.e");
        si.mem[0x1800..0x2000].copy_from_slice(rom_e);
    }
    pub fn run_instr(&mut self){
        let m = &mut self.mem;
        self.cpu.next(m);
    }
}