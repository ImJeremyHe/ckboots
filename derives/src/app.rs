use quote::quote;
use syn::Meta::List;
use syn::{Attribute, NestedMeta};
use syn::{DeriveInput, Path};

pub fn get_app_impl_block(input: DeriveInput) -> proc_macro2::TokenStream {
    let container = Container::from_attrs(input.attrs);
    let contract_exec_branches = container.contracts.iter().map(|contract| {
        quote! {
            if id == <#contract as ckboots::OnChain>::_id().expect("should have an id") {
                let ids = #contract::_get_args_ids();
                let args = ids.into_iter().map(|id| {
                    if id == "user_input" {
                        &user_input
                    } else {
                        let data = self._manager.get_by_id(id).expect(&format!("cannot find the {}, perhaps it is not registered", id));
                        data
                    }
                }).collect();
                let exec = #contract::new(args);
                return exec.run();
            }
        }
    });
    let ident = input.ident;

    let generate_contracts = if cfg!(feature = "contract-generator") {
        let type_str = container.types.iter().map(|p| {
            quote! {
                #p::__get_code_str__()
            }
        });
        let types = quote! {
            let type_strs = vec![
                #(#type_str),*
            ];
            ckboots::generators::contract::write_types("proj-name-contracts/contracts", type_strs);
        };

        let contract = container.contracts.iter().map(|c| {
            quote! {
                let _id = #c::_id();
                let _code = #c::generate_contract();
                ckboots::generators::contract::write_contract("proj-name-contracts/contracts", _code, _id);

            }
        });

        let capsule = {
            let ids = container.contracts.iter().map(|c| {
                quote! {
                    let _id = #c::_id();
                    _ids.push(_id);
                }
            });
            quote! {
                let mut _ids: Vec<&'static str> = Vec::new();
                #(#ids)*
                ckboots::generators::contract::write_capsule("proj-name", &_ids);
            }
        };
        let entry = {
            let ids = container.types.iter().map(|p| {
                quote! {
                    <#p as ckboots::OnChain>::_id()
                }
            });
            let ids_idx = 0..container.types.len();
            let type_paths = container.types.iter().map(|p| quote! {#p});
            quote! {
                let mut _type_ids_idx:Vec<(&'static str, usize)> = vec![
                    #((#ids.unwrap(), #ids_idx)),*
                ];
                _type_ids_idx.sort_by_key(|k| k.0);
                let _type_path: Vec<String> = vec![
                    #(ckboots::quote!{#type_paths}.to_string()),*
                ];
                let data = _type_ids_idx.into_iter().map(|(id, idx)| {
                    let p = _type_path.get(idx).unwrap().to_string();
                    (id.to_string(), p)
                }).collect::<Vec<(String, String)>>();
                ckboots::generators::contract::write_entry("proj-name-contracts/contracts", &data);
            }
        };
        quote! {
            #[test]
            pub fn generate_contracts() {
                use ckboots::__CodeStr__;
                #capsule
                #types
                #entry
                #(#contract)*
            }

        }
    } else {
        quote! {}
    };

    quote! {

        impl #ident {
            pub fn _exec<T: ckboots::OnChain>(&self, id: &str, user_input: T) -> ContractResult
            {
                let user_input = user_input._to_bytes();
                #(#contract_exec_branches)*
                panic!("could not match any contract id for {:?}", id)
            }
        }

        #generate_contracts


    }
}

struct Container {
    pub types: Vec<Path>,
    pub contracts: Vec<Path>,
}

impl Container {
    pub fn from_attrs(attrs: Vec<Attribute>) -> Self {
        let mut contracts: Vec<Path> = vec![];
        let mut types: Vec<Path> = vec![];

        attrs
            .iter()
            .flat_map(|attr| {
                if !attr.path.is_ident("app") {
                    return Err(());
                }
                match attr.parse_meta() {
                    Ok(List(meta)) => Ok(meta.nested.into_iter().collect::<Vec<_>>()),
                    _ => Err(()),
                }
            })
            .flatten()
            .for_each(|meta| match meta {
                NestedMeta::Meta(List(m)) => {
                    m.nested.into_iter().for_each(|e| match e {
                        NestedMeta::Meta(entry) => match entry {
                            syn::Meta::Path(p) => {
                                if m.path.is_ident("contracts") {
                                    contracts.push(p);
                                } else if m.path.is_ident("types") {
                                    types.push(p);
                                }
                            }
                            _ => panic!("required: #[app(contracts(...), types(...))]"),
                        },
                        NestedMeta::Lit(_) => todo!(),
                    });
                }
                _ => todo!(),
            });
        Container { types, contracts }
    }
}
