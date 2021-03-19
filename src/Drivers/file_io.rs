/* 
 *  ===========================================================
 *  
 *     Filename:    fileIO.rs
 *  Description:    read file functions to handle bytewise read
 *                  operations
 *
 *  ===========================================================
 * */

use std::io;
use std::fs;
use std::io::prelude::*;

pub fn filesize(filename: &str) -> io::Result<u64>{
    let meta = fs::metadata(filename)?;

    if meta.len() == 0 {
        Err(io::Error::new(
                io::ErrorKind::Other, 
                "Filesize zero"))
    } else {
        Ok(meta.len())
    }
}

pub fn read_binary(filename: &str) -> io::Result<Vec<u8>> {
    let mut fhandler = fs::File::open(filename)?;
    let mut fbuffer = Vec::new();

    //for byte in fhandler.bytes() {
    //    println!("{}", byte.unwrap());
    //}
    
    fhandler.read_to_end(&mut fbuffer);

    Ok(fbuffer)
}
