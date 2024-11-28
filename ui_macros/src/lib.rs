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
#[proc_macro_derive(SelectList)]
pub fn select_list_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_select_list(&ast)
}

fn impl_select_list(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Extract the type of the field named `list`
    let list_type = if let syn::Data::Struct(data_struct) = &ast.data {
        data_struct
            .fields
            .iter()
            .find_map(|field| {
                if field.ident.as_ref().map_or(false, |ident| ident == "list") {
                    if let syn::Type::Path(type_path) = &field.ty {
                        return Some(type_path.clone());
                    }
                }
                None
            })
            .expect("Expected a field named `list` with a concrete type")
    } else {
        panic!("SelectList can only be derived for structs");
    };

    // Extract the inner type from Vec<T>
    let inner_type = {
        let path = &list_type.path;
        if let Some(seg) = path.segments.iter().last() {
            if seg.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        inner.clone()
                    } else {
                        panic!("Expected Vec with a single generic argument");
                    }
                } else {
                    panic!("Expected Vec with angle bracket arguments");
                }
            } else {
                panic!("Expected field `list` to be of type Vec");
            }
        } else {
            panic!("Expected a valid path for type Vec");
        }
    };

    let gen = quote! {
        impl SelectList for #name {
            type Item = #inner_type;

            fn new(item: #inner_type) -> Self {
                Self {
                    selected: 0,
                    list: vec![item],
                }
            }

            fn get_selected(&self) -> &#inner_type {
                &self.list[self.selected]
            }

            fn select(&mut self, index: usize) {
                self.selected = index.min(self.list.len() - 1);
            }
        }
    };
    gen.into()
}
