use crate::components::theme_provider::{Theme, ThemeContext};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
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
        <Button
            variant=ButtonVariant::Ghost
            size=ButtonSize::Icon
            on:click=toggle_theme
        >
            {move || match theme_ctx.theme.get() {
                Theme::Light => "☀️",
                Theme::Dark => "🌙",
                Theme::System => "💻",
            }}
        </Button>
    }
}
