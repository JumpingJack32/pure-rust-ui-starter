use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust_ui_starter.css" />
        <Title text="Pure Rust UI Starter (Leptos 0.9)" />

        <Router>
            <main class="container">
                <Routes fallback=|| "404: Page Not Found">
                    <Route path=path!("/") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <h1>"Pure Rust Starter"</h1>
        <p class="bg-background text-red-600 text-4xl">
            "Built with Leptos 0.9.0-rc1 — pure WebAssembly performance without Node."
        </p>

        // Signal call count() works seamlessly on Stable Rust in 0.9!
        <button
            style="border: medium dashed green;"
            on:click=move |_| { set_count.update(|n| *n += 1) }
        >
            "Clicks: "
            {move || count()}
        </button>
    }
}
