extern crate log;
extern crate num;

use std::env;
use std::collections::HashMap;
use log::{warn};

use x86_architecture::emulator::{Emulator};
use x86_architecture::instruction::*;
use x86_architecture::emulator_function::*;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut emu = Emulator::new(0x7c00, 0x7c00, filename.to_string());

    let mut instructions: HashMap<u8, fn(&mut Emulator)> = HashMap::new();
    instructions.insert(0x01, add_rm32_r32);

    for i in 0..8 {
        instructions.insert(0x50+i, push_r32);
    }
    
    for i in 0..8 {
        instructions.insert(0x58+i, pop_r32);
    }

    instructions.insert(0x68, push_imm32);
    instructions.insert(0x6A, push_imm8);

    instructions.insert(0x83, code_83);
    instructions.insert(0x89, mov_rm32_r32);
    instructions.insert(0x8B, mov_r32_rm32);
    for i in 0..8 {
        instructions.insert(0xB8 + i, mov_r32_imm32);
    }
    instructions.insert(0xC3, ret);
    instructions.insert(0xC7, mov_rm32_imm32);
    instructions.insert(0xC9, leave);

    instructions.insert(0xE8, call_rel32);
    instructions.insert(0xE9, near_jump);
    instructions.insert(0xEB, short_jump);
    instructions.insert(0xFF, code_ff);

    while emu.eip < 1024 * 1024 {
        let code = get_code8(&emu, 0);

        println!("EIP = {:x}, Code = {:02x}", emu.eip, code);

        let instruction = match instructions.get(&code) {
            Some(k) => k,
            None => {
                warn!("Not implimented instruction: {:x}", code);
                break;
            },
        };
        
        instruction(&mut emu);

        if emu.eip == 0x00 {
            println!("\n\nend of program\n\n");
            break;
        }
    }

    dump_registers(&emu);

}