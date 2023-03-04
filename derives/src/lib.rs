mod app;
mod contract;
mod error;
mod on_chain;

use app::get_app_impl_block;
use contract::build_contract_entry;
use on_chain::get_on_chain_impl_block;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Item};

#[proc_macro_derive(OnChain, attributes(onchain))]
pub fn derive_on_chain(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    let on_chain = get_on_chain_impl_block(input.clone());

    let generator = if cfg!(feature = "contract-generator") {
        clean_attr(&mut input);
        let struct_code = quote! {#input};
        let impl_code = on_chain.clone();
        let code = quote! {
            use crate as ckboots;
            #struct_code
            #impl_code
        }
        .to_string();
        quote! {
            impl ckboots::__CodeStr__ for #ident {
                fn __get_code_str__() -> &'static str {
                    #code
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #on_chain
        #generator
    }
    .into()
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

#[proc_macro_derive(CkbError)]
pub fn derive_error(_: TokenStream) -> TokenStream {
    todo!()
}

fn clean_attr(input: &mut DeriveInput) {
    input.attrs = vec![];
    match &mut input.data {
        syn::Data::Struct(data) => data.fields.iter_mut().for_each(|f| f.attrs = vec![]),
        syn::Data::Enum(data) => data.variants.iter_mut().for_each(|f| f.attrs = vec![]),
        syn::Data::Union(_) => todo!(),
    };
}
