use leptos::prelude::*;

#[component]
pub fn Sheet(children: Children) -> impl IntoView {
    view! { <div class="relative">{children()}</div> }
}

#[component]
pub fn SheetContent(
    open: RwSignal<bool>,
    #[prop(into, default = "right".into())] side: String,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let side_styles = match side.as_str() {
        "left" => "left-0 top-0 h-full w-3/4 max-w-sm border-r",
        "top" => "top-0 inset-x-0 h-auto max-h-[80vh] border-b",
        "bottom" => "bottom-0 inset-x-0 h-auto max-h-[80vh] border-t",
        _ => "right-0 top-0 h-full w-3/4 max-w-sm border-l", // default: right
    };

    let base_content = "fixed z-50 gap-4 bg-white p-6 shadow-lg transition ease-in-out dark:bg-gray-950 dark:border-gray-800 text-gray-950 dark:text-gray-50 flex flex-col";
    let combined_content = if class.is_empty() {
        format!("{} {}", base_content, side_styles)
    } else {
        format!("{} {} {}", base_content, side_styles, class)
    };

    let close_sheet = move |_| open.set(false);

    view! {
        // Backdrop Overlay
        <div
            class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm transition-opacity"
            style=move || if open.get() { "" } else { "display: none;" }
            on:click=close_sheet
        />

        // Drawer Panel
        <div class=combined_content style=move || if open.get() { "" } else { "display: none;" }>
            <button
                type="button"
                class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-white transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:ring-offset-gray-950 dark:focus:ring-indigo-400"
                on:click=close_sheet
            >
                <span class="text-lg font-bold">"✕"</span>
                <span class="sr-only">"Close"</span>
            </button>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetHeader(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-col space-y-2 text-center sm:text-left";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn SheetTitle(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-lg font-semibold text-gray-950 dark:text-gray-50";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <h2 class=combined>{children()}</h2> }
}

#[component]
pub fn SheetDescription(
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
pub fn SheetFooter(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 mt-auto";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}
