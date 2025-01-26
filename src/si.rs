use crate::i8080_core;
use i8080_core::cpu::CPU;
use log::error;
use std::fs;
use std::ops::{Index, IndexMut};
use std::process::Output;
use minifb::Key::S;

#[derive(Debug, Copy, Clone)]
pub struct SI{
    pub rom:[u8;0x2000],
    ram:[u8;0x2000],
    shamt:u8,
    shift_reg:u16,
    watchdog:u8,
    dummy:u8
}
impl SI{
    const WATCHDOG_ADDDR:usize = 0x06;
    const SHAMT_W_ADDR:usize = 0x02;
    const SHIFT_REG_W_ADDR:usize = 0x04;
    const SHIF_REG_R_ADDR:usize = 0x03;
    pub fn new()->SI{
        let mut si = SI{
            rom:[0;0x2000],
            ram:[0;0x2000],
            shamt:0,
            shift_reg:0,
            watchdog:0xFF,
            dummy:0x00,
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
    pub fn interrupt(&mut self, cpu:&mut CPU,number:u8){
        if cpu.interrupt_enabled{
            cpu.instruction = number;
            cpu.rst(self);
        }
    }
    pub fn get_px(&mut self, coords:u16) -> bool{
        ((self.ram[(coords as usize)>>3] >> (coords & 7)) & 1) == 1
    }
    pub fn run_instr(&mut self, cpu:&mut CPU) -> u32{
        if self.watchdog == 0{
            cpu.instruction = 0xC7;
        }
        let cycles = cpu.next(self);
        if cpu.out_strobe.0{
            cpu.out_strobe.0 = false;
            match cpu.out_strobe.1{
                2 => self.shamt = cpu.out_strobe.2 & 7,
                3 => {
                    #[cfg(feature = "log")]
                    error!("NO SOUND 3 ");
                    //panic!("NO SOUND 3");
                }
                4 => self.shift_reg = ((cpu.out_strobe.2 as u16) << 8)| (self.shift_reg >> 8),
                5 => {
                    #[cfg(feature = "log")]
                    error!("NO SOUND 5");
                    //panic!("NO SOUND 5");
                },
                6 => {
                    self.watchdog = cpu.out_strobe.2
                },
                _ => panic!("unimplemented IO {:02X}", cpu.out_strobe.1)
            }
            cpu.set_input_n(Self::SHIF_REG_R_ADDR as u8, (self.shift_reg >> (8-self.shamt)) as u8)
        }
        cycles as u32
    }

}
impl Index<u16> for SI{
    type Output = u8;
    fn index(&self, index:u16) -> &Self::Output {
        let i = index & 0x3FFF;
        match i {
            0x0000..0x2000 => &self.rom[i as usize],
            0x2000..0x4000 => &self.ram[i as usize - 0x2000],
            _ => unreachable!(),
        }

    }
}

impl IndexMut<u16> for SI{
    fn index_mut(&mut self, index:u16) -> &mut Self::Output{
        let i = index & 0x3FFF;
        match i {
            0x0000..0x2000 => {
                #[cfg(feature = "log")]
                error!("Can't write to ROM");
                //panic!("can't write to ROM");
                &mut self.dummy
            }
            0x2000..0x4000 => &mut self.ram[i as usize - 0x2000],
            _ => unreachable!(),
        }
    }
}