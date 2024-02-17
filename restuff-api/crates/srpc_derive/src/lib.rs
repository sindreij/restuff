use quote::quote;
use syn::{self, parse_macro_input, ImplItem};

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

    quote!(
        #parsed_item

        impl SrpcRouter for #name {
            fn call(&self, call: &str) -> Response {
                match call {
                    #(#calls)*
                    _ => (
                        axum::http::StatusCode::NOT_FOUND,
                        axum::Json(SrpcError::from("No such call")),
                    ).into_response()
                }
            }
        }
    )
    .into()
}
