use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields};

#[proc_macro_derive(Gui)]
pub fn derive_inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            Fields::Unnamed(_) => {
                return Error::new_spanned(input.ident, "tuple structs are not supported")
                    .to_compile_error()
                    .into();
            }
            Fields::Unit => {
                return Error::new_spanned(input.ident, "unit structs are not supported")
                    .to_compile_error()
                    .into();
            }
        },
        Data::Enum(_) | Data::Union(_) => {
            return Error::new_spanned(input.ident, "only structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let struct_name = input.ident;

    let ui_impl = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_ty = &field.ty;

            quote_spanned! {field.span()=>
                ui.horizontal(|ui| {
                    ui.label(stringify!(#field_name));
                    <#field_ty as Gui>::ui(&mut self.#field_name, ui);
                });
            }
        })
        .collect::<proc_macro2::TokenStream>();

    let expanded = quote::quote! {
        impl enum2egui::Gui for #struct_name {
            fn ui(&mut self, ui: &mut egui::Ui) {
                ui.vertical(|ui| {
                    #ui_impl
                });
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
