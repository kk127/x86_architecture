use std::process;
use crate::emulator::Emulator;

use num_traits::FromPrimitive;
use log::error;

pub fn get_code8(emu: & Emulator, index: usize) -> u8{
    emu.memory[emu.eip + index]
}

pub fn get_code32(emu: & Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;
    
    for i in 0..4 {
        ret |= ((get_code8(emu, index + i) as u32) << (i * 8)) as u32;
    }
    ret
}

pub fn get_sign_code8(emu: &mut Emulator, index: usize) -> i8 {
    emu.memory[emu.eip + index] as i8
}

pub fn get_sign_code32(emu: &mut Emulator, index: usize) -> i32 {
    get_code32(emu, index) as i32 
}
pub fn set_memory8(emu: &mut Emulator, address: u32, value: u32) {
    emu.memory[address as usize] = (value & 0xff) as u8;
}

pub fn set_memory32(emu: &mut Emulator, address: u32, value: u32) {
    for i in 0..4 {
        set_memory8(emu, address + i, value >> (i * 8));
    }
}

pub fn get_register32(emu: & Emulator, index: u8) -> u32 {
    let key = match FromPrimitive::from_u8(index) {
        None => {
            error!{"index is not a key in Emulator register."};
            process::exit(1);
        },
        Some(v) => v,
    };

    match emu.registers.get(&key) {
        None => {
            error!("Can't get the value form register. key: {:?}", key);
            process::exit(1);
        }
        Some(&v) => v as u32,
    }

}

pub fn set_register32(emu: &mut Emulator, index: u8, value: u32) {
    let key = match FromPrimitive::from_u8(index) {
        None => {
            error!{"index is not a key in Emulator register."};
            process::exit(1);
        },
        Some(v) => v,
    };

    emu.registers.insert(key, value as usize);
}

pub fn get_memory8(emu: &mut Emulator, address: u32) -> u32 {
    emu.memory[address as usize] as u32
}

pub fn get_memory32(emu: &mut Emulator, address: u32) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret |= get_memory8(emu, address + i) << (8 * i);
    }

    ret
}