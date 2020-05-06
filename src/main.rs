extern crate log;
extern crate num;
extern crate x86_architecture;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
use log::{error, warn, info, debug};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use num::ToPrimitive;

// const REGISTERS_COUNT: usize = 8;

#[derive(Debug, PartialEq, Eq, Hash, FromPrimitive)]
enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

#[derive(Debug)]
struct Emulator {
    registers: HashMap<Register,  usize>,
    eflag: usize,
    memory: Vec<u8>,
    eip: usize,
}

impl Emulator {
    fn new(eip: usize, esp: usize, filename: String) -> Emulator {
        let path = Path::new(&filename);
        let display = path.display();
        
        // let mut file = match File::open(&path) {
        //     Ok(file) => file,
        //     Err(e) => panic!("Could not open {}: {}", display, e.description()),
        // };

        // let mut memory: Vec<i8> = Vec::new();
        // match file.read(&mut memory) {
        //     Ok(_) => info!("ROM file was successfully read"),
        //     Err(e) => panic!("Could not read {}: {}", display, e.description()),
        // }
        
        let memory: Vec<u8> = match fs::read(path) {
            Ok(data) => {
                info!("ROM file was successfully read");
                data
            }
            Err(_) => panic!("Could not read ROM file"),
        };

        // Register initialization
        let mut registers: HashMap<Register, usize> = HashMap::new();
        registers.insert(Register::ESP, esp);

        Emulator {
            registers,
            eflag: 0,
            memory,
            eip,
        }
    }
}

fn get_code8(emu: & Emulator, index: usize) -> u32{
    info!("emu.eip: {}, index: {}", emu.eip, index);
    emu.memory[emu.eip + index] as u32
}

fn get_code32(emu: & Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;
    
    for i in 0..4 {
        info!("{}", ret);
        info!("get_code 8: {}", get_code8(emu, index + i));
        info!("i * 8: {}", i * 8);
        info!("{}", get_code8(emu, index + i) << (i * 8));
        ret |= (get_code8(emu, index + i) << (i * 8)) as u32;
        info!("ret: {}", ret);
    }
    ret
}

fn get_sign_code8(emu: &mut Emulator, index: usize) -> i8 {
    println!("emu.memory[emu.eip + index]: {}", emu.memory[emu.eip + index]);
    println!("emu.memory[emu.eip + index]: {}", x86_architecture::convert_u8_twocomplement(emu.memory[emu.eip + index]));
    x86_architecture::convert_u8_twocomplement(emu.memory[emu.eip + index])
}

fn dump_registers(emu: & Emulator) {
    for (key, value) in &emu.registers {
        println!("{:?} = {:>08x}", key, value);
    }
    println!("EIP = {:>08x}", emu.eip);
}

fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: usize = (get_code8(emu, 0) - 0xB8) as usize;
    let value: usize = get_code32(emu, 1) as usize;

    emu.registers.insert(FromPrimitive::from_usize(reg).unwrap(), value);
    emu.eip += 5;
}


fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1);
    println!("diff: {}", diff);
    println!("diff +2 as usize: {}", diff + 2);
    emu.eip = (emu.eip as i8 + diff + 2) as usize;
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    // println!("{:?}", args);
    // println!("{}", filename);

    let mut emu = Emulator::new(0x0000, 0x7c00, filename.to_string());
    // println!("{:?}", emu);
    // println!("{}", emu.memory);

    // 関数ポインタテーブルの初期化
    // let mut instructions: [fn(&mut Emulator); 256] = [mov_r32_imm32; 256];
    // instructions[0xEB] = short_jump;
    let mut instructions: HashMap<u8, fn(&mut Emulator)> = HashMap::new();
    for i in 0..8 {
        instructions.insert(0xB8 + i, mov_r32_imm32);
    }
    instructions.insert(0xEB, short_jump);

    while emu.eip < 1024 * 1024 {
        let code = get_code8(&emu, 0) as u8;

        println!("EIP = {:x}, Code = {:02x}", emu.eip, code);

        let instruction = match instructions.get(&code) {
            Some(k) => k,
            None => {
                info!("Not implimented instruction: {:x}", code);
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

    error!("test error");
    warn!("test warning ");
    info!("test info");
    debug!("test debug");
}