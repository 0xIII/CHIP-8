// use std::io;
// use super::opcode::*;

// pub struct Interpreter {
//     opcodes: Operations 
// }

// impl Interpreter {

//     /**
//     *  @func   new()       return new opcode struct
//     *
//     *  @param  memory      input memory to perform actions on
//     * */
//     pub fn new(opcodes: Operations) -> Interpreter {
//         Interpreter {
//             opcodes: Operations::new(true, 0x0000)
//         }
//     }

//     /**
//     *  @func   execute()   execute instruction
//     *
//     *  @param  opc         opcode of instruction
//     * */
//     pub fn execute(&self, opc: u16) -> Result<(), io::Error>{
//         let MSN: u8 = ((0xF000 & opc) >> 12) as u8;

//         match MSN {
//             0x0 => {
//                 match opc {
//                     0x00E0 => {
//                         self.opcodes.clear_display();
//                         Ok(())
//                     },
//                     0x00EE => {
//                         self.opcodes.return_from_call();
//                         Ok(())
//                     },
//                     _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
//                 }
//             },
//             0x1 => {
//                 self.opcodes.jmp_address(opc);
//                 Ok(())
//             },
//             0x2 => {
//                 self.opcodes.call_subroutine(opc);
//                 Ok(())
//             },
//             0x3 => {
//                 self.opcodes.reg_val_compare(opc);
//                 Ok(())
//             },
//             0x4 => {
//                 self.opcodes.reg_val_noncompare(opc);
//                 Ok(())
//             },
//             0x5 => {
//                 self.opcodes.reg_compare(opc);
//                 Ok(())
//             },
//             0x6 => {
//                 self.opcodes.reg_set(opc);
//                 Ok(())
//             },
//             0x7 => {
//                 self.opcodes.reg_add(opc);
//                 Ok(())
//             },
//             0x8 => {
//                 // least significant nibble
//                 let LSN: u8  = (opc & 0xF) as u8;
                
//                 match LSN {
//                     0x0 => {
//                         self.opcodes.reg_assign(opc);
//                         Ok(())
//                     },
//                     0x1 => {
//                         self.opcodes.reg_or(opc);
//                         Ok(())
//                     },

//                     0x2 => {
//                         self.opcodes.reg_and(opc);
//                         Ok(())
//                     },

//                     0x3 => {
//                         self.opcodes.reg_xor(opc);
//                         Ok(())
//                     },

//                     0x4 => {
//                         self.opcodes.reg_add(opc);
//                         Ok(())
//                     },

//                     0x5 => {
//                         self.opcodes.regx_sub_regy(opc);
//                         Ok(())
//                     },

//                     0x6 => {
//                         self.opcodes.lsb_shift_right(opc);
//                         Ok(())
//                     },

//                     0x7 => {
//                         self.opcodes.regy_sub_regx(opc);
//                         Ok(())
//                     },

//                     0xE => {
//                         self.opcodes.lsb_shift_left(opc);
//                         Ok(())
//                     },
//                     _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
//                 }
//             },
//             0x9 => {
//                 self.opcodes.reg_noncompare(opc);
//                 Ok(())
//             },
//             0xA => {
//                 self.opcodes.jmp_I(opc);
//                 Ok(())
//             },
//             0xB => {
//                 self.opcodes.jmp_offset(opc);
//                 Ok(())
//             },
//             0xC => {
//                 self.opcodes.rand_reg(opc);
//                 Ok(())
//             },
//             0xD => {
//                 self.opcodes.draw_sprite(opc);
//                 Ok(())
//             },
//             0xE => {
//                 let DLSN: u8 = (opc & 0xFF) as u8;
//                 match DLSN {
//                     0x9E => {
//                         self.opcodes.stored_key_pressed(opc);
//                         Ok(())
//                     },

//                     0xA1 => {
//                         self.opcodes.stored_key_notpressed(opc);
//                         Ok(())
//                     },
//                     _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
//                 } 
//             },
//             0xF => {
//                 let DLSN: u8 = (opc & 0xFF) as u8;

//                 match DLSN {
//                     0x07 => {
//                         Ok(())
//                     },
//                     0x0A => {
//                         Ok(())
//                     },
//                     0x15 => {
//                         Ok(())
//                     },
//                     0x18 => {
//                         Ok(())
//                     },
//                     0x1E => {
//                         Ok(())
//                     },
//                     0x29 => {
//                         Ok(())
//                     },
//                     0x33 => {
//                         Ok(())
//                     },
//                     0x55 => {
//                         Ok(())
//                     },
//                     0x65 => {
//                         Ok(())
//                     },
//                     _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
//                 }
//             },
//             _   => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
//         }
//     }
// }

