use leptos::*;

use crate::api;
use crate::components::footer::TodoFooter;
use crate::components::header::TodoHeader;
use crate::components::todo_list::TodoList;
use crate::models::Todo;
use crate::router::use_filter;

#[component]
pub fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<Todo>>(Vec::new());
    let filter = use_filter();

    // Fetch todos on mount
    spawn_local({
        let set_todos = set_todos;
        async move {
            match api::fetch_todos().await {
                Ok(fetched) => set_todos.set(fetched),
                Err(e) => leptos::logging::error!("Failed to fetch todos: {}", e),
            }
        }
    });

    // Signal for header to report newly created todos
    let (created_todo, set_created_todo) = create_signal::<Option<Todo>>(None);

    // React to new todo creation
    create_effect(move |_| {
        if let Some(new_todo) = created_todo.get() {
            set_todos.update(|list| {
                list.push(new_todo);
            });
        }
    });

    let has_todos = move || !todos.get().is_empty();

    view! {
        <section class="todoapp">
            <TodoHeader on_create=set_created_todo />
            {move || {
                if has_todos() {
                    Some(view! {
                        <TodoList todos=todos set_todos=set_todos filter=filter />
                        <TodoFooter todos=todos set_todos=set_todos filter=filter />
                    })
                } else {
                    None
                }
            }}
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with Leptos + Axum"</p>
        </footer>
    }
}
