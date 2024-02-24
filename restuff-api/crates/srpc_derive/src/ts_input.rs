use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn ts_input_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let Data::Struct(content) = input.data else {
        panic!("TsInput derive only supports structs")
    };

    let fields = content.fields.iter().filter_map(|field| {
        let name = &field.ident.as_ref()?;

        let typename = &field.ty;

        let line = format!("{name}: {{}};");

        Some(quote! {
            writeln!(res, #line, <#typename as TsInput>::generate_ts_input_type()).unwrap();
        })
    });

    quote! {
        impl srpc::TsInput for #name {
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
