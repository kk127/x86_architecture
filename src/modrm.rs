use crate::emulator_function::{get_code8, get_sign_code8, get_sign_code32};
use crate::emulator::Emulator;
use crate::emulator_function::*;

use num_traits::FromPrimitive;
use log::error;
use std::process;

#[derive(Debug)]
pub enum OpRegIndex {
    Opecode(u8),
    RegIndex(u8),
}

pub enum Disp8_32 {
    Disp8(i8),
    Disp32(u32),
}

pub struct ModRM {
    pub md: u8,
    pub op_reg: OpRegIndex,
    pub rm: u8,
    pub sib: u8,
    pub disp: Disp8_32,
}

impl ModRM {
    pub fn new() -> ModRM {
        ModRM {
            md: 0,
            op_reg: OpRegIndex::RegIndex(0),
            rm: 0,
            sib: 0,
            disp: Disp8_32::Disp8(0),
        }
    }
}

pub fn parse_modrm(emu: &mut Emulator, modrm: &mut ModRM) {
    let code: u8 = get_code8(emu, 0);
    
    modrm.md = (code & 0xC0) >> 6;
    modrm.op_reg = OpRegIndex::Opecode((code & 0x38) >> 3);
    modrm.rm = code & 0x07;

    emu.eip += 1;

    if modrm.md != 3 && modrm.rm == 4 {
        modrm.sib = get_code8(emu, 0);
        emu.eip += 1;
    }

    if (modrm.md == 0 && modrm.rm == 5) || modrm.md == 2 {
        // get_sign_code32はi32を返すのに書籍では,disp32はu32を格納する
        modrm.disp = Disp8_32::Disp32(get_sign_code32(emu, 0) as u32); 
        emu.eip += 4;
    } else if modrm.md == 1 {
        modrm.disp = Disp8_32::Disp8(get_sign_code8(emu, 0));
        emu.eip += 1;
    }
}

pub fn set_rm32(emu: &mut Emulator, modrm: & ModRM, value: u32) {
    if modrm.md == 3 {
        set_register32(emu, modrm.rm, value);
    } else {
        let address: u32 = calc_memory_address(emu, modrm);
        set_memory32(emu, address, value);
    }
}

pub fn set_r32(emu: &mut Emulator, modrm: &ModRM, value: u32) {
    let index = match modrm.op_reg {
        OpRegIndex::Opecode(v) => v,
        OpRegIndex::RegIndex(v) => v,
    };

    set_register32(emu, index, value);
}

pub fn get_rm32(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    if modrm.md == 3 {
        get_register32(emu, modrm.rm)
    } else {
        let address: u32 = calc_memory_address(emu, modrm);
        get_memory32(emu, address)
    }
}

pub fn get_r32(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    let index;
    if let OpRegIndex::RegIndex(v) = modrm.op_reg {
        index = v;
    } else {
        error!("get_r32 can't match modrm.op_reg `{:?}`", modrm.op_reg);
        process::exit(1);
    }

    get_register32(emu, index)
}

pub fn calc_memory_address(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    if modrm.md == 0 {
        if modrm.rm == 4 {
            error!("not implimented ModRM mod = 0, rm = 4");
            process::exit(1);
        } else if modrm.rm == 5 {
            if let Disp8_32::Disp32(disp32) = modrm.disp {
                disp32
            } else {
                error!("not matched disp32 mod = 0, rm = 4");
                process::exit(1);
            }
        } else {
            get_register32(emu, modrm.rm)
        }
    } else if modrm.md == 1 {
        if modrm.rm == 4 {
            error!("not implimented ModRM mod = 1, rm = 4");
            process::exit(1);
        } else {
            if let Disp8_32::Disp8(disp8) = modrm.disp {
                (get_register32(emu, modrm.rm) as i32 + disp8 as i32) as u32
            } else {
                error!("not matched disp8 mod = 1, rm = 4");
                process::exit(1);
            }
        }
    } else if modrm.md == 2 {
        if modrm.rm == 4 {
            error!("not implimented ModRM mod = 2, rm = 4");
            process::exit(1);
        } else {
            if let Disp8_32::Disp32(disp32) = modrm.disp {
                get_register32(emu, modrm.rm) + disp32
            } else {
                error!("not matched disp32, mod = 2, rm = 4");
                process::exit(1);
            }
        }
    } else {
        error!("not implemented ModRM mod = 3"); 
        process::exit(1);
    }
}