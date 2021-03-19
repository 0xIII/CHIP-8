use std::io;

const MEMSIZE: usize = 4095;                  // Size of the total memory
const STACKSIZE: usize = 12;                   // Size of the Stack


pub enum MODE<T> {
    READ,
    WRITE(T)
}

/**
 *  - CHIP-8 offers 4K aka. 4096 memory location of which each can hold 8 bits
 *  - 0x0000 -> 0x200   Font or Interpreter Data
 *  - 0x200 -> 0xEA0    Program Data
 *  - 0xEA0 -> 0xEFF    Call Stack
 *  - 0xEFF -> 0xFFF    Display refresh
 * */

#[derive(Debug)]
pub struct Memory {
    // define memory
    pub mem: [u8; MEMSIZE],
    pub call_stack: Vec<u16>
}

impl Memory {
    /**
    *  @func   new()   Create new instance of memory (overwritten with zeroes)
    * */
    pub fn new() -> Memory {
        // return an instance of memory
        Memory {
            mem: [0x00; MEMSIZE],
            call_stack: Vec::new()
        }
    }
    

    /**
     *  @func   load    Load image into memory starting from the entry
     *
     *  @param  img     image to be loaded
     *
     *  @param  offset  from memory 0x0000
     * */
    pub fn load(&mut self, img: Vec<u8>, offset: usize) -> Result<(), io::Error>{

        // failsafe
        if img.len() > MEMSIZE  {
           return Err(io::Error::new(io::ErrorKind::Other, "Invalid image size")); 
        }

        // iterate through array 
        for n in 0..img.len() {
            self.mem[n+offset] = img[n];
        }

        Ok(())
    }
    
    /**
     *  @func   rw_memory()     read or write to memory
     *
     *  @param  mem_address     memory address to read/write from
     *
     *  @param  mode            access mode WRITE includes the data to be written
     * */
    pub fn rw_memory(&mut self, mem_address: usize, mode: MODE<u8>) -> Result<u8, io::Error> {

        if mem_address > MEMSIZE {
            return Err(io::Error::new(io::ErrorKind::Other, "Memory out of bounds"));
        } 

        match mode {
            MODE::READ          => {
               Ok(self.mem[mem_address])
            },
            MODE::WRITE(value)  => {
                self.mem[mem_address] = value;
                Ok(value)
            }
        }  
    }
    
    /**
     *  @func   push    push a return address (u16) onto the stack
     *
     *  @param  ret     return address to push
     * */
    pub fn push(&mut self, ret: u16) {
        if self.call_stack.len() < STACKSIZE {
            self.call_stack.push(ret);
        }
    }
    
    /**
     *  @func   pop     pop the last value of the stack
     * */
    pub fn pop(&mut self) -> Option<u16>{
        if self.call_stack.len() > 0 {
            self.call_stack.pop()
        } else {
            None
        } 
    }

}

/**
 *  Registers:
 *  - V0 to VF (1B)
 *  - VF doubles as flag
 *  - address register I (2B)
 * */
pub struct Registers {
    pub register_array: [u8; 16],
    pub address_register: u16,

    pub eip: u16
}

impl Registers {
    /**
    *  @func   new()   Create new instance of registers (overwritten with zeroes)
    * */
    pub fn new(entry: u16) -> Registers {
        Registers {
            register_array: [0x00; 16],
            address_register: 0x0000,

            eip: entry
        }
    }
    
    /**
     *  @func   rw_register()   Read/Write to reegister
     *  
     *  @param  reg_flag        register denominator (0-F)
     *
     *  @param  mode            Access mode (read/write)
     * */
    pub fn rw_register(&mut self, register_flag: usize, mode: MODE<u8>) -> Result<u8, io::Error>{
        match mode {
            MODE::READ          => {
                Ok(self.register_array[register_flag])
            },
            MODE::WRITE(value)  => {
                self.register_array[register_flag] = value;
                Ok(value)
            },
        }
    }
}
