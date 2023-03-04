use std::collections::HashMap;

use ckb_types::packed::{OutPoint, TransactionView};

use crate::contract::ContractResult;

pub struct App {}

impl App {
    pub fn bootstrap() -> Self {
        todo!()
    }

    pub fn build_tx(&self, res: ContractResult) -> (TransactionView, HashMap<&'static str, Cell>) {
        "".as_bytes();
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    outpoint: OutPoint,
    data: Vec<u8>,
}
