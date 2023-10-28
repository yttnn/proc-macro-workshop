use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;
use proc_macro2;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    // dbg!(ast.ident);
    let ident = ast.ident;
    let builder_ident = proc_macro2::Ident::new(
        &format!("{}Builder", ident.to_string().as_str()),
        ident.span(),
    );
    quote!(
        pub struct #builder_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl #builder_ident {
            pub fn build(&mut self) -> Result<#ident, std::boxed::Box<dyn std::error::Error>> {
                if self.executable.is_none() {
                    return Err(Box::<dyn std::error::Error>::from("executable is none"));
                }
                if self.args.is_none() {
                    return Err(Box::<dyn std::error::Error>::from("args is none"));
                }
                if self.env.is_none() {
                    return Err(Box::<dyn std::error::Error>::from("env is none"));
                }
                if self.current_dir.is_none() {
                    return Err(Box::<dyn std::error::Error>::from("current_dir is none"));
                }
                Ok(#ident {
                    executable: self.executable.clone().unwrap(),
                    args: self.args.clone().unwrap(),
                    env: self.env.clone().unwrap(),
                    current_dir: self.current_dir.clone().unwrap(),
                })
            }

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
        }
    )
    .into()
}
