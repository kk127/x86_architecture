use std::fs;
use std::path::Path;
use std::collections::HashMap;
use log::info;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub fn convert_u8_twocomplement(num: u8) -> i8 {
    let binary_num = format!("{:08x}", num);
    let str_binary = binary_num.chars().collect::<Vec<char>>();
    if str_binary[0] == '0' {
        num as i8
    } else {
        let inversed_bit = !num; 
        - ((inversed_bit + 1) as i8)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Register {
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
pub struct Emulator {
    pub registers: HashMap<Register,  usize>,
    pub eflag: usize,
    pub memory: Vec<u8>,
    pub eip: usize,
}

impl Emulator {
    pub fn new(eip: usize, esp: usize, filename: String) -> Emulator {
        let path = Path::new(&filename);
        
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
    convert_u8_twocomplement(emu.memory[emu.eip + index])
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

mod test {
    use super::*;

    #[test]
    fn test_convert_u8_twocomplement() {
        assert_eq!(convert_u8_twocomplement(  0),   0);
        assert_eq!(convert_u8_twocomplement(127),  127);
        assert_eq!(convert_u8_twocomplement(128), -128);
        assert_eq!(convert_u8_twocomplement(249),   -7);
    }
}