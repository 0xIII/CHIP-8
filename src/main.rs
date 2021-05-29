/*
* =====================================================================================
*
*       Filename:  chip8.rs
*
*    Description: CHIP-8 Emulator
*
*        Version:  1.0
*        Created:  21.02.2021 20:33:39
*       Revision:  none
*       Compiler:  gcc
*
*         Author:  kern_err
*
* =====================================================================================
*/

mod Drivers;
mod Interpreter;

use Drivers::{file_io};
use Drivers::display::*;
use Drivers::memory::{Memory, Registers};
use Interpreter::opcode::*;
use std::io;

// TODO: Add commandline input for e.g. ENTRY Address, command options, rom file

const ENTRY: u16 = 0x200;
const SPRITEENTRY: u16 = 0x200;
const SHIFTLSB: bool = true;
const IMAGE: &str = "cavern.ch8";

fn main() {

    let states = init();
    let mut mem: Memory = states.0;
    let mut reg: Registers = states.1;
    let opcodes: Operations = states.2; 

    //some filehandling
    let image = file_io::read_binary(IMAGE).unwrap();
    println!("Size: {}", file_io::filesize(IMAGE).unwrap());

    // load memory
    mem.load(image, ENTRY as usize).expect("Failed to load image");
    println!("Memory Dump: {:02x?}", mem.mem);

    // PC
    println!("EIP: {:#02x}", reg.eip);

    execute(0x0000, opcodes, mem, reg);
}

/**
 *  @func   init()     Initialize Memory, registers, opcode handler
 */
fn init() -> (Memory, Registers, Operations){
    (Memory::new(), Registers::new(ENTRY), Operations::new(SHIFTLSB, SPRITEENTRY))
}

/**
 *  @func   run()      Run the Emulator -> execute loop and timer
 */
fn run() {
    Display::run();
}

/**
*  @func   execute()   execute instruction
*
*  @param  opc         opcode of instruction
* */
fn execute(opc: u16, opcodes: Operations, pmem: Memory, preg: Registers) -> Result<(), io::Error>{
    let MSN: u8 = ((0xF000 & opc) >> 12) as u8;

    match MSN {
        0x0 => {
            match opc {
                0x00E0 => {
                    opcodes.clear_display();
                    Ok(())
                },
                0x00EE => {
                    opcodes.return_from_call(pmem, preg);
                    Ok(())
                },
                _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
            }
        },
        0x1 => {
            opcodes.jmp_address(opc, preg);
            Ok(())
        },
        0x2 => {
            opcodes.call_subroutine(opc, pmem, preg);
            Ok(())
        },
        0x3 => {
            opcodes.reg_val_compare(opc, preg);
            Ok(())
        },
        0x4 => {
            opcodes.reg_val_noncompare(opc, preg);
            Ok(())
        },
        0x5 => {
            opcodes.reg_compare(opc, preg);
            Ok(())
        },
        0x6 => {
            opcodes.reg_set(opc, preg);
            Ok(())
        },
        0x7 => {
            opcodes.reg_add(opc, preg);
            Ok(())
        },
        0x8 => {
            // least significant nibble
            let LSN: u8  = (opc & 0xF) as u8;
            
            match LSN {
                0x0 => {
                    opcodes.reg_assign(opc, preg);
                    Ok(())
                },
                0x1 => {
                    opcodes.reg_or(opc, preg);
                    Ok(())
                },

                0x2 => {
                    opcodes.reg_and(opc, preg);
                    Ok(())
                },

                0x3 => {
                    opcodes.reg_xor(opc, preg);
                    Ok(())
                },

                0x4 => {
                    opcodes.reg_add(opc, preg);
                    Ok(())
                },

                0x5 => {
                    opcodes.regx_sub_regy(opc, preg);
                    Ok(())
                },

                0x6 => {
                    opcodes.lsb_shift_right(opc, preg);
                    Ok(())
                },

                0x7 => {
                    opcodes.regy_sub_regx(opc, preg);
                    Ok(())
                },

                0xE => {
                    opcodes.lsb_shift_left(opc, preg);
                    Ok(())
                },
                _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
            }
        },
        0x9 => {
            opcodes.reg_noncompare(opc, preg);
            Ok(())
        },
        0xA => {
            opcodes.jmp_I(opc, preg);
            Ok(())
        },
        0xB => {
            opcodes.jmp_offset(opc, preg);
            Ok(())
        },
        0xC => {
            opcodes.rand_reg(opc, preg);
            Ok(())
        },
        0xD => {
            opcodes.draw_sprite(opc, preg);
            Ok(())
        },
        0xE => {
            let DLSN: u8 = (opc & 0xFF) as u8;
            match DLSN {
                0x9E => {
                    opcodes.stored_key_pressed(opc, preg);
                    Ok(())
                },

                0xA1 => {
                    opcodes.stored_key_notpressed(opc);
                    Ok(())
                },
                _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
            } 
        },
        0xF => {
            let DLSN: u8 = (opc & 0xFF) as u8;

            match DLSN {
                0x07 => {
                    Ok(())
                },
                0x0A => {
                    Ok(())
                },
                0x15 => {
                    Ok(())
                },
                0x18 => {
                    Ok(())
                },
                0x1E => {
                    Ok(())
                },
                0x29 => {
                    Ok(())
                },
                0x33 => {
                    Ok(())
                },
                0x55 => {
                    Ok(())
                },
                0x65 => {
                    Ok(())
                },
                _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
            }
        },
        _   => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
    }
}
