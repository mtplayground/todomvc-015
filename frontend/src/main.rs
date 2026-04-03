use leptos::*;

pub mod api;
pub mod models;

#[component]
fn App() -> impl IntoView {
    view! {
        <p>"Hello, TodoMVC!"</p>
    }
}

fn main() {
    mount_to_body(App);
}
