use leptos::prelude::*;

#[component]
pub fn Badge(
    #[prop(into, default = "default".into())] variant: String,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2";

    let variant_styles = match variant.as_str() {
        "secondary" => {
            " border-transparent bg-gray-100 text-gray-900 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-100 dark:hover:bg-gray-700"
        }
        "destructive" => {
            " border-transparent bg-red-600 text-white hover:bg-red-700 dark:bg-red-900 dark:text-red-100"
        }
        "outline" => " text-gray-900 dark:text-gray-100 border-gray-200 dark:border-gray-800",
        _ => {
            " border-transparent bg-indigo-600 text-white hover:bg-indigo-700 dark:bg-indigo-500 dark:hover:bg-indigo-600"
        } // default
    };

    let combined = if class.is_empty() {
        format!("{}{}", base, variant_styles)
    } else {
        format!("{}{} {}", base, variant_styles, class)
    };

    view! { <div class=combined>{children()}</div> }
}
