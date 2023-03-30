
// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};
use crate::error::Error;
use types::OnChain;

pub fn main() -> Result<(), Error> {
    




let bytes = types::load_input_data(0)?;
let wrapper = <types::OnChainWrapper as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
let input_id = wrapper.idx;
let bytes = wrapper.data;
let mut frog = <types::Frog as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;


{ frog . physical -= 1 ; frog . traval_cnt += 1 ; }


let bytes = types::load_output_data(0)?;
let wrapper = <types::OnChainWrapper as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
let output_id = wrapper.idx;
if input_id != output_id {
    return Err(crate::error::Error::TypeError);
}
let frog_output = <types::Frog as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
if !frog._eq(&frog_output) {
    return Err(crate::error::Error::NotEqual);
}


    Ok(())
}
