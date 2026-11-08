use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub set_theme: Callback<Theme>,
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(Theme::System);

    let set_theme = Callback::new(move |new_theme: Theme| {
        theme.set(new_theme);

        // Apply class to <html> element via JS on client-side
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(root) = document.document_element() {
                        let is_dark = match new_theme {
                            Theme::Dark => true,
                            Theme::Light => false,
                            Theme::System => window
                                .match_media("(prefers-color-scheme: dark)")
                                .ok()
                                .flatten()
                                .map(|m| m.matches())
                                .unwrap_or(false),
                        };

                        if is_dark {
                            let _ = root.class_list().add_1("dark");
                        } else {
                            let _ = root.class_list().remove_1("dark");
                        }
                    }
                }
            }
        }
    });

    provide_context(ThemeContext { theme, set_theme });

    children()
}
