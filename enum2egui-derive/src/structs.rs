use crate::derive_trait;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, DataStruct, Fields, FieldsNamed, FieldsUnnamed};

pub fn derive_struct(name: &Ident, data: &DataStruct) -> TokenStream {
    let DataStruct { fields, .. } = data;

    match fields {
        Fields::Named(named_fields) => named_field_struct_impl(name, named_fields),
        Fields::Unnamed(unnamed_fields) => tuple_struct_impl(name, unnamed_fields),
        Fields::Unit => generate_unit_struct_impl(name),
    }
}

fn generate_unit_struct_impl(name: &Ident) -> TokenStream {
    derive_trait(
        name,
        proc_macro2::TokenStream::new(),
        proc_macro2::TokenStream::new(),
    )
}

fn tuple_struct_impl(name: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    let FieldsUnnamed { unnamed, .. } = fields;
    let (field_blocks, field_blocks_mut) = tuple_struct_field_blocks(unnamed);

    let gui = struct_ui(name, field_blocks);
    let gui_mut = struct_ui(name, field_blocks_mut);
    derive_trait(name, gui, gui_mut)
}

fn tuple_struct_field_blocks(
    named: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> (TokenStream2, TokenStream2) {
    let mut field_blocks = TokenStream2::new();
    let mut field_blocks_mut = TokenStream2::new();

    named.into_iter().enumerate().for_each(|(index, field)| {
        field_blocks.extend(unnamed_field_block(field, index));
        field_blocks_mut.extend(unnamed_field_block_mut(field, index));
    });

    (field_blocks, field_blocks_mut)
}

fn unnamed_field_label(index: usize) -> String {
    format!("field_{}", index)
}

fn unnamed_field_block(field: &syn::Field, index: usize) -> proc_macro2::TokenStream {
    let field_name = unnamed_field_label(index);
    let field_type = &field.ty;
    let index = syn::Index::from(index);

    quote_spanned! { field.span() =>
        ui.horizontal(|ui| {
            ui.label(#field_name);
            <#field_type as GuiInspect>::ui(&self.#index, ui);
        });
    }
    .to_token_stream()
}

fn unnamed_field_block_mut(field: &syn::Field, index: usize) -> proc_macro2::TokenStream {
    let field_name = unnamed_field_label(index);
    let field_type = &field.ty;
    let index = syn::Index::from(index);

    quote_spanned! { field.span() =>
        ui.horizontal(|ui| {
            ui.label(#field_name);
            <#field_type as GuiInspect>::ui_mut(&mut self.#index, ui);
        });
    }
    .to_token_stream()
}

fn named_field_struct_impl(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let FieldsNamed { named, .. } = fields;
    let (field_blocks, field_blocks_mut) = named_struct_field_blocks(named);

    let gui = struct_ui(name, field_blocks);
    let gui_mut = struct_ui(name, field_blocks_mut);
    derive_trait(name, gui, gui_mut)
}

fn named_struct_field_blocks(
    named: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> (TokenStream2, TokenStream2) {
    let mut field_blocks = TokenStream2::new();
    let mut field_blocks_mut = TokenStream2::new();

    named.into_iter().for_each(|field| {
        field_blocks.extend(named_field_block(field));
        field_blocks_mut.extend(named_field_block_mut(field));
    });

    (field_blocks, field_blocks_mut)
}

fn named_field_block(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = &field.ident;
    let field_ty = &field.ty;
    quote_spanned! { field.span() =>
        ui.horizontal(|ui| {
            ui.label(stringify!(#field_name));
            <#field_ty as GuiInspect>::ui(&self.#field_name, ui);
        });
    }
    .to_token_stream()
}

fn named_field_block_mut(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = &field.ident;
    let field_ty = &field.ty;
    quote_spanned! { field.span() =>
        ui.horizontal(|ui| {
            ui.label(stringify!(#field_name));
            <#field_ty as GuiInspect>::ui_mut(&mut self.#field_name, ui);
        });
    }
    .to_token_stream()
}

fn struct_ui(name: &Ident, fields: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(stringify!(#name));
                ui.vertical(|ui| {
                    #fields
                });
            });
        });
    }
    .to_token_stream()
}
