// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use proc_macro2::TokenStream;
use proc_macro_error2::abort;
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, Generics, Ident};

pub fn derive_spanned_impl(item: DeriveInput) -> TokenStream {
    match item.data {
        syn::Data::Struct(data) => impl_spanned_for_struct(&item.ident, &item.generics, data),
        syn::Data::Enum(data) => impl_spanned_for_enum(&item.ident, &item.generics, data),
        syn::Data::Union(_) => abort!(&item, "union structs are not supported"),
    }
}

fn impl_spanned_for_struct(ident: &Ident, generics: &Generics, data: DataStruct) -> TokenStream {
    let Fields::Named(FieldsNamed { named: fields, .. }) = data.fields else {
        abort!(&data.fields, "unit and tuple structs are not supported");
    };

    if !fields
        .into_iter()
        .filter_map(|f| f.ident)
        .any(|i| i == "span")
    {
        abort!(&ident, "struct must have a `span: metal_lexer::Span` field",);
    }

    quote! {
        #[automatically_derived]
        impl #generics ::metal_lexer::Spanned for #ident #generics {
            fn span(&self) -> &::metal_lexer::Span {
                &self.span
            }
        }
    }
}

fn impl_spanned_for_enum(ident: &Ident, generics: &Generics, data: DataEnum) -> TokenStream {
    let arms = data.variants.iter().map(|v| {
        let variant = &v.ident;

        quote! {
            #ident :: #variant (obj) => ::metal_lexer::Spanned::span(obj)
        }
    });

    quote! {
        #[automatically_derived]
        impl #generics ::metal_lexer::Spanned for #ident #generics {
            fn span(&self) -> &::metal_lexer::Span {
                match self {
                    #(#arms),*
                }
            }
        }
    }
}
