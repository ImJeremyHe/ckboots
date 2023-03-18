#[cfg(feature = "contract-generator")]
mod code_str;

mod app;
mod cell_manager;
mod contract;
pub mod generators;
mod on_chain;
mod prelude;
mod utils;

pub use cell_manager::CellManager;
pub use contract::ContractResult;
pub use on_chain::*;
pub use prelude::*;

#[cfg(feature = "contract-generator")]
pub use code_str::__CodeStr__;

#[macro_export]
macro_rules! create_app {
    ($app:ident {
        types: [$($t:ty),+]$(,)?
        contracts: [$($c:ty),+]$(,)?
    }) => {
        use ckboots::CellManager;
        use ckboots::ContractResult;
        use ckboots_derives::CkbApp;

        #[derive(CkbApp)]
        #[app(contracts($($c),*), types($($t),*))]
        pub struct $app {
            _manager: ckboots::CellManager,
        }
    };
}
