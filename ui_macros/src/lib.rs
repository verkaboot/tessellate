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

#[proc_macro_derive(SelectList, attributes(select_list_type))]
pub fn select_list_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_select_list(&ast)
}

fn impl_select_list(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let select_list_type = ast
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("select_list_type") {
                attr.parse_args::<syn::Type>().ok()
            } else {
                None
            }
        })
        .expect("Expected a `select_list_type` attribute specifying the generic type");

    let gen = quote! {
        impl #impl_generics SelectList<#select_list_type> for #name #ty_generics #where_clause {
            fn new(item: #select_list_type) -> Self {
                Self {
                    selected: 0,
                    list: vec![item],
                }
            }

            fn get_selected(&self) -> &#select_list_type {
                &self.list[self.selected]
            }

            fn select(&mut self, index: usize) {
                self.selected = index.min(self.list.len() - 1);
            }
        }
    };
    gen.into()
}
