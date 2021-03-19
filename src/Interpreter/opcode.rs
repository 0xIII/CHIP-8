#[path="../Drivers/display.rs"]
mod display;

use super::super::Drivers::memory::{Memory, Registers, MODE};

// TODO: Rust lifetime 'a for references to memory and registers

pub struct Operations {
    SHIFTLSB: bool,
    sprite_load_address: u16
}

// Implement function new returning a new instance
impl Operations {
    
    /**
     *  @func   new()   
     * */
    pub fn new(shift_lsb: bool, sprite_addr: u16) -> Operations {
        Operations {
            SHIFTLSB: shift_lsb,
            sprite_load_address: sprite_addr
        }
    }

    /**
    *  @opcode     0NNN    Call machine code routine at address NNN
    * */
    pub fn call_machine_code(self,opc: u16) {

    }

    /**
    *  @opcode     00E0    Clear display
    * */
    pub fn clear_display(self) {
        
    }

    /**
    *  @opcode     00EE    return from subroutine
    * */
    pub fn return_from_call(self, mut pmem: Memory, mut preg: Registers) {
        // pop address from stack and restore the eip
        let address = pmem.pop();
        match address {
            Some(addr)  => preg.eip = addr-0x02,    // for jump and such subtract 0x2 as the interpreter loop will increment eip after execution
            None        => panic!("Error returning from call"),
        } 
    }

    /**
    *  @opcode     1NNN    Jump to address NNN
    * */
    pub fn jmp_address(self, opc: u16, mut preg: Registers) {
        let addr = 0xFFF & opc;
        // Implement Jump
        preg.eip = addr-0x2;
    }

    /**
    *  @opcode     2NNN   Call subroutine at NNN 
    * */
    pub fn call_subroutine(self, opc: u16, mut pmem: Memory, mut preg: Registers) {
        let addr = 0xFFF & opc;
        // call subroutine 
        // push current eip
        pmem.push(preg.eip-0x2);
        preg.eip = addr-0x2;
    }

    /**
    *  @opcode     3XNN    Skips next instruction if VX == NN
    * */
    pub fn reg_val_compare(self, opc: u16, mut preg: Registers) {
        let reg_val = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let value: u8 = (0xFF & opc) as u8;
        
        if reg_val == value {
            preg.eip += 0x2;
        }
    }
    /**
    *  @opcode     4XNN    Skips next instruction if VX != NN
    *
    *  might be called redundant, but I like to keep my flags to a minimum
    * */
    pub fn reg_val_noncompare(self, opc: u16, mut preg: Registers) {
        let reg_val = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let value: u8 = (0xFF & opc) as u8;

        if reg_val != value {
            preg.eip += 0x2; 
        }
    }

    /**
    *  @opcode     5XY0    Skips next instruction if VX == VY 
    * */
    pub fn reg_compare(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();

        if reg_x == reg_y {
            preg.eip  += 0x2;
        }
    }

    /**
    * @opcode      6XNN    Set VX to NN
    * */
    pub fn reg_set(self, opc: u16, mut preg: Registers) {
        let reg_flag = ((0xF00 & opc) >> 8) as u8;
        let value: u8 = (0xFF & opc) as u8;
        preg.rw_register(reg_flag as usize, MODE::WRITE(value));
    }

