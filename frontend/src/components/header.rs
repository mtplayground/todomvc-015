use leptos::*;

use crate::api;
use crate::models::{CreateTodo, Todo};

#[component]
pub fn TodoHeader(on_create: WriteSignal<Option<Todo>>) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(String::new());

    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            let title = input_value.get().trim().to_string();
            if title.is_empty() {
                return;
            }
            set_input_value.set(String::new());
            let on_create = on_create;
            spawn_local(async move {
                match api::create_todo(&CreateTodo { title }).await {
                    Ok(todo) => on_create.set(Some(todo)),
                    Err(e) => leptos::logging::error!("Failed to create todo: {}", e),
                }
            });
        }
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autofocus=true
                prop:value=input_value
                on:input=move |ev| set_input_value.set(event_target_value(&ev))
                on:keydown=handle_keydown
            />
        </header>
    }
}
