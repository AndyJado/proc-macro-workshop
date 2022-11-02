use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let stru = parse_macro_input!(input as DeriveInput);
    let (iden, data) = (stru.ident, stru.data);
    let fids = named_fields(&data);
    let builder_iden = quote::format_ident!("{}Builder", iden);
    let builder_fields = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        let ty = grantee_not_a(&fid.ty, "Option");
        quote! {#name: Option<#ty>,}
    });
    let builder_inits = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        quote! { #name: None, }
    });
    let builder_methods = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        let ty = grantee_not_a(&fid.ty, "Option");
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    let build_with = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        quote! { #name: self.#name.clone(), }
    });
    let expanded = quote! {
        pub struct #builder_iden { #(#builder_fields)* }

        impl #builder_iden {
            #(#builder_methods)*

            // pub fn build(&mut self) -> std::result::Result<#iden,std::boxed::Box<dyn std::error::Error>> {
            //     Ok(#iden { #(#build_with)* })
            // }
        }

        impl #iden {
            pub fn builder() -> #builder_iden {
                #builder_iden { #(#builder_inits)* }
            }
        }
    };

    expanded.into()
}

fn grantee_not_a(ty: &syn::Type, not_ty: &str) -> proc_macro2::TokenStream {
    if let syn::Type::Path(ty_path) = &ty {
        if let Some(last_segmn) = ty_path.path.segments.iter().rev().next() {
            //FIXME: how to match a Type properly?
            // quote! { std::option::Option<#> }
            if last_segmn.ident == not_ty {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) = &last_segmn.arguments
                {
                    quote! {#args}
                } else {
                    unimplemented!("Option<wa?>")
                }
            } else {
                quote! { #ty_path }
            }
        } else {
            unimplemented!("no last_segment?")
        }
    } else {
        unimplemented!("wa ty not ty_path?")
    }
}

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
