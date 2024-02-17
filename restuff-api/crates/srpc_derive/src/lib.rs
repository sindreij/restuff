use quote::quote;
use syn::{self, parse_macro_input, ImplItem};

mod ts;

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

    let typescript = ts::generate_ts(&parsed_item);

    quote!(
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

            fn generate_ts() -> &'static str {
                #typescript
            }
        }
    )
    .into()
}
