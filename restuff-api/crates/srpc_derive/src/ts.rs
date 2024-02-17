use syn::ImplItem;

pub(crate) fn generate_ts(parsed_item: &syn::ItemImpl) -> String {
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
            let name = &item.sig.ident.to_string();

            format!("    {name}: async () => rpc_call('{name}'),")
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("Hello world");

    dbg!(&calls);

    format!(
        "
export const client = {{
{calls}
}};
    "
    )
    .trim()
    .into()
}
