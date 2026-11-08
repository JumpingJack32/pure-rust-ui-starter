// app/src/lib.rs
pub mod components; // <-- This makes 'crate::components' available to app.rs and other files

pub mod app;
// If you have your main App component or router entry point here:
pub use app::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
