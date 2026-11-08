use leptos::prelude::*;

#[component]
pub fn NavigationMenu(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "relative z-10 flex max-w-max flex-1 items-center justify-center";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <nav class=combined>{children()}</nav> }
}

#[component]
pub fn NavigationMenuList(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "group flex flex-1 list-none items-center justify-center space-x-1";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <ul class=combined>{children()}</ul> }
}

#[component]
pub fn NavigationMenuItem(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = if class.is_empty() {
        "relative".to_string()
    } else {
        format!("relative {}", class)
    };

    view! { <li class=combined>{children()}</li> }
}

#[component]
pub fn NavigationMenuLink(
    #[prop(into, default = "#".into())] href: String,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "group inline-flex h-9 w-max items-center justify-center rounded-md px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 hover:text-gray-900 focus:bg-gray-100 focus:text-gray-900 focus:outline-none dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:bg-gray-800 dark:focus:text-gray-50";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <a href=href class=combined>
            {children()}
        </a>
    }
}
