use proc_macro2::Ident;
use quote::quote;
use syn::{AttributeArgs, ItemFn, LitStr, ReturnType, Signature, Type, TypePath};

pub fn build_contract_entry(attr: &AttributeArgs, func: &ItemFn) -> proc_macro2::TokenStream {
    let descriptor = ContractDescriptor::from_ast(attr, func);

    let entry = descriptor.attrs.entry;

    let type_iter = descriptor.args.iter().map(|e| match e.1 {
        SigArg::MutRef(p) => p,
        SigArg::UnmutRef(p) => p,
        SigArg::Value(p) => p,
    });

    let get_args_ids_func = {
        let iter = type_iter.clone();

        quote! {
            pub fn _get_args_ids() -> Vec<&'static str> {
                vec![
                    #(if let Some(id) = <#iter as ckboots::OnChain>::_id() {id} else {"user_input"}),*
                ]
            }
        }
    };

    let new_func = {
        let (idents, decodes) = descriptor.args.iter().enumerate().fold(
            (vec![], vec![]),
            |(mut prev_idents, mut prev_decodes), (idx, (ident, arg))| {
                let idents = quote! {
                    #ident,
                };
                let ty = arg.get_type_path();
                let decodes = quote! {
                    let _bytes = bytes.get(#idx).unwrap();
                    let (#ident, _) = ckboots::consume_and_decode::<#ty>(_bytes).unwrap();
                };
                prev_idents.push(idents);
                prev_decodes.push(decodes);
                (prev_idents, prev_decodes)
            },
        );
        quote! {
                pub fn new(bytes: Vec<&[u8]>) -> Self {
                    #(#decodes)*
                    Self {
                        #(#idents)*
                    }
                }
        }
    };

    let contract_id = descriptor.attrs.id;
    let run_func = {
        let func_block = func.block.as_ref();
        let init_branches = descriptor.args.iter().map(|(ident, arg)| match arg {
            SigArg::MutRef(p) => quote! {
                let _input_id = <#p as ckboots::OnChain>::_id().unwrap();
                let _input_data = <#p as ckboots::OnChain>::_to_bytes(&self.#ident);
                _inputs.push((_input_id, _input_data));
                let #ident = &mut self.#ident;
            },
            SigArg::UnmutRef(p) => quote! {
                let _dep_id = <#p as ckboots::OnChain>::_id().unwrap();
                _dep_ids.push(_dep_id);
                let #ident = &self.#ident.unwrap();
            },
            SigArg::Value(p) => {
                quote! {
                    let _user_input_data = <#p as ckboots::OnChain>::_to_bytes(&self.#ident);
                    _user_input = Some(_user_input_data);
                }
            }
        });
        let get_output_branch = descriptor.args.iter().map(|(ident, arg)| match arg {
            SigArg::MutRef(p) => {
                quote! {
                    let _data = <#p as ckboots::OnChain>::_to_bytes(#ident);
                    _outputs.push(_data);
                }
            }
            _ => quote! {},
        });
        quote! {
            pub fn run(mut self) -> ckboots::ContractResult {
                let mut _inputs: Vec<(&'static str, Vec<u8>)> = vec![];
                let mut _dep_ids: Vec<&'static str> = vec![];
                let _user_input: Option<Vec<u8>> = None;

                #(#init_branches)*

                #func_block

                let mut _outputs: Vec<Vec<u8>> = vec![];

                #(#get_output_branch)*

                let _input_output_data = _inputs.into_iter().zip(_outputs.into_iter()).map(|((v0, v1), v2)| {
                    (v0, v1, v2)
                }).collect();

                ckboots::ContractResult {
                    deps: _dep_ids,
                    user_input: _user_input,
                    contract_id: #contract_id,
                    input_output_data: _input_output_data,
                }
            }
        }
    };

    let ident_iter = descriptor.args.iter().map(|e| e.0);

    let id_func = quote! {
        pub fn _id() -> &'static str {
            #contract_id
        }
    };

    quote! {
        #[derive(ckboots_derives::OnChain)]
        pub struct #entry {
            #(#ident_iter: #type_iter,)*
        }

        impl #entry {

            #new_func

            #get_args_ids_func

            #id_func

            #run_func
        }
    }
}

