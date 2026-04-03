use leptos::*;

pub mod api;
pub mod app;
pub mod components;
pub mod models;
pub mod router;

fn main() {
    mount_to_body(app::TodoApp);
}
