<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/header/graph.svg?title=Pure%20Rust-UI%20Starter&subtitle=Pure%20Rust%20UI%20Starter&logo=rust&mode=dark&theme=slate" />
    <img alt="Pure Rust-UI Starter" src="https://shieldcn.dev/header/graph.svg?title=Pure%20Rust-UI%20Starter&subtitle=Pure%20Rust%20UI%20Starter&logo=rust&mode=light&theme=slate" />
  </picture>
</p>

<p align="center">
  <a href="https://github.com/JumpingJack32/pure-rust-ui-starter/stargazers">
    <img src="https://shieldcn.dev/github/stars/JumpingJack32/pure-rust-ui-starter.svg?variant=secondary" alt="GitHub stars" />
  </a>
  <a href="https://github.com/JumpingJack32/pure-rust-ui-starter/actions">
    <img src="https://shieldcn.dev/github/ci/JumpingJack32/pure-rust-ui-starter.svg?variant=secondary" alt="CI status" />
  </a>
  <a href="https://github.com/JumpingJack32/pure-rust-ui-starter/blob/main/LICENSE">
    <img src="https://shieldcn.dev/github/license/JumpingJack32/pure-rust-ui-starter.svg?variant=secondary" alt="License" />
  </a>
  <a href="https://github.com/JumpingJack32/pure-rust-ui-starter/issues">
    <img src="https://shieldcn.dev/github/issues/JumpingJack32/pure-rust-ui-starter.svg?variant=secondary" alt="GitHub issues" />
  </a>
  <a href="https://github.com/JumpingJack32/pure-rust-ui-starter/commits/main">
    <img src="https://shieldcn.dev/github/last-commit/JumpingJack32/pure-rust-ui-starter.svg?variant=secondary" alt="Last commit" />
  </a>
</p>

# Pure Rust-UI Starter

A clean starting point for building a desktop or native user interface entirely in Rust.

This project is designed to give you a simple, modern foundation for creating a UI-driven Rust application without needing a separate JavaScript or frontend stack.

## Why this starter?

- Pure Rust application development
- Easy to extend for desktop UI workflows
- Minimal setup for experimentation and product prototypes
- Good foundation for a custom native UI app

## Features

- Rust workspace-ready structure
- Simple app entry point
- Build and run pipeline via Cargo
- Easy to customize for your own interface and logic
- Suitable as a starting point for a more advanced Rust UI project

## Prerequisites

Before running the project, make sure you have the following installed:

- Rust and Cargo (stable toolchain)
- A supported operating system for your target platform
- Any platform-specific native development tools required by your chosen UI crate

To confirm Rust is installed:

```bash
rustc --version (rustc 1.99.1 (58cf8f9c3 2026-10-13))
cargo --version (cargo 1.99.1 (5f94df478 2026-08-27))
```

## Quick start

Clone the repository:

```bash
git clone https://github.com/JumpingJack32/pure-rust-ui-starter.git
cd pure-rust-ui-starter
```

## Run the project

```bash
cargo leptos watch  # For Web
```

Build the project:

```bash
cargo build
```

Run the app:

```bash
cargo run
```

## Project structure

```text
.rust-ui-starter/
├── app
│   ├── Cargo.toml
│   └── src
├── Cargo.lock
├── Cargo.toml
├── public
├── README.md
├── server
│   ├── Cargo.toml
│   └── src
├── style
│   ├── tailwind.css
│   └── tailwindcss-animate.css

```

If your project uses a different layout, adjust the structure above to match your actual files.

## Development workflow

During development, you can:

```bash
cargo fmt
cargo check
cargo run
```

This keeps the project clean while quickly validating the application as you build it.

## Extending the starter

This template is meant to be modified. Common next steps include:

- defining your app layout
- adding components and widgets
- wiring up state and user interactions
- integrating services, storage, or configuration
- improving styling and theming

## Contributing

Contributions, ideas, and improvements are welcome.

If you want to improve the starter:

