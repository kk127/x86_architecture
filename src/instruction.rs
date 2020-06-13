use crate::emulator::{Emulator}; 
use crate::modrm::*;
use crate::emulator_function::*;

use std::process;
use num_traits::FromPrimitive;
use log::error;


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

pub fn mov_rm32_imm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::new();
    parse_modrm(emu, &mut modrm);
    let value: u32 = get_code32(emu, 0);
    emu.eip += 4;
    set_rm32(emu, &modrm, value);
}

pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::new();
    parse_modrm(emu, &mut modrm);
    let r32: u32 = get_r32(emu, &modrm);
    set_rm32(emu, &modrm, r32);
}

pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::new();
    parse_modrm(emu, &mut modrm);
    let rm32: u32 = get_rm32(emu, &modrm);
    set_r32(emu, &modrm, rm32);
}

pub fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1);
    emu.eip = (emu.eip as i8 + diff + 2) as usize;
}

pub fn near_jump(emu: &mut Emulator) {
    let diff: i32 = get_sign_code32(emu, 1);
    emu.eip = (emu.eip as i32 + diff + 5) as usize;
}

pub fn add_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::new();
    parse_modrm(emu, &mut modrm);
    let r32: u32 = get_r32(emu, &modrm);
    let rm32: u32 = get_rm32(emu, &modrm);
    set_rm32(emu, &modrm, rm32 + r32);
}

pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM) {
    let rm32: u32 = get_rm32(emu, &modrm);
    let imm8: i8 = get_sign_code8(emu, 0);
    emu.eip += 1;
    set_rm32(emu, modrm, (rm32 as i64 - imm8 as i64) as u32);
}

pub fn code_83(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::new();
    parse_modrm(emu, &mut modrm);

    match modrm.op_reg {
        5 => sub_rm32_imm8(emu, &mut modrm),
        _ => {
            error!("not implemented: 83 {:?}", modrm.op_reg);
            process::exit(1);
        }
    }
}

pub fn inc_rm32(emu: &mut Emulator, modrm: &mut ModRM) {
    let value: u32 = get_rm32(emu, modrm);
    set_rm32(emu, modrm, value + 1);
}

pub fn code_ff(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::new();
    parse_modrm(emu, &mut modrm);

    match modrm.op_reg {
        0 => inc_rm32(emu, &mut modrm),
        _ => {
            error!("not implemented: FF {:?}", modrm.op_reg);
            process::exit(1)
        }
    }
}