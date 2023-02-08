mod on_chain;

use on_chain::get_on_chain_impl_block;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(OnChain, attributes(onchain))]
pub fn derive_on_chain(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    get_on_chain_impl_block(input).into()
}
