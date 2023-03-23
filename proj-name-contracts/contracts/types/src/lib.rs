#![no_std]
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

pub trait OnChain: Sized {
    fn _capacity(&self) -> u64;

    fn _id() -> Option<&'static str> {
        None
    }

    fn _to_bytes(&self) -> Vec<u8>;

    fn _from_bytes(bytes: &[u8]) -> Option<Self>;

    fn _fixed_size() -> Option<u64>;

    fn _eq(&self, other: &Self) -> bool;
}

macro_rules! impl_on_chain_for_builtin {
    ($t:ty, $c:literal) => {
        impl OnChain for $t {
            fn _capacity(&self) -> u64 {
                $c
            }

            fn _to_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().into()
            }

            fn _from_bytes(bytes: &[u8]) -> Option<Self> {
                let mut slice = [0u8; $c];
                for i in 0..$c {
                    slice[i] = bytes.get(i)?.clone();
                }
                Some(<$t>::from_le_bytes(slice))
            }

            fn _fixed_size() -> Option<u64> {
                Some($c)
            }

            fn _eq(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

impl_on_chain_for_builtin!(u8, 1);
impl_on_chain_for_builtin!(u16, 2);
impl_on_chain_for_builtin!(u32, 4);
impl_on_chain_for_builtin!(u64, 8);
impl_on_chain_for_builtin!(u128, 16);
impl_on_chain_for_builtin!(i8, 1);
impl_on_chain_for_builtin!(i16, 2);
impl_on_chain_for_builtin!(i32, 4);
impl_on_chain_for_builtin!(i64, 8);
impl_on_chain_for_builtin!(i128, 16);

impl<T: OnChain> OnChain for Vec<T> {
    fn _capacity(&self) -> u64 {
        let prefix = 8;
        self.iter().fold(0, |prev, item| prev + item._capacity()) + prefix
    }

    fn _to_bytes(&self) -> Vec<u8> {
        let mut total_capacity: u64 = 0;
        let mut bytes = vec![];
        self.iter().for_each(|element| {
            total_capacity += element._capacity();
            bytes.extend(element._to_bytes());
        });
        let mut res = total_capacity.to_le_bytes().to_vec();
        res.extend(bytes);
        res
    }

    fn _from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut result: Vec<T> = vec![];
        let mut bytes = bytes;
        while bytes.len() > 0 {
            let (item, left) = consume_and_decode::<T>(bytes)?;
            result.push(item);
            bytes = left;
        }
        Some(result)
    }

    fn _fixed_size() -> Option<u64> {
        None
    }

    fn _eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            let t = self.get(i).unwrap();
            let o = other.get(i).unwrap();
            if !t._eq(o) {
                return false;
            }
        }

        return true;
    }
}

// impl OnChain for String {
//     fn _capacity(&self) -> u64 {
//         self.as_bytes().len() as u64 + 8
//     }

//     fn _to_bytes(&self) -> Vec<u8> {
//         let bytes = self.as_bytes().to_vec();
//         let mut prefix = bytes.len().to_le_bytes().to_vec();
//         prefix.extend(bytes);
//         prefix
//     }

//     fn _from_bytes(bytes: &[u8]) -> Option<Self> {
//         String::from_utf8(bytes.to_vec()).ok()
//     }

//     fn _fixed_size() -> Option<u64> {
//         None
//     }

//     fn _eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

pub fn consume_and_decode<T: OnChain>(bytes: &[u8]) -> Option<(T, &[u8])> {
    if let Some(capacity) = T::_fixed_size() {
        let end = capacity as usize;
        let item = T::_from_bytes(&bytes[0..end])?;
        Some((item, &bytes[end..]))
    } else {
        let size: [u8; 8] = bytes[0..8].try_into().unwrap();
        let end = usize::from_le_bytes(size);
        let item = T::_from_bytes(&bytes[8..end + 8])?;
        Some((item, &bytes[end + 8..]))
    }
}

use ckb_std::ckb_constants::Source;
use ckb_std::high_level::load_cell_data;
use ckb_std::high_level::load_witness_args;
use ckb_std::syscalls::SysError;
use ckb_std::prelude::Entity;

pub fn load_cell_deps_data(idx: usize) -> Result<Vec<u8>, SysError> {
    load_cell_data(idx, Source::CellDep)
}

pub fn load_input_data(idx: usize) -> Result<Vec<u8>, SysError> {
    load_cell_data(idx, Source::Input)
}

pub fn load_output_data(idx: usize) -> Result<Vec<u8>, SysError> {
    load_cell_data(idx, Source::Output)
}

pub fn load_user_input() -> Result<Vec<u8>, SysError> {
    let witness_arg = load_witness_args(0, Source::Input)?;
    if let Some(b) = witness_arg.input_type().to_opt() {
        Ok(b.as_slice().to_vec())
    } else {
        Ok(vec![])
    }
}
use crate as ckboots ; pub struct Frog
{ pub physical : u8, pub traval_cnt : u8, } impl ckboots :: OnChain for Frog
{
    fn _capacity(& self) -> u64
    { self.physical._capacity() + self.traval_cnt._capacity() } fn
    _to_bytes(& self) -> Vec < u8 >
    {
        let mut result = Vec :: with_capacity(self._capacity() as usize) ;
        result.extend(< u8 as ckboots :: OnChain > ::
        _to_bytes(& self.physical)) ;
        result.extend(< u8 as ckboots :: OnChain > ::
        _to_bytes(& self.traval_cnt)) ; if let Some(_) = Frog :: _fixed_size()
        { result } else
        {
            let mut prefix : Vec < u8 > = result.len().to_le_bytes().to_vec()
            ; prefix.extend(result) ; prefix
        }
    } fn _from_bytes(bytes : & [u8]) -> Option < Self >
    {
        let left = bytes ; let(physical, left) = ckboots :: consume_and_decode
        :: < u8 > (left) ? ; let(traval_cnt, left) = ckboots ::
        consume_and_decode :: < u8 > (left) ? ;
        Some(Self { physical, traval_cnt, })
    } fn _fixed_size() -> Option < u64 >
    {
        let size = < u8 as ckboots :: OnChain > :: _fixed_size() ? + < u8 as
        ckboots :: OnChain > :: _fixed_size() ? ; Some(size)
    } fn _id() -> Option < & 'static str > { Some("frog") } fn
    _eq(& self, other : & Self) -> bool
    {
        if! self.physical._eq(& other.physical) { return false ; } if!
        self.traval_cnt._eq(& other.traval_cnt) { return false ; } true
    }
}