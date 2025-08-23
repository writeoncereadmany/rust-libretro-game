use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl crate::events::event::EventTrait for #ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
    output.into()
}