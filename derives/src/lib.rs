mod app;
mod contract;
mod on_chain;

use app::get_app_impl_block;
use contract::build_contract_entry;
use on_chain::get_on_chain_impl_block;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Item};

#[proc_macro_derive(OnChain, attributes(onchain))]
pub fn derive_on_chain(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    get_on_chain_impl_block(input).into()
}

#[proc_macro_derive(CkbApp, attributes(app))]
pub fn derive_app(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    get_app_impl_block(input).into()
}

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let attr = parse_macro_input!(attr as AttributeArgs);
    if let Item::Fn(func) = &item {
        let entry = build_contract_entry(&attr, func);
        quote! {
            #entry

            #[allow(dead_code)]
            #item
        }
        .into()
    } else {
        panic!("#[contract] should only be used on function items")
    }
}
