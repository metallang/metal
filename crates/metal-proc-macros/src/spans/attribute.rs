use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, Field, FieldMutability, Fields, Ident, PathArguments, PathSegment,
    Token, Type, TypePath, Visibility,
};

pub fn spanned_impl(item: TokenStream) -> TokenStream {
    let mut item: syn::ItemStruct = syn::parse2(item).expect("expected a struct item");

    let span_ty = Type::Path(TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: Some(Token![::](Span::call_site())),
            segments: Punctuated::from_iter(vec![
                PathSegment {
                    ident: Ident::new("metal_lexer", Span::call_site()),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: Ident::new("Span", Span::call_site()),
                    arguments: PathArguments::None,
                },
            ]),
        },
    });

    let span_field: Field = Field {
        attrs: vec![],
        vis: Visibility::Public(Token![pub](Span::call_site())),
        mutability: FieldMutability::None,
        ident: Some(Ident::new("span", Span::call_site())),
        colon_token: Some(Token![:](Span::call_site())),
        ty: span_ty,
    };

    match &mut item.fields {
        Fields::Named(fields) => fields.named.push(span_field),
        _ => panic!("tuple structs are not supported"),
    }

    item.to_token_stream()
}
