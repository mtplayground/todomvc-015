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
    let id_edit = id.clone();
    let id_edit_blur = id.clone();
    let (completed, set_completed) = create_signal(todo.completed);
    let (title, set_title) = create_signal(todo.title.clone());
    let (editing, set_editing) = create_signal(false);
    let (edit_text, set_edit_text) = create_signal(todo.title.clone());
    let edit_input = create_node_ref::<html::Input>();

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

    let start_editing = move |_| {
        set_edit_text.set(title.get());
        set_editing.set(true);
        // Focus the input after it renders
        let input = edit_input;
        request_animation_frame(move || {
            if let Some(el) = input.get() {
                let _ = el.focus();
            }
        });
    };

    let save_edit = {
        let id_edit = id_edit.clone();
        let id_edit_blur = id_edit_blur.clone();
        std::rc::Rc::new(move || {
            let trimmed = edit_text.get().trim().to_string();
            set_editing.set(false);

            if trimmed.is_empty() {
                let id = id_edit.clone();
                let on_delete = on_delete;
                spawn_local(async move {
                    match api::delete_todo(&id).await {
                        Ok(()) => on_delete.set(Some(id)),
                        Err(e) => leptos::logging::error!("Failed to delete todo: {}", e),
                    }
                });
                return;
            }

            if trimmed == title.get() {
                return;
            }

            set_title.set(trimmed.clone());
            let id = id_edit_blur.clone();
            let on_toggle = on_toggle;
            spawn_local(async move {
                match api::update_todo(
                    &id,
                    &UpdateTodo {
                        title: Some(trimmed),
                        ..Default::default()
                    },
                )
                .await
                {
                    Ok(updated) => on_toggle.set(Some(updated)),
                    Err(e) => leptos::logging::error!("Failed to save edit: {}", e),
                }
            });
        })
    };

    let save_for_keydown = save_edit.clone();
    let save_for_blur = save_edit.clone();

    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
        "Enter" => save_for_keydown(),
        "Escape" => {
            set_edit_text.set(title.get());
            set_editing.set(false);
        }
        _ => {}
    };

    let handle_blur = move |_| {
        if editing.get() {
            save_for_blur();
        }
    };

    view! {
        <li class:completed=completed class:editing=editing>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=handle_toggle
                />
                <label on:dblclick=start_editing>{title}</label>
                <button class="destroy" on:click=handle_destroy></button>
            </div>
            <input
                class="edit"
                node_ref=edit_input
                prop:value=edit_text
                on:input=move |ev| set_edit_text.set(event_target_value(&ev))
                on:blur=handle_blur
                on:keydown=handle_keydown
            />
        </li>
    }
}
