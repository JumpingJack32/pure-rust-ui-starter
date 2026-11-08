use crate::components::landing::Navbar;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use ui_components::Button;
use ui_components::ThemeProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <ThemeProvider>
            <Stylesheet id="leptos" href="/pkg/rust_ui_starter.css" />
            <Title text="Pure Rust UI Starter (Leptos 0.9)" />
            <Router>
                <main class="container snap-center">
                    <Routes fallback=|| "404: Page Not Found">
                        <Route path=path!("/") view=HomePage />
                    </Routes>
                </main>
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // let (count, set_count) = signal(0);

    view! {
        <div class="min-h-screen w-screen bg-background text-foreground transition-colors duration-200">
            <Navbar />
            // <header class="p-4 border-b border-border flex justify-between items-center">
            // <h1 class="text-xl font-bold">"My Leptos App"</h1>
            // <ModeToggle />
            // </header>
            <div class="p-8 m-4">
                <p class="p-8 m-4 text-amber-600">
                    "Welcome to your high-performance Leptos scaffold."
                </p>
                <div class="m4">
                    <Button class="m-4" variant="primary">
                        "Primary Button"
                    </Button>
                    <Button class="m-4" variant="outline">
                        "Outline Button"
                    </Button>
                    <Button class="m-4" variant="ghost">
                        "Ghost Button"
                    </Button>
                </div>
            </div>
        </div>
    }
}
