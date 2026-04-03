use leptos::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn from_hash(hash: &str) -> Self {
        match hash {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

fn get_current_hash() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
}

pub fn use_filter() -> ReadSignal<Filter> {
    let (filter, set_filter) = create_signal(Filter::from_hash(&get_current_hash()));

    // Listen for hashchange events
    let closure = Closure::<dyn Fn()>::new(move || {
        set_filter.set(Filter::from_hash(&get_current_hash()));
    });

    if let Some(window) = web_sys::window() {
        let _ =
            window.add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref());
    }

    // Leak the closure so it lives for the lifetime of the app
    closure.forget();

    filter
}
