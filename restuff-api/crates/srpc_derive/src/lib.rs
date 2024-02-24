use syn::{self, parse_macro_input, DeriveInput};

mod router_typescript;
mod srpc_router;
mod ts_input;
mod zod;

#[proc_macro_derive(ZodSchema)]
pub fn zod_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let res = zod::zod_gen_impl(input);

    res.into()
}

#[proc_macro_derive(TsInput)]
pub fn ts_input(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let res = ts_input::ts_input_impl(input);

    res.into()
}

#[proc_macro_attribute]
pub fn srpc_router(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_item = parse_macro_input!(item as syn::ItemImpl);

    srpc_router::srpc_router_impl(parsed_item).into()
}
