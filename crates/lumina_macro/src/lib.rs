extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use lumina_core::BufferValue;

#[proc_macro_derive(BufferValue)]
pub fn buffer_value_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;


    let buffer_value = quote! {
        impl BufferValue for #name {}
    };

   return TokenStream::from(buffer_value);
}