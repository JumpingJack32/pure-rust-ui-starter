use leptos::prelude::*;

// 1. Define the variants using a type-safe Rust enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
    Ghost,
}

// 2. Implement the styling helper directly on the enum
impl ButtonVariant {
    pub fn to_classes(self, additional_class: &str) -> String {
        let base_styles = "inline-flex items-center justify-center px-4 py-2 rounded-lg font-medium text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed";

        let variant_styles = match self {
            ButtonVariant::Outline => {
                "border border-gray-300 text-gray-700 bg-white hover:bg-gray-50 focus:ring-indigo-500"
            }
            ButtonVariant::Ghost => {
                "text-gray-700 bg-transparent hover:bg-gray-100 focus:ring-gray-500"
            }
            ButtonVariant::Primary => {
                "bg-indigo-400 text-white hover:bg-indigo-700 focus:ring-indigo-500"
            }
        };

        if additional_class.is_empty() {
            format!("{} {}", base_styles, variant_styles)
        } else {
            format!("{} {} {}", base_styles, variant_styles, additional_class)
        }
    }
}

// 3. The Button component
#[component]
pub fn Button(
    #[prop(into, default = "button".into())] r#type: String,
    #[prop(into, default = "primary".into())] variant: String,
    #[prop(into, default = "".into())] class: String,
    #[prop(default = false)] disabled: bool,
    children: Children,
) -> impl IntoView {
    // Shared base styles for all buttons
    let base_styles = "inline-flex items-center justify-center px-4 py-2 rounded-lg font-medium text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed";

    // Variant-specific styles
    let variant_styles = match variant.as_str() {
        "outline" => {
            " border border-gray-300 text-gray-700 bg-white hover:bg-gray-50 focus:ring-indigo-500"
        }
        "ghost" => " text-gray-700 bg-transparent hover:bg-gray-100 focus:ring-gray-500",
        _ => " bg-indigo-400 text-white hover:bg-indigo-700 focus:ring-indigo-500", // default "primary"
    };

    let combined_classes = format!(
        "{}{}{}{}",
        base_styles,
        variant_styles,
        if class.is_empty() { "" } else { " " },
        class
    );

    view! {
        <button type=r#type class=combined_classes disabled=disabled>
            {children()}
        </button>
    }
}