1. Fork the project
2. Create a feature branch
3. Make your changes
4. Run validation commands
5. Open a pull request

## Notes

This repository is a starter template and can be customized for your own product or experiment.

It is intentionally lightweight so that you can build on top of it without unnecessary boilerplate.

## How to use `cargo` to create the UI Library

```bash
cargo new ui_components --lib
```

Cargo will add it automatically to the main Cargo.toml in the root directory :

```toml
[workspace]
resolver = "2"
members = ["app", "server", "ui_components"]
...
```

Then you can move or create your component files:

1. Match the exact version your root toml (e.g., "0.78 or "0.9.0-rc" )
2. Create a button.rs file inside ui_components/src/.
3. Open ui_components/Cargo.toml and add Leptos to its dependencies so it understands your components:

---

## Create app/src/components/avatar.rs

```rust
use leptos::prelude::*;

#[component]
pub fn Avatar(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-800 items-center justify-center";
    let combined = format!("{} {}", base, class);

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
    let combined = format!("{} {}", base, class);

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
    let combined = format!("{} {}", base, class);

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
    let combined = format!("{} {}", base, class);

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
    let combined = format!("{} {}", base, class);

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
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}
```

You can group these related sub-components inside a single module file for clean imports:

### How to Import and Use Avatar

You can import all these sub-components cleanly just like you planned:

> User Profile View

```rust
use crate::components::avatar::Avatar;

#[component]
pub fn UserProfileView() -> impl IntoView {
    view! {
        <div class="flex items-center gap-3">
            // Avatar with an image URL
            <Avatar src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e" alt="John Doe" />

            // Avatar with text fallback initials
            <Avatar fallback="JD" class="bg-indigo-600 text-white" />
        </div>
    }
}
```

> User Section

```rust
use crate::components::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage,
};
use leptos::prelude::*;

#[component]
pub fn UserSection() -> impl IntoView {
    view! {
        <div class="p-6 flex flex-col gap-6">
            // Single Avatar with Image, Fallback, and Badge
            <Avatar>
                <AvatarImage src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e" alt="User" />
                <AvatarFallback>"JD"</AvatarFallback>
                <AvatarBadge class="bg-green-500" />
            </Avatar>

            // Avatar Group
            <AvatarGroup>
                <Avatar>
                    <AvatarImage src="https://images.unsplash.com/photo-1494790108377-be9c29b29330" />
                    <AvatarFallback>"U1"</AvatarFallback>
                </Avatar>
                <Avatar>
                    <AvatarFallback class="bg-indigo-600 text-white">"AB"</AvatarFallback>
                </Avatar>
                <AvatarGroupCount>"+3"</AvatarGroupCount>
            </AvatarGroup>
        </div>
    }
}
```

### Create app/src/components/card.rs

```rust
use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "rounded-xl border border-gray-200 bg-white text-gray-900 shadow-sm dark:border-gray-800 dark:bg-gray-950 dark:text-gray-50";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-col space-y-1.5 p-6";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-lg font-semibold leading-none tracking-tight";
    let combined = format!("{} {}", base, class);

    view! {
        <h3 class=combined>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-sm text-gray-500 dark:text-gray-400";
    let combined = format!("{} {}", base, class);

    view! {
        <p class=combined>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "p-6 pt-0";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex items-center p-6 pt-0";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}
```

How to Import and Use Them
You can import all these sub-components cleanly just like you planned:

```rs
use crate::components::ui::card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
use crate::components::ui::button::Button;
use crate::components::ui::avatar::{Avatar, AvatarImage, AvatarFallback};
use leptos::prelude::*;

#[component]
pub fn UserCardExample() -> impl IntoView {
    view! {
        <Card class="max-w-md">
            <CardHeader>
                <div class="flex items-center gap-4">
                    <Avatar>
                        <AvatarImage src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e" />
                        <AvatarFallback>"JG"</AvatarFallback>
                    </Avatar>
                    <div>
                        <CardTitle>"Gio Gunn"</CardTitle>
                        <CardDescription>"@giogunn"</CardDescription>
                    </div>
                </div>
            </CardHeader>
            <CardContent>
                <p class="text-sm">"Building high-performance Leptos workspaces with Tailwind CSS v4 and custom UI components."</p>
            </CardContent>
            <CardFooter class="justify-end gap-2">
                <Button variant="outline">"Cancel"</Button>
                <Button>"Follow"</Button>
            </CardFooter>
        </Card>
    }
}
```

Here is the complete, modular **Badge** component for your UI library, complete with style variants like default, secondary, outline, and destructive.

### 1. Create `app/src/components/ui/badge.rs`

Create a new file at **`app/src/components/ui/badge.rs`**:

```rust
use leptos::prelude::*;

#[component]
pub fn Badge(
    #[prop(into, default = "default".into())] variant: String,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2";
    
    let variant_styles = match variant.as_str() {
        "secondary" => " border-transparent bg-gray-100 text-gray-900 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-100 dark:hover:bg-gray-700",
        "destructive" => " border-transparent bg-red-600 text-white hover:bg-red-700 dark:bg-red-900 dark:text-red-100",
        "outline" => " text-gray-900 dark:text-gray-100 border-gray-200 dark:border-gray-800",
        _ => " border-transparent bg-indigo-600 text-white hover:bg-indigo-700 dark:bg-indigo-500 dark:hover:bg-indigo-600", // default
    };

    let combined = format!("{}{}{}{}", base, variant_styles, if class.is_empty() { "" } else { " " }, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

```

---

### 2. Update `src/components/ui/mod.rs`

Export the badge module and re-export the `Badge` component:

```rust
pub mod button;
pub mod avatar;
pub mod card;
pub mod badge; // <-- Add this

// Re-exports
pub use button::Button;
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use badge::Badge; // <-- Add this

```

---

### 3. How to Use It in Your Views

You can now drop badges into your landing page components or cards to highlight statuses, tags, or counts:

```rust
use crate::components::ui::badge::Badge;
use leptos::prelude::*;

#[component]
pub fn BadgeShowcase() -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center p-4">
            <Badge>"Default"</Badge>
            <Badge variant="secondary">"Secondary"</Badge>
            <Badge variant="outline">"Outline"</Badge>
            <Badge variant="destructive">"Destructive"</Badge>
        </div>
    }
}

```

Here is the complete, modular **Input** component for your UI library, styled with Tailwind CSS v4 and designed to match standard form inputs.

### 1. Create `app/src/components/ui/input.rs`

Create a new file at **`app/src/components/ui/input.rs`**:

```rust
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
    
    let combined = format!("{} {}", base, class);

    view! {
        <input
            type=r#type
            placeholder=placeholder
            value=value
            disabled=disabled
            class=combined
        />
    }
}

```

---

### 2. Update `src/components/ui/mod.rs` /2

Export the input module and re-export the `Input` component:

```rust
pub mod button;
pub mod avatar;
pub mod card;
pub mod badge;
pub mod input; // <-- Add this

// Re-exports
pub use button::Button;
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use badge::Badge;
pub use input::Input; // <-- Add this

```

---

How to Use It in Your Views

You can now combine your inputs with buttons and cards to create forms, such as a newsletter signup block:

```rust
use crate::components::ui::input::Input;
use crate::components::ui::button::Button;
use leptos::prelude::*;

#[component]
pub fn NewsletterForm() -> impl IntoView {
    view! {
        <div class="flex max-w-sm items-center space-x-2">
            <Input type="email" placeholder="Enter your email..." />
            <Button>"Subscribe"</Button>
        </div>
    }
}

```

Let's tackle these remaining complex UI components in order, starting with **Dropdown Menu**.

Because a dropdown menu requires interactive state management (handling clicks to open/close and tracking selected states), we'll implement it using Leptos signals and event listeners.

### 1. Create `app/src/components/ui/dropdown_menu.rs`

Create a new file at **`app/src/components/ui/dropdown_menu.rs`**:

```rust
use leptos::prelude::*;

#[component]
pub fn DropdownMenu(
    children: Children,
) -> impl IntoView {
    view! {
        <div class="relative inline-block text-left">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("inline-flex w-full justify-center items-center {}", class);
    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuContent(
    #[signal] open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "absolute right-0 z-50 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none dark:bg-gray-950 dark:ring-white/10";
    let combined = format!("{} {}", base, class);

    view! {
        <Show when=move || open.get()>
            <div class=combined>
                <div class="p-1" role="none">
                    {children()}
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(into, default = "".into())] class: String,
    #[prop(into, default = "".into())] on_click: String,
    children: Children,
) -> impl IntoView {
    let base = "block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800 rounded-sm cursor-pointer";
    let combined = format!("{} {}", base, class);

    view! {
        <button type="button" class=combined role="menuitem">
            {children()}
        </button>
    }
}

```

---

### 2. Update `src/components/ui/mod.rs` /dropdown_menu

Export the dropdown module and its elements:

```rust
pub mod button;
pub mod avatar;
pub mod card;
pub mod badge;
pub mod input;
pub mod dropdown_menu; // <-- Add this

// Re-exports
pub use button::Button;
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use badge::Badge;
pub use input::Input;
pub use dropdown_menu::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem}; // <-- Add this

```

---

Here is the complete, modular **Sheet** (slide-over / drawer) component set for your UI library. It supports controlling visibility via a `RwSignal<bool>`, customizable positions (left, right, top, bottom), clean backdrop blur overlays, and compound components (`Sheet`, `SheetContent`, `SheetHeader`, `SheetTitle`, `SheetDescription`, `SheetFooter`).

### 1. Create `app/src/components/ui/sheet.rs`

Create a new file at **`app/src/components/ui/sheet.rs`**:

```rust
use leptos::prelude::*;

#[component]
pub fn Sheet(
    children: Children,
) -> impl IntoView {
    view! {
        <div class="relative">
            {children()}
        </div>
    }
}

#[component]
pub fn SheetContent(
    #[signal] open: RwSignal<bool>,
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
    let combined_content = format!("{} {} {}", base_content, side_styles, class);

    let close_sheet = move |_| open.set(false);

    view! {
        <Show when=move || open.get()>
            // Backdrop Overlay
            <div
                class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm transition-opacity"
                on:click=close_sheet
            />
            
            // Drawer Panel
            <div class=combined_content>
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
        </Show>
    }
}

#[component]
pub fn SheetHeader(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-col space-y-2 text-center sm:text-left";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetTitle(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-lg font-semibold text-gray-950 dark:text-gray-50";
    let combined = format!("{} {}", base, class);

    view! {
        <h2 class=combined>
            {children()}
        </h2>
    }
}

#[component]
pub fn SheetDescription(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-sm text-gray-500 dark:text-gray-400";
    let combined = format!("{} {}", base, class);

    view! {
        <p class=combined>
            {children()}
        </p>
    }
}

#[component]
pub fn SheetFooter(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 mt-auto";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

```

---

### 2. Update `src/components/ui/mod.rs` /Sheet

Export the `sheet` module and re-export all sub-components:

```rust
pub mod button;
pub mod avatar;
pub mod card;
pub mod badge;
pub mod input;
pub mod dropdown_menu;
pub mod navigation_menu;
pub mod sheet; // <-- Add this

// Re-exports
pub use button::Button;
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use badge::Badge;
pub use input::Input;
pub use dropdown_menu::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
pub use navigation_menu::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuLink};
pub use sheet::{Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter}; // <-- Add this

```

---

### 3. Usage Example (e.g., Mobile Navigation Drawer)

Here is how you can pair it with `Button` and `RwSignal`:

```rust
use crate::components::ui::button::Button;
use crate::components::ui::sheet::{Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter};
use leptos::prelude::*;

#[component]
pub fn MobileDrawerExample() -> impl IntoView {
    let open = RwSignal::new(false); // 👈 Usage Example (e.g., Mobile Navigation Drawer) Here is how you can pair it with Button and RwSignal:

    view! {
        <div>
            <Button on:click=move |_| open.set(true)>
                "Open Navigation"
            </Button>

            <Sheet>
                <SheetContent open=open side="left">
                    <SheetHeader>
                        <SheetTitle>"Menu"</SheetTitle>
                        <SheetDescription>"Navigate through the application sections."</SheetDescription>
                    </SheetHeader>
                    <div class="flex flex-col space-y-3 py-4">
                        <a href="#features" class="text-sm font-medium hover:underline">"Features"</a>
                        <a href="#pricing" class="text-sm font-medium hover:underline">"Pricing"</a>
                        <a href="#about" class="text-sm font-medium hover:underline">"About Us"</a>
                    </div>
                    <SheetFooter>
                        <Button variant="outline" on:click=move |_| open.set(false)>
                            "Close Menu"
                        </Button>
                    </SheetFooter>
                </SheetContent>
            </Sheet>
        </div>
    }
}

```

---

Here is the complete, modular **Accordion** component set for your UI library, featuring interactive expand/collapse state management using Leptos signals.

### 1. Create `app/src/components/ui/accordion.rs`

Create a new file at **`app/src/components/ui/accordion.rs`**:

```rust
use leptos::prelude::*;

#[component]
pub fn Accordion(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "divide-y divide-gray-200 dark:divide-gray-800";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "border-b border-gray-200 dark:border-gray-800";
    let combined = format!("{} {}", base, class);

    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTrigger(
    #[signal] open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline text-left text-gray-900 dark:text-gray-100 cursor-pointer";
    let combined = format!("{} {}", base, class);

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
            }>
                "▼"
            </span>
        </button>
    }
}

#[component]
pub fn AccordionContent(
    #[signal] open: RwSignal<bool>,
    #[prop(into, default = "".into())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "pb-4 pt-0 text-sm text-gray-600 dark:text-gray-400";
    let combined = format!("{} {}", base, class);

    view! {
        <Show when=move || open.get()>
            <div class=combined>
                {children()}
            </div>
        </Show>
    }
}

```

---

### 2. Update `src/components/ui/mod.rs` /Accordion

Export the accordion module and re-export its components:

```rust
pub mod button;
pub mod avatar;
pub mod card;
pub mod badge;
pub mod input;
pub mod dropdown_menu;
pub mod navigation_menu;
pub mod sheet;
pub mod accordion; // <-- Add this

// Re-exports
pub use button::Button;
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use badge::Badge;
pub use input::Input;
pub use dropdown_menu::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
pub use navigation_menu::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuLink};
pub use sheet::{Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter};
pub use accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent}; // <-- Add this

```

---

### 3. Usage Example (e.g., FAQ Section)

```rust
use crate::components::ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use leptos::prelude::*;

#[component]
pub fn FaqAccordionExample() -> impl IntoView {
    let item1_open = RwSignal::new(false);
    let item2_open = RwSignal::new(false);

    view! {
        <Accordion class="max-w-xl mx-auto">
            <AccordionItem>
                <AccordionTrigger open=item1_open>"Is this compatible with Leptos 0.9?"</AccordionTrigger>
                <AccordionContent open=item1_open>
                    "Yes! This entire UI component library is natively built for Leptos 0.9 using the new prelude and signal system."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger open=item2_open>"How do I customize the styling?"</AccordionTrigger>
                <AccordionContent open=item2_open>
                    "All components accept a `class` prop that merges cleanly with the default Tailwind CSS v4 styling classes."
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}

```

---

### 🎉 All 9 Base UI Components Completed

You now have the complete foundational UI suite (`button`, `avatar`, `card`, `badge`, `input`, `dropdown_menu`, `navigation_menu`, `sheet`, and `accordion`).
