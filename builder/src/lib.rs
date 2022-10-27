use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inpu = parse_macro_input!(input as DeriveInput);
    let name = inpu.ident;
    let builder_name = format_ident!("{}Builder", name);
    let belly = builder(&inpu.data);
    let builder_belly = builder_body(&inpu.data);
    let builder_field_init = builder_body_init(&inpu.data);
    let implied = member_stact(&inpu.data);
    let expaned = quote! {
        pub struct #builder_name {#builder_belly}
        impl #builder_name {
            fn wa(&mut self) {
                let s = stringify!(#belly);
                println!("{:?}", s);
            }
            #implied
            fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                let executable = self.executable.take().unwrap();
                let args = self.args.take().unwrap();
                let env = self.env.take().unwrap();
                let current_dir = self.current_dir.take().unwrap();
                Ok(#name {#belly})
            }
        }
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name { #builder_field_init }
            }
        }
    };
    proc_macro::TokenStream::from(expaned)
}

type FieldsFinger = dyn Fn(&syn::FieldsNamed) -> TokenStream;

fn struct_data(cb: &FieldsFinger, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => cb(fields),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn member_stact(data: &Data) -> TokenStream {
    let cb: Box<FieldsFinger> = Box::new(|fields| {
        let recurse = fields.named.iter().map(|f| &f.ident);
        let recurse_ty = fields.named.iter().map(|f| &f.ty);
        quote! {
            #(fn #recurse(&mut self, #recurse: #recurse_ty) -> &mut Self {
                self.#recurse = Some(#recurse);
                self
            })*
        }
    });
    struct_data(&cb, data)
}

fn builder_body(data: &Data) -> TokenStream {
    let cb: Box<FieldsFinger> = Box::new(|fields| {
        let recurse = fields.named.iter().map(|f| &f.ident);
        let recurse_ty = fields.named.iter().map(|f| &f.ty);
        quote! {
            #(#recurse : Option<#recurse_ty>),*
        }
    });
    struct_data(&cb, data)
}

fn builder_body_init(data: &Data) -> TokenStream {
    let cb: Box<FieldsFinger> = Box::new(|fields| {
        let recurse = fields.named.iter().map(|f| &f.ident);
        quote! {
            #(#recurse : None),*
        }
    });
    struct_data(&cb, data)
}

fn builder(data: &Data) -> TokenStream {
    let cb: Box<FieldsFinger> = Box::new(|fields| {
        let recurse = fields.named.iter().map(|f| &f.ident);
        quote! {
            #(#recurse),*
        }
    });
    struct_data(&cb, data)
}
