use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Lit, Meta, MetaList,
    MetaNameValue, NestedMeta, Path,
};

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
        // change the builder method on attribute says
        let each_iden = attrs
            .iter()
            .map(|attr| {
                attr.parse_meta().unwrap();
                todo!()
            })
            .next();
        let ty_each = grantee_not_a(&fid.ty, "Vec").1;
        let ty = grantee_not_a(&fid.ty, "Option").1;
        let field_method = quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        };
        let each_method = quote! {
            pub fn #each_iden(&mut self,#each_iden: #ty_each) -> &mut Self {
                let field = &mut self.#name;
                match field {
                    Some(v) => {
                        v.push(#each_iden);
                    },
                    None => {
                        *field = Some(vec![#each_iden]);
                    }
                }
                self
            }
        };
        // "each = env" & field name env conflict, then only generate each method.
        if name == &each_iden {
            each_method
        } else {
            match &each_iden {
                // "each = arg, field = args"
                Some(_) => {
                    quote! {
                        #each_method
                        #field_method
                    }
                }
                // with no attributes
                None => field_method,
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

macro_rules! field_each_attr_guard {
    ($b:ident, $e:ident) => {
        if b != "builder" || e != "each" {
            compile_error!("expected `builder(each = \"...\")`")
        }
    };
}

fn attr2iden(meta: Meta) {
    let Meta::List(MetaList {
        path: Path { segments, .. },
        nested,
        ..
    }) = meta else {return};
    let builder_attr_str = &segments.iter().rev().next().unwrap().ident.to_string();
    // let each_iden_qot;
    let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit,.. })) = &nested.iter().next().unwrap() else {return};
    let lit_str = lit.suffix();
    field_each_attr_guard!(builder_attr_qot, lit);
    // Meta::List(meta_list) => {
    //     //FIXME: I suppose you do rev() on a Path Ty to get the value
    //     if meta_list.path.segments.iter().rev().next().unwrap().ident == "builder" {
    //         match meta_list.nested.iter().next() {
    //             Some(ref nest_meta) => {
    //                 let &NestedMeta::Meta(Meta::NameValue(MetaNameValue{path, lit: Lit::Str(lit_str),..})) = nest_meta else {
    //                     unimplemented!("builder attribute typo?");
    //                 };
    //                 if path.segments.iter().rev().next().unwrap().ident == "each" {
    //                     // lit_str.value()
    //                     Ident::new(&lit_str.value(),lit_str.span())
    //                 } else {
    //                     //FIXME:
    //                     compile_error!("expected `builder(each = \"...\")`")
    //                 }
    //             }
    //             None => panic!("[foo(asda = as)")
    //         }
    //     } else {
    //         unimplemented!("only builder in metalist_path")
    //     }
    // }
    // _ => unimplemented!("now only [builder(bla = \"bla\")]"),
}
