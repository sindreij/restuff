use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn sprc_input_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let Data::Struct(content) = input.data else {
        panic!("SrpcInput derive only supports structs")
    };

    let fields = content.fields.iter().filter_map(|field| {
        let name = &field.ident.as_ref()?;

        let typename = &field.ty;

        let line = format!("{name}: {{}};");

        Some(quote! {
            writeln!(res, #line, <#typename as SrpcInput>::generate_ts_input_type()).unwrap();
        })
    });

    quote! {
        impl srpc::SrpcInput for #name {
            fn generate_ts_input_type() -> String {
                use std::fmt::Write;

                let mut res = String::new();

                res.push_str("{\n");

                #(#fields)*

                res.push_str("}\n");

                res
            }
        }
    }
}
