use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput, Generics, Ident};

pub fn derive_spanned_impl(item: DeriveInput) -> TokenStream {
    match item.data {
        syn::Data::Struct(data) => impl_spanned_for_struct(&item.ident, &item.generics, data),
        syn::Data::Enum(data) => impl_spanned_for_enum(&item.ident, &item.generics, data),
        syn::Data::Union(_) => unimplemented!(),
    }
}

fn impl_spanned_for_struct(ident: &Ident, generics: &Generics, data: DataStruct) -> TokenStream {
    if !data.fields.into_iter().any(|f| {
        f.ident
            .unwrap_or_else(|| panic!("tuple structs are not supported"))
            == "span"
    }) {
        panic!(
            "struct {} must have a `span: metal_lexer::Span` field",
            ident
        );
    }

    quote! {
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
        impl #generics ::metal_lexer::Spanned for #ident #generics {
            fn span(&self) -> &::metal_lexer::Span {
                match self {
                    #(#arms),*
                }
            }
        }
    }
}