struct ContractDescriptor<'a> {
    pub attrs: Attrs<'a>,
    pub args: Vec<(&'a Ident, SigArg<'a>)>,
}

impl<'a> ContractDescriptor<'a> {
    pub fn from_ast(attrs: &'a AttributeArgs, func: &'a ItemFn) -> Self {
        let attrs = parse_attrs(attrs);
        let args = parse_signature(&func.sig);

        ContractDescriptor { attrs, args }
    }
}

struct Attrs<'a> {
    pub entry: &'a Ident,
    pub id: &'a LitStr,
}

fn parse_signature(sig: &Signature) -> Vec<(&Ident, SigArg)> {
    check_signature(&sig);

    let inputs = &sig.inputs;
    let mut result: Vec<(&Ident, SigArg)> = Vec::with_capacity(inputs.len());
    inputs.iter().for_each(|fn_arg| match fn_arg {
        syn::FnArg::Receiver(_) => panic!("'self' is not allowed in the contract function"),
        syn::FnArg::Typed(arg) => {
            let ty = arg.ty.as_ref();
            let ident = match arg.pat.as_ref() {
                syn::Pat::Ident(pat_ident) => {
                    if pat_ident.by_ref.is_some() {
                        panic!("unexpected ref");
                    }
                    if pat_ident.mutability.is_some() {
                        panic!("unexpected mut")
                    }
                    &pat_ident.ident
                }
                _ => panic!("unexpected pat type"),
            };
            let arg = parse_sig_type(ty);
            result.push((ident, arg));
        }
    });

    result
}

fn check_signature(sig: &Signature) {
    if !matches!(&sig.output, ReturnType::Default) {
        panic!("contract function should has no return type")
    }

    if sig.asyncness.is_some() {
        panic!("async is not allowed in the contract functions")
    }

    if sig.generics.params.len() > 0 {
        unimplemented!("generic is not supported in the contract functions");
    }

    if sig.abi.is_some() {
        panic!("abi is not allowed in the contract functions")
    }

    if sig.variadic.is_some() {
        panic!("variadic is not allowed in the contract functions")
    }
}

fn parse_attrs(attrs: &AttributeArgs) -> Attrs {
    let mut entry: Option<&Ident> = None;
    let mut id: Option<&LitStr> = None;
    attrs.iter().for_each(|meta| match meta {
        syn::NestedMeta::Meta(m) => match m {
            syn::Meta::NameValue(value) => {
                if value.path.is_ident("id") {
                    match &value.lit {
                        syn::Lit::Str(l) => id = Some(l),
                        _ => panic!("use literal string instead"),
                    }
                } else if value.path.is_ident("entry") {
                }
            }
            syn::Meta::Path(i) => {
                let ident = i.get_ident().expect("illegal path");
                entry = Some(ident);
            }
            _ => panic!("unexpected attr"),
        },
        syn::NestedMeta::Lit(_) => panic!("unexpected literal"),
    });
    Attrs {
        entry: entry.expect("missing entry"),
        id: id.expect("missing id"),
    }
}

enum SigArg<'a> {
    MutRef(&'a TypePath),
    UnmutRef(&'a TypePath),
    Value(&'a TypePath),
}

impl<'a> SigArg<'a> {
    pub fn get_type_path(&self) -> &'a TypePath {
        match self {
            SigArg::MutRef(t) => t,
            SigArg::UnmutRef(t) => t,
            SigArg::Value(t) => t,
        }
    }
}

fn parse_sig_type(ty: &Type) -> SigArg {
    match ty {
        Type::Path(p) => SigArg::Value(p),
        Type::Reference(r) => {
            let t = r.elem.as_ref();
            let p = match t {
                Type::Path(p) => p,
                _ => panic!("type path is expected"),
            };
            if r.mutability.is_some() {
                SigArg::MutRef(p)
            } else {
                SigArg::UnmutRef(p)
            }
        }
        _ => todo!(),
    }
}
