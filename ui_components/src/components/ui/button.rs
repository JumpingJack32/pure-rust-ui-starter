use leptos::prelude::*;

// 1. Define the Variant Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
    Ghost,
}

// 2. Define the Size Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Icon,
}

// 3. Keep styling centralized via a single helper function
pub fn button_variants(variant: ButtonVariant, size: ButtonSize, additional_class: &str) -> String {
    let base_styles = "inline-flex items-center justify-center rounded-lg font-medium text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_styles = match variant {
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

    let size_styles = match size {
        ButtonSize::Sm => "px-3 py-1.5 text-xs",
        ButtonSize::Md => "px-4 py-2 text-sm",
        ButtonSize::Lg => "px-5 py-2.5 text-base",
        ButtonSize::Icon => "h-9 w-9 p-0", // Perfect for putting an SVG inside
    };

    if additional_class.is_empty() {
        format!("{} {} {}", base_styles, variant_styles, size_styles)
    } else {
        format!(
            "{} {} {} {}",
            base_styles, variant_styles, size_styles, additional_class
        )
    }
}

// 4. The Unified Button Component
#[component]
pub fn Button(
    #[prop(into, default = "button".into())] r#type: String,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(into, default = "".into())] class: String,
    #[prop(default = false)] disabled: bool,
    children: Children,
) -> impl IntoView {
    let combined_classes = button_variants(variant, size, &class);

    view! {
        <button type=r#type class=combined_classes disabled=disabled>
            {children()}
        </button>
    }
}
