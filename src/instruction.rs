use crate::emulator::{Emulator}; 

use num_traits::FromPrimitive;

pub fn get_code8(emu: & Emulator, index: usize) -> u32{
    emu.memory[emu.eip + index] as u32
}

pub fn get_code32(emu: & Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;
    
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) << (i * 8)) as u32;
    }
    ret
}

pub fn get_sign_code8(emu: &mut Emulator, index: usize) -> i8 {
    emu.memory[emu.eip + index] as i8
}

pub fn get_sign_code32(emu: &mut Emulator, index: usize) -> i32 {
    get_code32(emu, index) as i32 
}

pub fn dump_registers(emu: & Emulator) {
    for (key, value) in &emu.registers {
        println!("{:?} = {:>08x}", key, value);
    }
    println!("EIP = {:>08x}", emu.eip);
}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: usize = (get_code8(emu, 0) - 0xB8) as usize;
    let value: usize = get_code32(emu, 1) as usize;

    emu.registers.insert(FromPrimitive::from_usize(reg).unwrap(), value);
    emu.eip += 5;
}

pub fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1);
    emu.eip = (emu.eip as i8 + diff + 2) as usize;
}

pub fn near_jump(emu: &mut Emulator) {
    let diff: i32 = get_sign_code32(emu, 1);
    emu.eip = (emu.eip as i32 + diff + 5) as usize;
}