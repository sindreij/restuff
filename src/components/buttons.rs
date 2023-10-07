use leptos::{component, tracing, view, IntoView, Scope, *};

#[component]
pub fn Button(cx: Scope, #[prop(into)] type_: String, children: Children) -> impl IntoView {
    view! {cx,
        <button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" type="button">
            {children(cx)}
        </button>
    }
}
