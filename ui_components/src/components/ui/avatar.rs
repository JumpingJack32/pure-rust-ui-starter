use leptos::prelude::*;

#[component]
pub fn Avatar(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-800 items-center justify-center";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(into, default = "Avatar image".into())] alt: String,
    #[prop(into, default = "".into())] class: String,
) -> impl IntoView {
    let base = "aspect-square h-full w-full object-cover";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <img src=src alt=alt class=combined />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex h-full w-full items-center justify-center rounded-full bg-gray-100 text-sm font-medium text-gray-700 dark:bg-gray-700 dark:text-gray-200";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <span class=combined>
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarBadge(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "absolute bottom-0 right-0 h-3 w-3 rounded-full ring-2 ring-white dark:ring-gray-900";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <span class=combined>
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarGroup(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex items-center -space-x-2 overflow-hidden";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn AvatarGroupCount(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "relative flex h-10 w-10 shrink-0 items-center justify-center rounded-full border-2 border-white bg-gray-300 text-xs font-semibold text-gray-800 dark:border-gray-900 dark:bg-gray-700 dark:text-gray-200";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}