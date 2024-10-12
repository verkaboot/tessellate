extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(SliderValue)]
pub fn slider_value_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_slider_value(&ast)
}

fn impl_slider_value(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SliderValue for #name {
            fn from_f32(input: f32) -> Self {
                Self(input)
            }

            fn to_f32(&self) -> f32 {
                self.0
            }
        }
    };
    gen.into()
}
