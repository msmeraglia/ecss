extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Component)]
pub fn component_get_entity(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_get_entity_macro(&ast)
}

fn impl_get_entity_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, t_where) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics Component for #name #ty_generics #t_where {
            fn get_entity(&self) -> usize {
                self.entity_id
            }
        }
    };
    gen.into()
}
