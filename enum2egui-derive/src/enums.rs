use crate::{derive_trait, get_custom_label, has_skip_attr};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed, PathArguments, Type};

pub fn derive_enum(name: &Ident, data: &DataEnum) -> TokenStream {
    let DataEnum { variants, .. } = data;

    let mut selections_mut = proc_macro2::TokenStream::new();
    let mut field_match_arms = proc_macro2::TokenStream::new();

    for variant in variants.iter() {
        if has_skip_attr(&variant.attrs) {
            continue;
        }

        let variant_name = &variant.ident;

        // Pass variant attributes to get custom labels
        let selection_mut = match &variant.fields {
            Fields::Unit => unit_impl_mut(name, variant_name, &variant.attrs),
            Fields::Named(fields) => named_impl_mut(name, variant_name, fields, &variant.attrs),
            Fields::Unnamed(fields) => unnamed_impl_mut(name, variant_name, fields, &variant.attrs),
        };
        selections_mut.extend(selection_mut);

        match &variant.fields {
            Fields::Named(fields) => {
                let field = named_match_arm(name, variant_name, fields);
                field_match_arms.extend(field);
            }
            Fields::Unnamed(fields) => {
                let field = unnamed_match_arm(name, variant_name, fields);
                field_match_arms.extend(field);
            }
            Fields::Unit => {}
        };
    }

    let gui: proc_macro2::TokenStream = quote! {
        ui.label(format!("{self}"));
    }
    .to_token_stream();

    let gui_mut: proc_macro2::TokenStream = quote! {
        ui.vertical(|ui| {
            egui::ComboBox::from_id_source(ui.next_auto_id())
                .selected_text(format!("{self}"))
                .show_ui(ui, |ui| {
                    #selections_mut
                });

            match self {
                #field_match_arms
                _ => {}
            }
        });
    }
    .to_token_stream();

    derive_trait(name, gui, gui_mut)
}

fn unit_impl_mut(
    name: &Ident,
    variant_name: &Ident,
    attrs: &Vec<syn::Attribute>,
) -> proc_macro2::TokenStream {
    let label = get_custom_label(attrs).unwrap_or_else(|| variant_name.to_string());

    quote! {
        if ui
            .selectable_label(matches!(self, #name::#variant_name), #label)
            .clicked()
        {
            *self = #name::#variant_name;
        }
    }
    .to_token_stream()
}

fn named_impl_mut(
    name: &Ident,
    variant_name: &Ident,
    fields: &FieldsNamed,
    attrs: &Vec<syn::Attribute>,
) -> proc_macro2::TokenStream {
    let mut default_fields = proc_macro2::TokenStream::new();
    let label = get_custom_label(attrs).unwrap_or_else(|| variant_name.to_string());

    let FieldsNamed { named, .. } = fields;
    named.iter().for_each(|field| {
        let field_name = &field.ident;
        let default_field = quote! {
            #field_name: Default::default(),
        }
        .to_token_stream();
        default_fields.extend(default_field);
    });

    quote! {
        if ui
            .selectable_label(matches!(self, #name::#variant_name { .. }), #label)
            .clicked()
        {
            *self = #name::#variant_name { #default_fields };
        }
    }
    .to_token_stream()
}

fn unnamed_impl_mut(
    name: &Ident,
    variant_name: &Ident,
    fields: &FieldsUnnamed,
    attrs: &Vec<syn::Attribute>,
) -> proc_macro2::TokenStream {
    let mut default_fields = proc_macro2::TokenStream::new();
    let label = get_custom_label(attrs).unwrap_or_else(|| variant_name.to_string());

    let FieldsUnnamed { unnamed, .. } = fields;
    unnamed.iter().for_each(|field| {
        let field_type = &field.ty;
        let default_field: proc_macro2::TokenStream = if is_vec_type(field_type) {
            quote! {
                Vec::default(),
            }
        } else {
            quote! {
                #field_type::default(),
            }
        }
        .to_token_stream();
        default_fields.extend(default_field);
    });

    quote! {
        if ui
            .selectable_label(matches!(self, #name::#variant_name( .. )), #label)
            .clicked()
        {
            *self = #name::#variant_name(#default_fields);
        }
    }
    .to_token_stream()
}

fn named_match_arm(
    name: &Ident,
    variant_name: &Ident,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let FieldsNamed { named, .. } = fields;

    let mut match_fields = proc_macro2::TokenStream::new();
    let mut labels = proc_macro2::TokenStream::new();

    for field in named.iter() {
        let field_name = &field.ident;
        let field_entry: proc_macro2::TokenStream = quote! {
            #field_name,
        }
        .to_token_stream();
        match_fields.extend(field_entry);

        let field_type = &field.ty;
        let label = get_custom_label(&field.attrs)
            .unwrap_or_else(|| field_name.as_ref().unwrap().to_string());

        let label_block: proc_macro2::TokenStream = quote! {
            ui.horizontal(|ui| {
                ui.label(#label);
                <#field_type as GuiInspect>::ui_mut(#field_name, ui);
            });
        }
        .to_token_stream();
        labels.extend(label_block);
    }

    quote! {
        #name::#variant_name { #match_fields } => {
            ui.vertical(|ui| {
                #labels
            });
        }
    }
    .to_token_stream()
}

fn unnamed_match_arm(
    name: &Ident,
    variant_name: &Ident,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let FieldsUnnamed { unnamed, .. } = fields;

    let mut match_fields = proc_macro2::TokenStream::new();
    let mut labels = proc_macro2::TokenStream::new();

    for (index, field) in unnamed.iter().enumerate() {
        let field_name = Ident::new(&format!("field_{index}"), Span::call_site());
        let field_entry: proc_macro2::TokenStream = quote! {
            #field_name,
        }
        .to_token_stream();
        match_fields.extend(field_entry);

        let field_type = &field.ty;
        let label = get_custom_label(&field.attrs).unwrap_or_else(|| format!("field_{}", index));

        let label_block: proc_macro2::TokenStream = quote! {
            ui.horizontal(|ui| {
                ui.label(#label);
                <#field_type as GuiInspect>::ui_mut(#field_name, ui);
            });
        }
        .to_token_stream();
        labels.extend(label_block);
    }

    quote! {
        #name::#variant_name(#match_fields) => {
            #labels
        }
    }
    .to_token_stream()
}

fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec"
                && matches!(segment.arguments, PathArguments::AngleBracketed(_));
        }
    }
    false
}
