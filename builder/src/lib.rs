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
    // named fids syn REP of stru
    let fids = named_fields(&data);
    let fids_tys: HashMap<&Ident, &syn::Type> = HashMap::new();
    for fid in fids.named.iter() {
        if let syn::Type::Path(ty_path) = &fid.ty {
            &ty_path.path.segments.iter();
        };
    }

    unimplemented!()
}

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
