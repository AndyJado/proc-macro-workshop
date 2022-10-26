use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inpu = parse_macro_input!(input as DeriveInput);
    let name = inpu.ident;
    let belly = builder(&inpu.data);
    let expaned = quote! {
        pub struct CommandBuilder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }
        impl CommandBuilder {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }
            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
            fn wa(&mut self) {
                let s = stringify!(#belly);
                println!("{:?}", s);
            }
            fn build(mut self) -> Result<#name, Box<dyn std::error::Error>> {
                let executable = self.executable.unwrap();
                let args = self.args.unwrap();
                let env = self.env.unwrap();
                let current_dir = self.current_dir.unwrap();
                //FIXME: quote!
                Ok(#name {#belly})
            }
        }
        impl #name {
            pub fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };
    proc_macro::TokenStream::from(expaned)
}

fn builder(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => {
                // FIXME: how to return unwraped?
                let recurse = fields.named.iter().map(|f| f.ident.clone().unwrap());
                quote! {
                    #(#recurse:#recurse),*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
