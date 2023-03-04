// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    debug,
    high_level::{load_script, load_input, load_cell_type},
    ckb_types::{bytes::Bytes, prelude::*},
    constants::Source,
};
use molecule::prelude::Entity;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let mut contract_ids = vec!["c1", "c2", "c3"];
    contract_ids.sort();

    let app_name = "app_name";

    let input = load_input(0, Source::Input)?;     
    let mut blake2b = blake2b_rs::Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(input.as_slice());
    blake2b.update(app_name.as_bytes());
    let mut hash = [0; 32];
    blake2b.finalize(&mut hash);

    for (idx, id) in contract_ids.into_iter().enumerate() {
        let output = load_cell_type(idx, Source::GroupOutput)?;
        if output.is_none() {
            return Err(Error::ContractTypeScriptMissing);
        }
        let output = output.unwrap();
        output.args()
    }
    Ok(())
}

// hash equals to blake2b(input[0] | app_name)
fn check_output_contract(hash: &[u8], id: &'static str, actual: [u8; 32]) -> Result<(), Error> {
    let mut blake2b = blake2b_rs::Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(hash);
    blake2b.update(id.as_bytes());
    let mut ret = [0; 32];
    blake2b.finalize(&mut ret);

    if ret != actual {
        return Err(Error::WrongContractId);
    }

    Ok(())
}