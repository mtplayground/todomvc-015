use leptos::*;

use crate::api;
use crate::components::todo_item::TodoItem;
use crate::models::Todo;

#[component]
pub fn TodoList(todos: ReadSignal<Vec<Todo>>, set_todos: WriteSignal<Vec<Todo>>) -> impl IntoView {
    let all_completed = move || todos.get().iter().all(|t| t.completed) && !todos.get().is_empty();

    let handle_toggle_all = move |_| {
        let new_completed = !all_completed();
        let set_todos = set_todos;
        spawn_local(async move {
            match api::toggle_all(new_completed).await {
                Ok(updated_todos) => set_todos.set(updated_todos),
                Err(e) => leptos::logging::error!("Failed to toggle all: {}", e),
            }
        });
    };

    // Signal for child TodoItem to report toggled/deleted items
    let (toggled_todo, set_toggled_todo) = create_signal::<Option<Todo>>(None);
    let (deleted_id, set_deleted_id) = create_signal::<Option<String>>(None);

    // React to toggle events from children
    create_effect(move |_| {
        if let Some(updated) = toggled_todo.get() {
            set_todos.update(|list| {
                if let Some(item) = list.iter_mut().find(|t| t.id == updated.id) {
                    *item = updated;
                }
            });
        }
    });

    // React to delete events from children
    create_effect(move |_| {
        if let Some(id) = deleted_id.get() {
            set_todos.update(|list| {
                list.retain(|t| t.id != id);
            });
        }
    });

    view! {
        <section class="main">
            <input
                id="toggle-all"
                class="toggle-all"
                type="checkbox"
                prop:checked=all_completed
                on:change=handle_toggle_all
            />
            <label for="toggle-all">"Mark all as complete"</label>
            <ul class="todo-list">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id.clone()
                    children=move |todo| {
                        view! {
                            <TodoItem
                                todo=todo
                                on_toggle=set_toggled_todo
                                on_delete=set_deleted_id
                            />
                        }
                    }
                />
            </ul>
        </section>
    }
}
