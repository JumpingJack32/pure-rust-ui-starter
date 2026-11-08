use ui_components::ModeToggle;
use ui_components::Button;
use ui_components::{NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList};
use ui_components::{Sheet, SheetContent, SheetHeader, SheetTitle};
use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let is_mobile_open = RwSignal::new(false);

    let nav_items = vec![
        ("Features", "#features"),
        ("Testimonials", "#testimonials"),
        ("Pricing", "#pricing"),
        ("FAQ", "#faq"),
    ];

    let desktop_nav = nav_items
        .iter()
        .map(|&(label, href)| {
            view! {
                <NavigationMenuItem>
                    <NavigationMenuLink href=href>{label}</NavigationMenuLink>
                </NavigationMenuItem>
            }
        })
        .collect_view();

    let mobile_nav = nav_items
        .iter()
        .map(|&(label, href)| {
            let close_sheet = move |_| is_mobile_open.set(false);
            view! {
                <a
                    href=href
                    on:click=close_sheet
                    class="text-base font-medium text-gray-700 hover:text-indigo-600 dark:text-gray-300 dark:hover:text-indigo-400 transition-colors"
                >
                    {label}
                </a>
            }
        })
        .collect_view();

    view! {
        <header class="sticky top-0 z-40 w-full border-b border-gray-200 bg-white/80 backdrop-blur-md dark:border-gray-800 dark:bg-gray-950/80">
            <div class="container mx-auto flex h-16 items-center justify-between px-4 sm:px-8">
                // --- Brand Logo ---
                <a href="#" class="flex items-center gap-2 font-bold text-lg tracking-tight">
                    <LogoIcon />
                    <span class="bg-linear-to-r from-indigo-500 to-purple-600 bg-clip-text text-transparent">
                        "LeptosApp"
                    </span>
                </a>

                // --- Desktop Navigation Menu ---
                <div class="hidden md:flex items-center gap-6">
                    <NavigationMenu>
                        <NavigationMenuList>
                            {desktop_nav}
                        </NavigationMenuList>
                    </NavigationMenu>
                </div>

                // --- Right Side Actions (Desktop & Mobile) ---
                <div class="flex items-center gap-3">
                    <ModeToggle />

                    <div class="hidden md:flex items-center gap-2">
                        <Button variant="ghost">"Sign In"</Button>
                        <Button variant="outline">"Get Started"</Button>
                    </div>

                    // --- Mobile Menu Trigger ---
                    <div class="md:hidden">
                        <Button
                            variant="ghost"
                            class="px-2"
                            on:click=move |_| is_mobile_open.set(true)
                        >
                            <MenuIcon />
                            <span class="sr-only">"Toggle Menu"</span>
                        </Button>
                    </div>
                </div>
            </div>

            // --- Mobile Drawer Sheet ---
            <Sheet>
                <SheetContent open=is_mobile_open side="right">
                    <SheetHeader>
                        <SheetTitle class="text-left flex items-center gap-2">
                            <LogoIcon />
                            <span>"LeptosApp"</span>
                        </SheetTitle>
                    </SheetHeader>

                    <nav class="flex flex-col gap-4 py-6">
                        {mobile_nav}
                    </nav>

                    <div class="flex flex-col gap-3 mt-auto pt-6 border-t border-gray-200 dark:border-gray-800">
                        <Button variant="outline" class="w-full">
                            "Sign In"
                        </Button>
                        <Button class="w-full">"Get Started"</Button>
                    </div>
                </SheetContent>
            </Sheet>
        </header>
    }
}

// --- Inline SVG Icons (Lucide-compatible) ---

#[component]
fn LogoIcon() -> impl IntoView {
    view! {
        <svg
            class="h-6 w-6 text-indigo-600 dark:text-indigo-400"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M12 2L2 7l10 5 10-5-10-5z" />
            <path d="M2 17l10 5 10-5" />
            <path d="M2 12l10 5 10-5" />
        </svg>
    }
}

#[component]
fn MenuIcon() -> impl IntoView {
    view! {
        <svg
            class="h-6 w-6"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
            />
        </svg>
    }
}
