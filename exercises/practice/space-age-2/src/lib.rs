extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn planet(attr: TokenStream, item: TokenStream) -> TokenStream {

    let ast = syn::parse(item).unwrap();
    impl_planet_macro(&ast, attr.to_string().parse::<f64>().unwrap_or(1.0).to_owned())
}

fn impl_planet_macro(ast: &syn::DeriveInput, rate: f64) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        struct #name;
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / (31557600.0 * #rate)
            }
        }
    };
    gen.into()
}
