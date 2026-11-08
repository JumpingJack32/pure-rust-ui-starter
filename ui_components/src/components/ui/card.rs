use leptos::prelude::*;

#[component]
pub fn Card(#[prop(into, default = "".into())] class: String, children: Children) -> impl IntoView {
    let base = "rounded-xl border border-gray-200 bg-white text-gray-900 shadow-sm dark:border-gray-800 dark:bg-gray-950 dark:text-gray-50";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn CardHeader(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-col space-y-1.5 p-6";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn CardTitle(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-lg font-semibold leading-none tracking-tight";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <h3 class=combined>{children()}</h3> }
}

#[component]
pub fn CardDescription(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-sm text-gray-500 dark:text-gray-400";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <p class=combined>{children()}</p> }
}

#[component]
pub fn CardContent(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "p-6 pt-0";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn CardFooter(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex items-center p-6 pt-0";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}