    /**
     * @opcode     7XNN     Add to VX
     * */
    pub fn reg_add(self, opc: u16, mut preg: Registers) {
        let mut temp_val = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        temp_val += (0xFF & opc) as u8;
        
        preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(temp_val));
    }

    /**
     *  @opcode     8XY0    set register value to other register value    
     * */
    pub fn reg_assign(self, opc: u16, mut preg: Registers) {
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();
        
        preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(reg_y));
    }

    /**
     *  @opcode     8XY1    set VX to VX | VY
     * */
    pub fn reg_or(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();

        preg.rw_register(((0xF00 & opc) >> 8 ) as usize, MODE::WRITE(reg_x | reg_y));
    }

    /**
     *  @opcode     8XY2    set Vx to VX & VY
     * */
    pub fn reg_and(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();

        preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(reg_x & reg_y));
    }

    /**
     *  @opcode     8XY3    set VX to VX ^ VY   
     * */
    pub fn reg_xor(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();

        preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(reg_x ^ reg_y));
    }

    /**
     *  @opcode     8XY4    set VX to VX + VY
     * */
    pub fn reg_add_reg(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();
        
        // set carry (VF = 1)
        if ((((0xF0 & reg_x) >> 4) + ((0xF0 & reg_y) >> 4)) > 0xF) || ((((0xF00 & reg_x) >> 8) + ((0xF00 & reg_y) >> 8)) > 0xF) {
            preg.rw_register(0xF as usize, MODE::WRITE(0x1));
        } else {
            preg.rw_register(0xF as usize, MODE::WRITE(0x0));
        }

        preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(reg_x + reg_y));
    }

    /**
     *  @opcode     8XY5    set VX to VX - VY 
     * */
    pub fn regx_sub_regy(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();
        
        if ((((0xF0 & reg_x) >> 4) - ((0xF0 & reg_y) >> 4)) < 0x0) || ((((0xF00 & reg_x) >> 8) - ((0xF00 & reg_y) >> 8)) < 0x0) {
            preg.rw_register(0xF as usize, MODE::WRITE(0x1));
        } else {
            preg.rw_register(0xF as usize, MODE::WRITE(0x0));
        }

        preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(reg_x-reg_y));
    }

    /**
     *  @opcode     8XY6    false) Either just shifts the value in register VX to the right by one
     *                      true ) Or it stores the LSB of VY in VF before shifting right by one
     *                      and storing in the result in VX
     * */
    pub fn lsb_shift_right(self, opc: u16, mut preg: Registers) {
        if !self.SHIFTLSB {
            let mut reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
            reg_x = &reg_x >> 1;
            preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::WRITE(reg_x));
        } else {
            let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();
            
        } 
    }
    
    /**
     *  @opcode     8XY7    Set VX to VX - VY -> on borrow set VF to 0
     */
    pub fn regy_sub_regx(self, opc: u16, mut preg: Registers) {
        let reg_x = preg.rw_register(((0xF00 & opc) >> 8) as usize, MODE::READ).unwrap();
        let reg_y = preg.rw_register(((0xF0 & opc) >> 4) as usize, MODE::READ).unwrap();

        let result = 0;
    }

    /**
     *  @opcode     8XYE    
     */
    pub fn lsb_shift_left(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     9XY0    
     */
    pub fn reg_noncompare(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     ANNN    
     */
    pub fn jmp_I(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     BNNN    
     */
    pub fn jmp_offset(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     CXNN
     */
    pub fn rand_reg(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     DXYN    
     */
    pub fn draw_sprite(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     EX9E
     */
    pub fn stored_key_pressed(self, opc: u16, mut preg: Registers) {

    }

    /**
     *  @opcode     EXA1
     */
    pub fn stored_key_notpressed(self, opc: u16) {

    }

    /**
     *  @opcode     FX07
     */
    pub fn get_delay(self, opc: u16) {

    }

    /**
     *  @opcode     FX0A
     */
    pub fn await_press(opc: u16) {

    }

    /**
     *  @opcode     FX15
     */
    pub fn set_delay_timer(opc: u16) {

    }

    /**
     *  @opcode     FX18
     */
    pub fn set_sound_timer(opc: u16) {

    }

    /**
     *  @opcode     FX1E
     */
    pub fn reg_add_I(opc: u16) {

    }

    /**
     *  @opcode     FX29
     */
    pub fn set_I_sprite_reg(opc: u16, sprite_address: u16) {

    }

    /**
     *  @opcode     FX33
     */
    pub fn store_bcd_at_I(opc: u16) {

    }

    /**
     *  @opcode     FX55
     */
    pub fn write_reg_mem(opc: u16) {

    }

    /**
     *  @opcode     FX65
     */
    pub fn read_reg_mem(opc: u16) {

    }
}
