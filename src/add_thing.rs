use leptos::*;
use leptos_router::{ActionForm, MultiActionForm};

use crate::components::Button;

#[server(AddThing, "/api")]
pub async fn add_thing(cx: Scope, title: String) -> Result<(), ServerFnError> {
    println!("add_thing: {}", title);

    leptos_axum::redirect(cx, "/");
    Ok(())
}

#[component]
pub fn AddThing(cx: Scope) -> impl IntoView {
    let add_thing = create_server_action::<AddThing>(cx);

    // create_effect(cx, move |_| {
    //     log!("add_thing: {}", add_thing.version().get());
    // });

    view! {cx,
        <div class="max-w-screen-lg w-full mx-auto p3">
            <h1 class="text-lg font-bold">Add Thing</h1>
            <ActionForm action=add_thing>
                <label>Title
                    <input type="text" class="border-4" name="title" />
                </label>
                <Button type_="submit">Add</Button>
            </ActionForm>
        </div>
    }
}
