extern crate i8080_core;
use i8080_core::CPU;
pub struct SI{
    cpu:CPU,
    mem:[u8;0x4000]
}
impl SI{
    pub fn new()->SI{
        SI{
            
        }
    }
}