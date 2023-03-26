
// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};
use crate::error::Error;
use types::OnChain;

pub fn main() -> Result<(), Error> {
    let script = types::load_exec_script()?;
    if script.len() == 0 {
        
let bytes = types::load_output_data(0);
let frog_output = <types::Frog as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
let _default = <types::Frog as types::OnChain>::_default();
if !frog_output.eq(&frog_output) {
    return Err(crate::error::Error::NotEqual);
}

    } else {
        types::exec_script(&script)?;
    }
    Ok(())
}
