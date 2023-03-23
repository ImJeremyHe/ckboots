pub fn write_types(save_path: &str, types: Vec<&str>) {
    let mut content = get_prelude().to_string();
    let on_chain = include_str!("../../on_chain.rs");
    content.push_str(on_chain);
    content.push_str(get_utils());
    types.into_iter().for_each(|s| {
        content.push_str(s);
    });

    let p = env!("CARGO_MANIFEST_DIR");
    let target = std::path::Path::new(p);
    let types_dir = target.join(save_path).join("types");
    let _ = std::fs::remove_dir_all(types_dir.clone());
    std::fs::create_dir_all(types_dir.clone()).unwrap();

    std::fs::write(
        types_dir.join("Cargo.toml"),
        r#"
[package]
name = "types"
version = "0.1.0"
edition = "2021"

[dependencies]
ckb-std = "0.10.0"
"#
        .trim_start(),
    )
    .unwrap();

    let src_dir = types_dir.join("src");
    std::fs::create_dir_all(src_dir.clone()).unwrap();
    std::fs::write(src_dir.join("lib.rs"), content).unwrap();
}

pub fn get_utils() -> &'static str {
    r#"
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::load_cell_data;
use ckb_std::syscalls::SysError;

pub fn load_cell_deps_data(idx: usize) -> Result<Vec<u8>, SysError> {
    load_cell_data(idx, Source::CellDep)
}

pub fn load_input_data(idx: usize) -> Result<Vec<u8>, SysError> {
    load_cell_data(idx, Source::Input)
}

pub fn load_output_data(idx: usize) -> Result<Vec<u8>, SysError> {
    load_cell_data(idx, Source::Output)
}
"#
}

fn get_prelude() -> &'static str {
    r#"#![no_std]
#[allow(unused_imports)]
use core::option::Option::Some;
use core::result::Result;
use core::option::Option;
use core::option::Option::None;
use core::marker::Sized;
use core::convert::Into;
use core::convert::TryInto;
use core::clone::Clone;
use core::iter::Extend;
use core::iter::Iterator;

use alloc::vec;
use alloc::vec::Vec;

#[macro_use]
extern crate alloc;

"#
}