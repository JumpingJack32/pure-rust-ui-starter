use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(into, default = "text".into())] r#type: String,
    #[prop(into, default = "".into())] placeholder: String,
    #[prop(into, default = "".into())] value: String,
    #[prop(into, default = "".into())] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let base = "flex h-10 w-full rounded-md border border-gray-200 bg-white px-3 py-2 text-sm ring-offset-white file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-800 dark:bg-gray-950 dark:ring-offset-gray-950 dark:placeholder:text-gray-400 dark:focus-visible:ring-indigo-400";

    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <input type=r#type placeholder=placeholder value=value disabled=disabled class=combined /> }
}
