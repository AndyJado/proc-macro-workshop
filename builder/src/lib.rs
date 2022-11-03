use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let stru = parse_macro_input!(input as DeriveInput);
    let (iden, data) = (stru.ident, stru.data);
    let fids = named_fields(&data);
    let builder_iden = quote::format_ident!("{}Builder", iden);
    let builder_fields = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        let ty = grantee_not_a(&fid.ty, "Option").1;
        quote! {#name: Option<#ty>,}
    });
    let builder_inits = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        quote! { #name: None, }
    });
    let builder_methods = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        let attrs = &fid.attrs;
        for atri in attrs {
            atri.parse_meta();
        }
        let ty = grantee_not_a(&fid.ty, "Option").1;
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    let build_with = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        let (is_a, _) = grantee_not_a(&fid.ty, "Option");
        match is_a {
            Some(_) => {
                quote! { #name: self.#name.clone(), }
            }
            None => {
                quote! { #name: self.#name.clone().expect("builder should pick this field"), }
            }
        }
    });
    let expanded = quote! {
        pub struct #builder_iden { #(#builder_fields)* }

        impl #builder_iden {
            #(#builder_methods)*

            pub fn build(&mut self) -> std::result::Result<#iden,std::boxed::Box<dyn std::error::Error>> {
                Ok(#iden { #(#build_with)* })
            }
        }

        impl #iden {
            pub fn builder() -> #builder_iden {
                #builder_iden { #(#builder_inits)* }
            }
        }
    };

    expanded.into()
}

/// ty: Option<Idk>, not_ty: "Option" -> ("Option", Idk)
fn grantee_not_a<'a>(
    ty: &syn::Type,
    not_ty: &'a str,
) -> (Option<&'a str>, proc_macro2::TokenStream) {
    if let syn::Type::Path(ty_path) = &ty {
        if let Some(last_segmn) = ty_path.path.segments.iter().rev().next() {
            if last_segmn.ident == not_ty {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) = &last_segmn.arguments
                {
                    (Some(not_ty), quote! {#args})
                } else {
                    unimplemented!("wa? inside Option<_> can be empty?")
                }
            } else {
                (None, quote! { #ty_path })
            }
        } else {
            unimplemented!("ty can be path but has no last_segment?")
        }
    } else {
        unimplemented!("wa ty can't be a ty_path?")
    }
}

/// DeriveInput.data -> {x: i32, y: u8}
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
