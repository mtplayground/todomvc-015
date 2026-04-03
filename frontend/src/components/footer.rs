use leptos::*;

use crate::api;
use crate::models::Todo;
use crate::router::Filter;

#[component]
pub fn TodoFooter(
    todos: ReadSignal<Vec<Todo>>,
    set_todos: WriteSignal<Vec<Todo>>,
    filter: ReadSignal<Filter>,
) -> impl IntoView {
    let active_count = move || todos.get().iter().filter(|t| !t.completed).count();
    let has_completed = move || todos.get().iter().any(|t| t.completed);

    let handle_clear_completed = move |_| {
        let set_todos = set_todos;
        spawn_local(async move {
            match api::delete_completed().await {
                Ok(()) => {
                    set_todos.update(|list| {
                        list.retain(|t| !t.completed);
                    });
                }
                Err(e) => leptos::logging::error!("Failed to clear completed: {}", e),
            }
        });
    };

    let items_text = move || {
        let count = active_count();
        if count == 1 {
            "1 item left".to_string()
        } else {
            format!("{} items left", count)
        }
    };

    view! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{active_count}</strong>
                " "
                {items_text}
            </span>
            <ul class="filters">
                <li>
                    <a href="#/" class:selected=move || filter.get() == Filter::All>"All"</a>
                </li>
                <li>
                    <a href="#/active" class:selected=move || filter.get() == Filter::Active>"Active"</a>
                </li>
                <li>
                    <a href="#/completed" class:selected=move || filter.get() == Filter::Completed>"Completed"</a>
                </li>
            </ul>
            {move || {
                if has_completed() {
                    Some(view! {
                        <button class="clear-completed" on:click=handle_clear_completed>
                            "Clear completed"
                        </button>
                    })
                } else {
                    None
                }
            }}
        </footer>
    }
}
