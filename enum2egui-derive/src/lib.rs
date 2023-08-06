mod enums;
mod structs;

use enums::derive_enum;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use structs::derive_struct;
use syn::{parse_macro_input, Data, DeriveInput, Error};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(Gui)]
pub fn derive_gui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match &input.data {
        Data::Struct(data) => derive_struct(name, data),
        Data::Enum(data) => derive_enum(name, data),
        Data::Union(..) => derive_error!("enum2egui does not support unions"),
    }
}

pub(crate) fn derive_trait(
    name: &Ident,
    gui: proc_macro2::TokenStream,
    gui_mut: proc_macro2::TokenStream,
) -> TokenStream {
    quote! {
        impl enum2egui::GuiInspect for #name {
            fn ui(&self, ui: &mut egui::Ui) {
                #gui
            }

            fn ui_mut(&mut self, ui: &mut egui::Ui) {
                #gui_mut
            }
        }
    }
    .to_token_stream()
    .into()
}