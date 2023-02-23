use std::collections::HashMap;

use ckb_types::packed::{CellDep, OutPoint, TransactionView};

use crate::{config::Config, contract::Contract};

pub struct App {
    pub statuses: HashMap<&'static str, Cell>,
    pub pending: HashMap<&'static str, Cell>,
    pub contracts: HashMap<&'static str, CellDep>,
    pub config: Config,
}

impl App {
    pub fn bootstrap() -> Self {
        todo!()
    }

    fn build_tx(&self, contract: Contract) -> (TransactionView, HashMap<&'static str, Cell>) {
        todo!()
    }

    pub fn exec(&mut self, contract: Contract) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    outpoint: OutPoint,
}
