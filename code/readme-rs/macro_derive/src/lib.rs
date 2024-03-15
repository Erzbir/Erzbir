use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(ExpertiseImpl)]
pub fn impl_expertise(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Expertise for #name {}
    };
    gen.into()
}

#[proc_macro_derive(HobbyImpl)]
pub fn impl_hobby(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Hobby for #name {}
    };
    gen.into()
}
