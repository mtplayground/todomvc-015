use leptos::*;

use crate::api;
use crate::models::{Todo, UpdateTodo};

#[component]
pub fn TodoItem(
    todo: Todo,
    on_toggle: WriteSignal<Option<Todo>>,
    on_delete: WriteSignal<Option<String>>,
) -> impl IntoView {
    let id = todo.id.clone();
    let id_toggle = id.clone();
    let id_delete = id.clone();
    let (completed, set_completed) = create_signal(todo.completed);
    let (title, _set_title) = create_signal(todo.title.clone());

    let handle_toggle = move |_| {
        let new_completed = !completed.get();
        set_completed.set(new_completed);
        let id = id_toggle.clone();
        let on_toggle = on_toggle;
        spawn_local(async move {
            match api::update_todo(
                &id,
                &UpdateTodo {
                    completed: Some(new_completed),
                    ..Default::default()
                },
            )
            .await
            {
                Ok(updated) => on_toggle.set(Some(updated)),
                Err(e) => leptos::logging::error!("Failed to toggle todo: {}", e),
            }
        });
    };

    let handle_destroy = move |_| {
        let id = id_delete.clone();
        let on_delete = on_delete;
        spawn_local(async move {
            match api::delete_todo(&id).await {
                Ok(()) => on_delete.set(Some(id)),
                Err(e) => leptos::logging::error!("Failed to delete todo: {}", e),
            }
        });
    };

    view! {
        <li class:completed=completed>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=handle_toggle
                />
                <label>{title}</label>
                <button class="destroy" on:click=handle_destroy></button>
            </div>
        </li>
    }
}
