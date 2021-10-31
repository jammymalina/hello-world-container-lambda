use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsonDisplay)]
pub fn derive_json_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let value = serde_json::json!(self);
                write!(f, "{}", value.to_string())
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
