pub fn write_contract(save_path: &str, func_code: String, id: &str) {
    let p = env!("CARGO_MANIFEST_DIR");
    let target = std::path::Path::new(p);
    let contract_dir = target.join(save_path).join(id);

    let _ = std::fs::remove_dir_all(contract_dir.clone());
    std::fs::create_dir_all(contract_dir.clone()).unwrap();

    std::fs::write(
        contract_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{id}"
version = "0.1.0"
edition = "2021"

[dependencies]
types = {{version = "*", path = "../types"}}
ckb-std = "0.10.0"
"#
        ),
    )
    .unwrap();

    std::fs::write(
        contract_dir.join("rust-toolchain.toml"),
        r#"
[toolchain]
channel = "nightly-2022-08-01"
"#
        .trim_start(),
    )
    .unwrap();

    let src_dir = contract_dir.join("src");
    std::fs::create_dir_all(src_dir.clone()).unwrap();

    std::fs::write(src_dir.join("entry.rs"), func_code).unwrap();
    std::fs::write(src_dir.join("error.rs"), get_error_code()).unwrap();
    std::fs::write(src_dir.join("main.rs"), get_main_content()).unwrap();
}

fn get_error_code() -> &'static str {
    r#"
use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    TypeError,
    Encoding,
    NotEqual,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

    "#
}

fn get_main_content() -> &'static str {
    r##"
#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

#[allow(unused_imports)]

mod entry;
mod error;

use ckb_std::default_alloc;
use core::arch::asm;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry(_argc: u64, _argv: *const *const u8) -> i8 {
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

"##
}

pub fn get_contract_code(
    cell_deps: &[(String, String)],
    inputs: &[(String, String)],
    user_input: Option<(String, String)>,
    code: String,
) -> String {
    let cell_deps = load_cell_deps(cell_deps);
    let input = load_input(inputs);
    let output = load_output(inputs);
    let user_input = load_user_input(user_input);
    let content = format!("{cell_deps}\n\n{user_input}\n\n{input}\n\n{code}\n\n{output}\n");

    let prelude = format!(
        r#"
// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{{vec, vec::Vec}};
use crate::error::Error;
use types::OnChain;

pub fn main() -> Result<(), Error> {{
    {content}
    Ok(())
}}
"#
    );

    prelude
}

fn load_cell_deps(data: &[(String, String)]) -> String {
    let string =
        data.into_iter()
            .enumerate()
            .fold("".to_string(), |mut prev, (idx, (ident, type_path))| {
                let ident = ident.trim_matches('"');
                let type_path = type_path.trim_matches('"');
                let s = format!(
                    "
let bytes = types::load_cell_deps_data({idx})?;
let wrapper = <types::OnChainWrapper as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
let bytes = wrapper.data;
let {ident} = <types::{type_path} as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
"
                );
                prev.push_str(&s);
                prev
            });
    string
}

fn load_user_input(data: Option<(String, String)>) -> String {
    if data.is_none() {
        return String::from("");
    }
    let data = data.unwrap();
    let ident = data.0.trim_matches('"');
    let type_path = data.1.trim_matches('"');

    format!("
let bytes = types::load_user_input();
let {ident} = <types::{type_path} as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
")
}

fn load_input(data: &[(String, String)]) -> String {
    let string =
        data.into_iter()
            .enumerate()
            .fold("".to_string(), |mut prev, (idx, (ident, type_path))| {
                let ident = ident.trim_matches('"');
                let type_path = type_path.trim_matches('"');
                let s = format!(
                    "
let bytes = types::load_input_data({idx})?;
let wrapper = <types::OnChainWrapper as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
let input_id = wrapper.id;
let bytes = wrapper.data;
let mut {ident} = <types::{type_path} as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
"
                );
                prev.push_str(&s);
                prev
            });
    string
}

fn load_output(data: &[(String, String)]) -> String {
    let string =
        data.into_iter()
            .enumerate()
            .fold("".to_string(), |mut prev, (idx, (ident, type_path))| {
                let ident = ident.trim_matches('"');
                let type_path = type_path.trim_matches('"');
                let s = format!(
                    "
let bytes = types::load_output_data({idx})?;
let wrapper = <types::OnChainWrapper as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
let output_id = wrapper.id;
if input_id != output_id {{
    return Err(crate::error::Error::TypeError);
}}
let {ident}_output = <types::{type_path} as types::OnChain>::_from_bytes(&bytes).ok_or(crate::error::Error::Encoding)?;
if !{ident}._eq(&{ident}_output) {{
    return Err(crate::error::Error::NotEqual);
}}
"
                );
                prev.push_str(&s);
                prev
            });
    string
}
