use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Constant)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl entity::Component for #ident {
            fn get(entity: &crate::entities::entity::Entity) -> Option<Self> {
                Some(entity.get::<#ident>()?.clone())
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Variable)]
pub fn derive_variable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl entity::Component for #ident {
            fn get(entity: &crate::entities::entity::Entity) -> Option<Self> {
                Some(entity.get::<#ident>()?.clone())
            }
        }
        impl entity::Variable for #ident {
            fn set(self, entity: &mut crate::entities::entity::Entity) {
                entity.set(self)
            }
            fn remove(entity: &mut crate::entities::entity::Entity) {
                entity.remove::<#ident>()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl crate::events::event::EventTrait for #ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn dispatch(&self, dispatcher: &crate::events::dispatcher::Dispatcher, world: &mut crate::entities::entity::Entities, events: &mut crate::events::event::Events) {
                dispatcher.dispatch(self, world, events);
            }
        }
    };
    output.into()
}