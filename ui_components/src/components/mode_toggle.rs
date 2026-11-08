// app/src/components/mode_toggle.rs
use crate::components::theme_provider::{Theme, ThemeContext};
// use crate::components::theme_provider::ThemeContext;
use crate::components::ui::button::Button; // Adjust path if your button is elsewhere
use leptos::prelude::*;

#[component]
pub fn ModeToggle() -> impl IntoView {
    let theme_ctx =
        use_context::<ThemeContext>().expect("ModeToggle must be used within a ThemeProvider");

    let toggle_theme = move |_| {
        let next_theme = match theme_ctx.theme.get() {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::System,
            Theme::System => Theme::Light,
        };
        theme_ctx.set_theme.run(next_theme);
    };

    view! {
        <Button variant="outline" on:click=toggle_theme class="px-3 py-1.5 text-xs font-semibold">
            {move || match theme_ctx.theme.get() {
                Theme::Light => "☀️ Light",
                Theme::Dark => "🌙 Dark",
                Theme::System => "💻 System",
            }}
        </Button>
    }
}
