use proc_macro2::Group;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::parse;
use syn::parse::Parse;
use syn::LitStr;
use syn::Meta::List;
use syn::Meta::NameValue;
use syn::Meta::Path;
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

    let defaults = data.fields.iter().map(|f| {
        let ident = f.ident.as_ref().expect("");
        let ty = &f.ty;
        let mut tokenstream: Option<TokenStream> = None;
        f.attrs
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
                NestedMeta::Meta(NameValue(nv)) => {
                    if nv.path.is_ident("default") {
                        let ts = lit_to_tokenstream(nv.lit);
                        tokenstream = Some(ts);
                    }
                }
                _ => panic!("#[onchain(default = <value>)]"),
            });
        if let Some(ts) = tokenstream {
            quote! {
                #ident: #ts,
            }
        } else {
            quote! {
                #ident: <#ty as ckboots::OnChain>::_default(),
            }
        }
    });

    quote! {
        impl #impl_generics ckboots::OnChain for #ident #type_generics #where_clause{
            fn _capacity(&self) -> u64 {
                #(self.#field_idents._capacity())+*
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
                let left = bytes;
                #(let (#field_idents, left) = ckboots::consume_and_decode::<#field_types>(left)?;)*
                Some(Self {
                    #(#field_idents,)*
                })
            }

            fn _fixed_size() -> Option<u64> {
                let size = #(<#field_types as ckboots::OnChain>::_fixed_size()?)+*;
                Some(size)
            }

            fn _id() -> Option<&'static str> {
                #id_tokens
            }

            fn _eq(&self, other: &Self) -> bool {
                #(if !self.#field_idents._eq(&other.#field_idents) {
                    return false;
                })*
                true
            }

            fn _default() -> Self {
                Self {
                    #(#defaults)*
                }
            }
        }

        impl #impl_generics #ident #type_generics #where_clause {
            pub fn onchain_new(
                #(#field_idents: #field_types),*
            ) -> Self {
                Self {
                    #(#field_idents),*
                }
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

    let capacity_branch = variant_field_index.clone().map(|(_, f, v)| {
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

    let eq_branch = variant_field_index.clone().map(|(_, f, v)| {
        if f.is_none() {
            quote! {
                (Self::#v, Self::#v) => true
            }
        } else {
            quote! {
                (Self::#v(i1), Self::#v(i2)) => i1._eq(i2)
            }
        }
    });

    let mut default_ident: Option<&syn::Ident> = None;
    let mut default_ty: Option<&syn::Type> = None;
    let mut default_value: Option<TokenStream> = None;
    data.variants.iter().for_each(|v| {
        let ident = &v.ident;
        let f = v.fields.iter().next();
        let _ = &v
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
                NestedMeta::Meta(NameValue(nv)) => {
                    if nv.path.is_ident("default") {
                        let ts = lit_to_tokenstream(nv.lit);
                        if f.is_none() {
                            panic!("maybe #[onchain(default)]")
                        }
                        if default_ident.is_none() {
                            default_ident = Some(ident);
                            default_value = Some(ts);
                            default_ty = Some(&f.as_ref().unwrap().ty);
                        } else {
                            panic!("should only one #[onchain(default)]")
                        }
                    }
                }
                NestedMeta::Meta(Path(path)) => {
                    if path.is_ident("default") {
                        if default_ident.is_some() {
                            panic!("should only one #[onchain(default)]")
                        }
                        default_ident = Some(ident);
                        if let Some(_) = f {
                            default_ty = Some(&f.as_ref().unwrap().ty);
                        }
                    }
                }
                _ => todo!(),
            });
    });

    let default_func = {
        if default_ident.is_none() {
            panic!("should use #[onchain(default)] to specify the default variant")
        }
        let ident = default_ident.unwrap();
        let branch = match (default_ty, default_value) {
            (None, None) => quote! {
                Self::#ident
            },
            (Some(ty), None) => quote! {
                Self::#ident(<#ty as ckboots::OnChain>::_default())
            },
            (_, Some(ts)) => quote! {
                Self::#ident(#ts)
            },
        };
        quote! {
            fn _default() -> Self {
                #branch
            }
        }
    };

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

            fn _eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#eq_branch,)*
                    _ => false,
                }
            }

            #default_func
        }
    }
}

fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token(token, span))
        .collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}

pub fn parse_lit_into_expr(lit: &syn::LitStr) -> Result<syn::Expr, ()> {
    match parse_lit_str(lit) {
        Ok(r) => Ok(r),
        Err(_) => {
            let _ = format!("failed to parse path: {:?}", lit.value());
            Err(())
            // panic!("{:?}", msg)
        }
    }
}

pub fn parse_lit_str<T>(s: &syn::LitStr) -> parse::Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

fn spanned_tokens(s: &syn::LitStr) -> parse::Result<TokenStream> {
    let stream = syn::parse_str(&s.value())?;
    Ok(respan(stream, s.span()))
}

fn lit_to_tokenstream(lit: syn::Lit) -> TokenStream {
    match lit {
        syn::Lit::Str(lit_str) => {
            let expr = parse_lit_into_expr(&lit_str).expect("parse lit into expr failed");
            quote! {
                #expr
            }
        }
        syn::Lit::Int(i) => quote! {#i},
        syn::Lit::Float(f) => quote! {#f},
        syn::Lit::Bool(b) => quote!(#b),
        syn::Lit::ByteStr(_) => todo!(),
        syn::Lit::Byte(_) => todo!(),
        syn::Lit::Char(_) => todo!(),
        syn::Lit::Verbatim(_) => todo!(),
    }
}
