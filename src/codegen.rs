use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::parse::FieldBinding;

pub fn emit_default(struct_name: &Ident, bindings: &[FieldBinding]) -> TokenStream {
    let field_idents = bindings.iter().map(|b| &b.ident);
    let env_keys = bindings.iter().map(|b| b.env_key.as_str());

    quote! {
        impl ::core::default::Default for #struct_name {
            fn default() -> Self {
                #struct_name {
                    #( #field_idents: ::dotenvy_macro::dotenv!(#env_keys), )*
                }
            }
        }
    }
}

pub fn emit_static(struct_name: &Ident, bindings: &[FieldBinding]) -> TokenStream {
    let field_idents = bindings.iter().map(|b| &b.ident);
    let env_keys = bindings.iter().map(|b| b.env_key.as_str());

    quote! {
        impl #struct_name {
            pub const INSTANCE: #struct_name = #struct_name {
                #( #field_idents: ::dotenvy_macro::dotenv!(#env_keys), )*
            };
        }
    }
}
