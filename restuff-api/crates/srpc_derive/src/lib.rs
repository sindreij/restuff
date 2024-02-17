use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(SrpcRouter)]
pub fn srpc_router_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let ast = syn::parse(input).unwrap();
    let input = parse_macro_input!(input as DeriveInput);

    impl_srpc_router(&input).into()
}

fn impl_srpc_router(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    dbg!(ast);

    quote! {
        impl SrpcRouter for #name {
            fn call(&self, call: &str) -> Response {
                todo!("todo")
            }
        }
    }
}
