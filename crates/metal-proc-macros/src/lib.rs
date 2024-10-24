use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

// TODO: move implementations to their own respective submodules
// TODO: use proc-macro-error2 instead of panicking

#[proc_macro_derive(Spanned)]
pub fn spanned_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(data) => impl_spanned_for_struct(&input.ident, &input.generics, data),
        syn::Data::Enum(data) => impl_spanned_for_enum(&input.ident, &input.generics, data),
        syn::Data::Union(_) => unimplemented!(),
    }
}

fn impl_spanned_for_struct(
    ident: &syn::Ident,
    generics: &syn::Generics,
    data: syn::DataStruct,
) -> TokenStream {
    if data
        .fields
        .into_iter()
        .find(|f| {
            f.ident
                .as_ref()
                .unwrap_or_else(|| panic!("tuple structs are not supported"))
                .to_string()
                == "span"
        })
        .is_none()
    {
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
    .into()
}

fn impl_spanned_for_enum(
    ident: &syn::Ident,
    generics: &syn::Generics,
    data: syn::DataEnum,
) -> TokenStream {
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
    .into()
}

/// Add a `span: metal_lexer::Span` field to a named struct.
#[proc_macro_attribute]
pub fn spanned(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: syn::ItemStruct = syn::parse(item).expect("expected a struct item");

    let span_field: syn::Field = syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Public(syn::Token![pub](Span::call_site())),
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new("span", Span::call_site())),
        colon_token: Some(syn::Token![:](Span::call_site())),
        ty: syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: Some(syn::Token![::](Span::call_site())),
                segments: syn::punctuated::Punctuated::from_iter(
                    std::iter::once(syn::PathSegment {
                        ident: syn::Ident::new("metal_lexer", Span::call_site()),
                        arguments: syn::PathArguments::None,
                    })
                    .chain(std::iter::once(syn::PathSegment {
                        ident: syn::Ident::new("Span", Span::call_site()),
                        arguments: syn::PathArguments::None,
                    })),
                ),
            },
        }),
    };

    match &mut item.fields {
        syn::Fields::Named(fields) => fields.named.push(span_field),
        _ => panic!("tuple structs are not supported"),
    }

    item.to_token_stream().into()
}
