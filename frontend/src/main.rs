use leptos::*;

pub mod api;
pub mod components;
pub mod models;
pub mod router;

#[component]
fn App() -> impl IntoView {
    view! {
        <p>"Hello, TodoMVC!"</p>
    }
}

fn main() {
    mount_to_body(App);
}
