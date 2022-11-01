use quote::quote;
use syn::{parse_quote, Stmt, Type};

fn main() {
    let quo_par = quote!(Option<String>);
    let quo_bra = quote! {std::option::Option<String>};
    // let aa: Stmt = parse_quote! {
    //     let a: #quo_par = Some("wo?".to_own());
    // };
    println! {"{:#?}\n{:#?}",quo_bra,quo_par};
    // println!("{:#?}", aa);
}
