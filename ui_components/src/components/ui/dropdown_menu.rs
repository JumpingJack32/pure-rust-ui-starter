use leptos::prelude::*;

#[component]
pub fn DropdownMenu(children: Children) -> impl IntoView {
    view! { <div class="relative inline-block text-left">{children()}</div> }
}

#[component]
pub fn DropdownMenuTrigger(
    open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = if class.is_empty() {
        "inline-flex w-full justify-center items-center".to_string()
    } else {
        format!("inline-flex w-full justify-center items-center {}", class)
    };

    let toggle = move |_| {
        open.update(|val| *val = !*val);
    };

    view! { <div class=combined on:click=toggle>{children()}</div> }
}

#[component]
pub fn DropdownMenuContent(
    open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "absolute right-0 z-50 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none dark:bg-gray-950 dark:ring-white/10";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <div class=combined style=move || if open.get() { "" } else { "display: none;" }>
            <div class="p-1" role="none">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800 rounded-sm cursor-pointer";
    let combined = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    view! {
        <button type="button" class=combined role="menuitem">
            {children()}
        </button>
    }
}
