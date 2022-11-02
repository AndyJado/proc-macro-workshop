use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    const OPTION_SEGMENTS: &[&'static str] = &["std", "option", "Option"];
    // const VEC_SEGMENTS: &[&'static str] = &["std", "vec", "Vec"];
    let stru = parse_macro_input!(input as DeriveInput);
    let (iden, data) = (stru.ident, stru.data);
    let fids = named_fields(&data);
    let builder_iden = quote::format_ident!("{}Builder", iden);
    let builder_fields = fids.named.iter().map(|fid| {
        let name = fid.ident.as_ref().unwrap();
        let ty = &fid.ty;
        quote! {#name: }
    });
    for fid in fids.named.iter() {
        if let syn::Type::Path(ty_path) = &fid.ty {
            if let Some(last_segmn) = ty_path.path.segments.iter().rev().next() {
                //FIXME: how to match a Type properly?
                if last_segmn.ident.to_string() == "Option" {
                    // quote! { std::option::Option<#> }
                    quote! { #ty_path }
                } else {
                    quote! { Option<#ty_path> }
                }
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        };
    }

    let expanded = quote! {
        pub struct #builder_iden { #(#builder_fields)* }

        impl #builder_iden {
            #!builder_methods
            pub fn build(&mut self) -> std::result::Result<#iden,std::box::Box<dyn std::error::Error>> {
                Ok(#iden { #!builder_fields })
            }
        }

        impl #iden {
            pub fn builder() -> #builder_iden {
                #builder_iden { #!builder_init }
            }
        }
    };

    expanded.into()
}

fn grantee_a_option(ty: &syn::Type) {}

/// panic unless named struct
fn named_fields(data: &Data) -> &FieldsNamed {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(named_fields),
        ..
    }) = data
    {
        named_fields
    } else {
        unimplemented!()
    }
}

#[test]
fn quote_par_bra() {
    let quo_par = quote::quote!(std::option::Option<String>);
    let quo_bra = quote::quote! {std::option::Option<String>};
    assert_eq!(quo_bra.to_string(), quo_par.to_string());
}

macro_rules! eval {
    ($e: expr) => {
        $e
    };
}

#[test]
fn eval_hijane() {
    let a = 5;
    let ts2 = quote! {
        let a = 3
    };
    eval!(ts2);
    assert_eq!(a, 5);
}
