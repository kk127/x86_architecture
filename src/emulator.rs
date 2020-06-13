use std::fs;
use std::collections::HashMap;
use std::path::Path;
use num_derive::FromPrimitive;
use log::info;

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

        let mut memory: Vec<u8> = vec![0; eip];
        
        let mut data: Vec<u8> = match fs::read(path) {
            Ok(data) => {
                info!("ROM file was successfully read.");
                data
            }
            Err(_) => panic!("Could not read ROM file."),
        };

        memory.append(&mut data);
        let memory = memory;

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