use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, ImplItem, Pat};

use crate::router_typescript;

pub(crate) fn srpc_router_impl(parsed_item: syn::ItemImpl) -> TokenStream {
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

            let params = item.sig.inputs.iter().filter_map(|item| {
                let syn::FnArg::Typed(item) = item else {return None};
                let Pat::Ident(ident) = &*item.pat else {
                    return None;
                };

                Some((ident.clone(), &item.ty))
            }).collect::<Vec<_>>();

            let (query_params, params_calls) = params.iter().map(|(ident, ty)| {
                let query_param = quote!(#ident: #ty);
                let param_call = quote!(params.#ident);

                (query_param, param_call)
            }).unzip::<_, _, Vec<_>, Vec<_>>();

            let call = match item.sig.asyncness {
                Some(_) => quote!(self.#name(#(#params_calls),*).await),
                None => quote!(self.#name(#(#params_calls),*)),
            };

            quote!(
                stringify!(#name) => {
                    #[derive(serde::Deserialize)]
                    struct QueryParams {
                        #(
                            #query_params
                        ),*
                    }

                    let axum::extract::Query(params) = match axum::extract::Query::<QueryParams>::try_from_uri(&uri) {
                        Ok(params) => params,
                        Err(err) => return err.into_response(),
                    };

                    let result = #call;
                    result.into_response()
                }
            )
        })
        .collect::<Vec<_>>();

    let typescript = router_typescript::generate_router_typescript(&parsed_item);

    let res = quote!(
        #parsed_item

        #[async_trait::async_trait]
        impl srpc::SrpcRouter for #name {
            async fn call(&self, call: &str, uri: axum::http::Uri) -> axum::response::Response {
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

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srpc_router_impl() {
        let input = quote! {
            impl MyRouter {
                pub async fn my_call(&self, this_is_a_param: String) -> i32 {
                    42
                }
            }
        };

        let parsed_item = syn::parse2(input).unwrap();

        let res = srpc_router_impl(parsed_item);
        eprintln!("{}", res);

        let res = prettyplease::unparse(&syn::parse2(res.clone()).unwrap());

        insta::assert_snapshot!(res, @r###"
        impl MyRouter {
            pub async fn my_call(&self, this_is_a_param: String) -> i32 {
                42
            }
        }
        #[async_trait::async_trait]
        impl srpc::SrpcRouter for MyRouter {
            async fn call(&self, call: &str, uri: axum::http::Uri) -> axum::response::Response {
                use axum::response::IntoResponse;
                match call {
                    stringify!(my_call) => {
                        #[derive(serde::Deserialize)]
                        struct QueryParams {
                            this_is_a_param: String,
                        }
                        let axum::extract::Query(params) = match axum::extract::Query::<
                            QueryParams,
                        >::try_from_uri(&uri) {
                            Ok(params) => params,
                            Err(err) => return err.into_response(),
                        };
                        let result = self.my_call(params.this_is_a_param).await;
                        result.into_response()
                    }
                    _ => {
                        (
                            axum::http::StatusCode::NOT_FOUND,
                            axum::Json(srpc::SrpcError::from("No such call")),
                        )
                            .into_response()
                    }
                }
            }
            fn generate_ts() -> String {
                use std::fmt::Write;
                let mut res = String::new();
                res.push_str("// This file is generated by srpc-derive\n");
                res.push_str("\n");
                res.push_str("import { rpcCall } from './rpcClient';\n");
                res.push_str("import { z } from 'zod';\n");
                res.push_str("\n");
                writeln!(
                    res, "export const myCallSchema = {};\n", < i32 as srpc::ZodSchema >
                    ::generate_zod_schema()
                )
                    .unwrap();
                res.push_str("type MyCallParams = {\nthis_is_a_param: string,\n};\n\n");
                res.push_str("export const client = {\n");
                res.push_str(
                    "myCall: async (params: MyCallParams) => rpcCall('my_call', myCallSchema, params),\n",
                );
                res.push_str("};\n\n");
                res
            }
        }
        "###);
    }
}
