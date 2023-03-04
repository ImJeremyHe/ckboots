use quote::quote;
use syn::LitStr;
use syn::Meta::List;
use syn::Meta::NameValue;
use syn::{DataEnum, DataStruct, DeriveInput, NestedMeta};

pub fn get_on_chain_impl_block(input: DeriveInput) -> proc_macro2::TokenStream {
    let mut id: Option<LitStr> = None;
    input
        .attrs
        .iter()
        .flat_map(|attr| {
            if !attr.path.is_ident("onchain") {
                return Err(());
            }
            match attr.parse_meta() {
                Ok(List(meta)) => Ok(meta.nested.into_iter().collect::<Vec<_>>()),
                _ => Err(()),
            }
        })
        .flatten()
        .for_each(|meta| match meta {
            NestedMeta::Meta(NameValue(m)) if m.path.is_ident("id") => {
                let lit = m.lit;
                match lit {
                    syn::Lit::Str(lit_str) => id = Some(lit_str),
                    _ => todo!(),
                }
            }
            _ => todo!(),
        });

    match &input.data {
        syn::Data::Struct(data) => get_struct_impl_block(&input, &data, id),
        syn::Data::Enum(data) => get_enum_impl_block(&input, &data, id),
        syn::Data::Union(_) => todo!(),
    }
}

fn get_struct_impl_block(
    input: &DeriveInput,
    data: &DataStruct,
    id: Option<LitStr>,
) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let field_types = data.fields.iter().fold(vec![], |mut prev, f| {
        let ty = &f.ty;
        prev.push(ty);
        prev
    });
    let field_idents = data.fields.iter().fold(vec![], |mut prev, i| {
        let ident = i.ident.as_ref().expect("unnamed field is not supported");
        prev.push(ident);
        prev
    });

    let id_tokens = if let Some(id) = id {
        quote! {Some(#id)}
    } else {
        quote!(None)
    };
    quote! {
        impl #impl_generics ckboots::OnChain for #ident #type_generics #where_clause{
            fn _capacity(&self) -> u64 {
                0u64 #(+ self.#field_idents._capacity())*
            }

            fn _to_bytes(&self) -> Vec<u8> {
                let mut result = Vec::with_capacity(self._capacity() as usize);
                #(result.extend(<#field_types as ckboots::OnChain>::_to_bytes(&self.#field_idents));)*
                if let Some(_) = #ident::_fixed_size() {
                    result
                } else {
                    let mut prefix: Vec<u8> = result.len().to_le_bytes().to_vec();
                    prefix.extend(result);
                    prefix
                }
            }

            fn _from_bytes(bytes: &[u8]) -> Option<Self> {
                let mut left = bytes;
                #(let (#field_idents, left) = ckboots::consume_and_decode::<#field_types>(left)?;)*
                Some(Self {
                    #(#field_idents,)*
                })
            }

            fn _fixed_size() -> Option<u64> {
                let size = 0u64 #(+ <#field_types as ckboots::OnChain>::_fixed_size()?)*;
                Some(size)
            }

            fn _id() -> Option<&'static str> {
                #id_tokens
            }

        }
    }
}

fn get_enum_impl_block(
    input: &DeriveInput,
    data: &DataEnum,
    id: Option<LitStr>,
) -> proc_macro2::TokenStream {
    if data.variants.len() == 0 {
        panic!("onchain enum should has at least 1 variant")
    }
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let id_tokens = if let Some(id) = id {
        quote! {Some(#id)}
    } else {
        quote!(None)
    };

    let variant_field_index = data.variants.iter().enumerate().map(|(i, v)| {
        if v.fields.len() > 1 {
            panic!("variant should not have more than 1 field")
        }
        let field = v.fields.iter().next();
        (i as u8, field, &v.ident)
    });

    let to_bytes_branch = variant_field_index.clone().map(|(i, f, v)| {
        if let Some(_) = f {
            quote! {
                Self::#v(s) => {
                    let mut result = <u8 as ckboots::OnChain>::_to_bytes(&#i);
                    result.extend(s._to_bytes());
                    result
                }
            }
        } else {
            quote! {
                Self::#v => <u8 as ckboots::OnChain>::_to_bytes(&#i)
            }
        }
    });

    let from_bytes_branch = variant_field_index.clone().map(|(i, f, v)| {
        if let Some(field) = f {
            let ty = &field.ty;
            quote! {
                #i => {
                    let (item, _) = ckboots::consume_and_decode::<#ty>(left)?;
                    Some(Self::#v(item))
                }
            }
        } else {
            quote! {
                #i => Some(Self::#v)
            }
        }
    });

    let capacity_branch = variant_field_index.map(|(_, f, v)| {
        if f.is_none() {
            quote! {
                Self::#v => 0
            }
        } else {
            quote! {
                Self::#v(s) => s._capacity()
            }
        }
    });

    quote! {
        impl #impl_generics ckboots::OnChain for #ident #type_generics #where_clause{
            fn _capacity(&self) -> u64 {
                let prefix = 1u64;
                let cap = match self {
                    #(#capacity_branch,)*
                    _ => 0,
                };
                cap + prefix
            }

            fn _id() -> Option<&'static str> {
                #id_tokens
            }

            fn _to_bytes(&self) -> Vec<u8> {
                let bytes = match self {
                    #(#to_bytes_branch,)*
                    _ => unreachable!(),
                };
                let mut prefix = (bytes.len() as u64).to_le_bytes().to_vec();
                prefix.extend(bytes);
                prefix
            }

            fn _from_bytes(bytes: &[u8]) -> Option<Self> {
                let (idx, left) = ckboots::consume_and_decode::<u8>(bytes)?;
                match idx {
                    #(#from_bytes_branch,)*
                    _ => unreachable!(),
                }
            }

            fn _fixed_size() -> Option<u64> {
                None
            }
        }
    }
}
