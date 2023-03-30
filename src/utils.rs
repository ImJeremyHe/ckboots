use ckb_hash::new_blake2b;
use ckb_types::{
    core::ScriptHashType,
    packed::{Byte32, Bytes, Script},
    prelude::{Builder, Entity, Pack},
};

// Blake2b(input_hash | app_name) to calculate the args in type script
pub fn get_onchain_type_args(input_hash: Byte32, app_name: &str) -> Bytes {
    let mut blake2b = new_blake2b();
    blake2b.update(input_hash.as_slice());
    blake2b.update(app_name.as_bytes());
    let mut result = [0; 32];
    blake2b.finalize(&mut result);
    result.as_slice().pack()
}

pub fn get_type_script_hash(code_hash: Byte32, input_hash: Byte32, app_name: &str) -> Byte32 {
    let args = get_onchain_type_args(input_hash, app_name);
    let script = Script::new_builder()
        .code_hash(code_hash)
        .args(args)
        .hash_type(ScriptHashType::Type.into())
        .build();
    script.calc_script_hash()
}
