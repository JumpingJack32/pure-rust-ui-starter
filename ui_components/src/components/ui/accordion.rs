use leptos::prelude::*;

#[component]
pub fn Accordion(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "divide-y divide-gray-200 dark:divide-gray-800";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn AccordionItem(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "border-b border-gray-200 dark:border-gray-800";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn AccordionTrigger(
    open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline text-left text-gray-900 dark:text-gray-100 cursor-pointer";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    let toggle = move |_| {
        open.update(|val| *val = !*val);
    };

    view! {
        <button type="button" class=combined on:click=toggle>
            {children()}
            <span class=move || {
                if open.get() {
                    "transform rotate-180 transition-transform duration-200"
                } else {
                    "transform rotate-0 transition-transform duration-200"
                }
            }>"▼"</span>
        </button>
    }
}

#[component]
pub fn AccordionContent(
    open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "pb-4 pt-0 text-sm text-gray-600 dark:text-gray-400";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <div class=combined style=move || if open.get() { "" } else { "display: none;" }>
            {children()}
        </div>
    }
}
