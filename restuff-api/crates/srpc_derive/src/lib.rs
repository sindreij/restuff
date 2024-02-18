use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, ImplItem};

mod router_typescript;
mod zod;

#[proc_macro_derive(ZodGen)]
pub fn zod_gen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let res = zod::zod_gen_impl(input);

    // eprintln!(
    //     "{}",
    //     prettyplease::unparse(&syn::parse(res.clone().into()).unwrap())
    // );

    res.into()
}

#[proc_macro_attribute]
pub fn srpc_router(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_item = parse_macro_input!(item as syn::ItemImpl);

    let name = &parsed_item.self_ty;

    let calls = parsed_item
        .items
        .iter()
        .filter_map(|item| {
            if let ImplItem::Fn(item) = item {
                if let syn::Visibility::Public(_) = item.vis {
                    Some(item)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|item| {
            let name = &item.sig.ident;

            quote!(
                stringify!(#name) => {
                    let result = self.#name();
                    axum::Json(result).into_response()
                }
            )
        })
        .collect::<Vec<_>>();

    let typescript = router_typescript::generate_router_typescript(&parsed_item);

    let res = quote!(
        #parsed_item

        impl srpc::SrpcRouter for #name {
            fn call(&self, call: &str) -> axum::response::Response {
                use axum::response::IntoResponse;

                match call {
                    #(#calls)*
                    _ => (
                        axum::http::StatusCode::NOT_FOUND,
                        axum::Json(srpc::SrpcError::from("No such call")),
                    ).into_response()
                }
            }

            fn generate_ts() -> String {
                #typescript
            }
        }
    );

    // eprintln!(
    //     "{}",
    //     prettyplease::unparse(&syn::parse(res.clone().into()).unwrap())
    // );

    res.into()
}
