use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn zod_gen_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let Data::Struct(content) = input.data else {
        panic!("ZodSchema only supports structs right now")
    };

    let fields = content.fields.iter().filter_map(|field| {
        let name = &field.ident.as_ref()?;

        let typename = &field.ty;

        Some(quote! {
            writeln!(res, "{}: {},", stringify!(#name), <#typename>::generate_zod_schema()).unwrap();
        })
    });

    quote! {
        impl srpc::ZodSchema for #name {
            fn generate_zod_schema() -> String {
                use std::fmt::Write;

                let mut res = String::new();

                writeln!(res, "z.object({{").unwrap();

                #(#fields)*

                writeln!(res, "}})").unwrap();

                res
            }
        }
    }
}
