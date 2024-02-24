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

        let line = format!("{name}: {{}},");

        Some(quote! {
            writeln!(res, #line, <#typename>::generate_zod_schema()).unwrap();
        })
    });

    quote! {
        impl srpc::ZodSchema for #name {
            fn generate_zod_schema() -> String {
                use std::fmt::Write;

                let mut res = String::new();

                res.push_str("z.object({\n");

                #(#fields)*

                res.push_str("})\n");

                res
            }
        }
    }
}
