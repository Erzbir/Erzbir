use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Languages)]
pub fn impl_language(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Languages for #name {}
    };
    gen.into()
}

#[proc_macro_derive(Hobbies)]
pub fn impl_hobby(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Hobbies for #name {}
    };
    gen.into()
}


#[proc_macro_derive(TechStack)]
pub fn impl_tech_stack(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TechStack for #name {}
    };
    gen.into()
}
