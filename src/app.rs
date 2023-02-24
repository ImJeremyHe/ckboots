use std::collections::HashMap;

use ckb_types::packed::{CellDep, OutPoint, TransactionView};

use crate::{config::Config, contract::ContractResult};

pub struct App {
    statuses: HashMap<&'static str, Cell>,
    pending: HashMap<&'static str, Cell>,
    contracts: HashMap<&'static str, CellDep>,
    config: Config,
}

impl App {
    pub fn bootstrap() -> Self {
        todo!()
    }

    pub fn build_tx(&self, res: ContractResult) -> (TransactionView, HashMap<&'static str, Cell>) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    outpoint: OutPoint,
    data: Vec<u8>,
}
