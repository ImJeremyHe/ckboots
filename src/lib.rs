mod app;
mod cell_manager;
mod contract;
mod on_chain;

pub use cell_manager::CellManager;
pub use contract::ContractResult;
pub use on_chain::*;

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
