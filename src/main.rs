extern crate log;
extern crate num;

mod lib;

use std::env;
use std::collections::HashMap;
use log::{info, warn};
use lib::*;

// const REGISTERS_COUNT: usize = 8;


fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut emu = Emulator::new(0x7c00, 0x7c00, filename.to_string());

    let mut instructions: HashMap<u8, fn(&mut Emulator)> = HashMap::new();
    for i in 0..8 {
        instructions.insert(0xB8 + i, mov_r32_imm32);
    }
    instructions.insert(0xE9, near_jump);
    instructions.insert(0xEB, short_jump);

    while emu.eip < 1024 * 1024 {
        let code = get_code8(&emu, 0) as u8;

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