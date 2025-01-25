use crate::i8080_core;
use i8080_core::cpu::CPU;
use log::error;
use std::fs;
use std::ops::{Index, IndexMut};
use std::process::Output;

pub struct SI{
    cpu:CPU,
    pub rom:[u8;0x2000],
    ram:[u8;0x2000],
    shamt:u8,
    shift_reg:u16,
    watchdog:u8
}
impl SI{
    const WATCHDOG_ADDDR:usize = 0x06;
    const SHAMT_W_ADDR:usize = 0x02;
    const SHIFT_REG_W_ADDR:usize = 0x04;
    const SHIF_REG_R_ADDR:usize = 0x03;
    pub fn new()->SI{
        let mut si = SI{
            cpu:CPU::new(0,0),
            rom:[0x76;0x2000],
            ram:[0;0x2000],
            shamt:0,
            shift_reg:0,
            watchdog:0xFF
        };
        let rom_h = fs::read("./roms/invaders.h").unwrap();
        si.rom[0..0x800].copy_from_slice(&rom_h);
        let rom_g = fs::read("./roms/invaders.g").unwrap();
        si.rom[0x800..0x1000].copy_from_slice(&rom_g);
        let rom_f = fs::read("./roms/invaders.f").unwrap();
        si.rom[0x1000..0x1800].copy_from_slice(&rom_f);
        let rom_e = fs::read("./roms/invaders.e").unwrap();
        si.rom[0x1800..0x2000].copy_from_slice(&rom_e);
        si
    }
    pub fn interrupt(&mut self, number:u8){
        if self.cpu.interrupt_enabled{
            let m = &mut self.;
            self.cpu.instruction = number;
            self.cpu.rst(m);
        }
    }
    pub fn get_px(&mut self, coords:u16) -> bool{
        ((self.mem[0x2000+(coords>>3) as usize] >> (coords & 7)) & 1) == 1
    }
    pub fn run_instr(&mut self) -> u32{
        let m = &mut self.mem;
        if self.watchdog == 0{
            self.cpu.instruction = 0xC7;
        }
        self.cpu.next(m);
        if self.cpu.out_strobe.0{
            self.cpu.out_strobe.0 = false;
            match self.cpu.out_strobe.1{
                2 => self.shamt = self.cpu.out_strobe.2 & 7,
                3 => {
                    #[cfg(feature = "log")]
                    error!("NO SOUND 3 ");
                    //panic!("NO SOUND 3");
                }
                4 => self.shift_reg = ((self.cpu.out_strobe.2 as u16) << 8)| (self.shift_reg >> 8),
                5 => {
                    #[cfg(feature = "log")]
                    error!("NO SOUND 5");
                    //panic!("NO SOUND 5");
                },
                6 => {
                    self.watchdog = self.cpu.out_strobe.2
                },
                _ => panic!("unimplemented IO {:02X}", self.cpu.out_strobe.1)
            }
            self.cpu.set_input_n(Self::SHIF_REG_R_ADDR as u8, (self.shift_reg >> (8-self.shamt)) as u8)
        }
    }
}
impl Index<u16> for SI{
    type Output = u8;
    fn index(&self, index:u16) -> &Self::Output {
        match index & 0x3FFF {
            0x0000..0x2000 => &self.rom[index as usize],
            0x2000..0x4000 => &self.ram[]
        }

    }
}

}
impl IndexMut<u16, Output=u8> for SI{
    fn index_mut(&mut self, index:u16) -> &mut Output{
        match index & 0x3FFF {
            0..0x2000 => {

            }
        }
    }
}