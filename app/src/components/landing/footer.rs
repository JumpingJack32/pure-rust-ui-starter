use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let follow_us_links = vec![
        (
            "Github",
            "https://github.com/JumpingJack32/pure-rust-ui-starter",
        ),
        ("Twitter", "#"),
        ("Dribbble", "#"),
    ];

    let platforms_links = vec![("Web", "#"), ("Mobile", "#"), ("Desktop", "#")];

    let about_links = vec![
        ("Features", "#features"),
        ("Pricing", "#pricing"),
        ("FAQ", "#faq"),
    ];

    let community_links = vec![("Youtube", "#"), ("Discord", "#"), ("Twitch", "#")];

    view! {
        <footer class="border-t border-gray-200 bg-white dark:border-gray-800 dark:bg-gray-950 text-gray-900 dark:text-gray-50">
            <div class="container mx-auto px-4 py-12 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 gap-8 md:grid-cols-5">

                    // --- Brand Logo Section ---
                    <div class="md:col-span-1 flex flex-col justify-between">
                        <div>
                            <a
                                href="#"
                                class="flex items-center gap-2 font-bold text-lg tracking-tight"
                            >
                                <LogoIcon />
                                <span class="font-bold text-gray-900 dark:text-white">
                                    "Rust don't "
                                    <span class="line-through text-red-500 font-semibold decoration-2 decoration-red-500">
                                        "React"
                                    </span>
                                </span>
                            </a>
                            <p class="mt-4 text-xs text-gray-500 dark:text-gray-400 leading-relaxed">
                                "Blazingly fast, type-safe full-stack web applications powered by Leptos and Rust."
                            </p>
                        </div>
                    </div>

                    // --- Links Columns ---
                    <div class="grid grid-cols-2 gap-8 md:col-span-4 sm:grid-cols-4">

                        // Follow Us
                        <div>
                            <h3 class="text-xs font-semibold uppercase tracking-wider text-[#0F4761] dark:text-white mb-4">
                                "Follow Us"
                            </h3>
                            <ul class="space-y-3 text-sm">
                                {follow_us_links
                                    .into_iter()
                                    .map(|(label, href)| {
                                        view! {
                                            <li>
                                                <a
                                                    href=href
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="text-gray-600 hover:text-indigo-600 dark:text-gray-400 dark:hover:text-indigo-400 transition-colors"
                                                >
                                                    {label}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>

                        // Platforms
                        <div>
                            <h3 class="text-xs font-semibold uppercase tracking-wider text-[#0F4761] dark:text-white mb-4">
                                "Platforms"
                            </h3>
                            <ul class="space-y-3 text-sm">
                                {platforms_links
                                    .into_iter()
                                    .map(|(label, href)| {
                                        view! {
                                            <li>
                                                <a
                                                    href=href
                                                    class="text-gray-600 hover:text-indigo-600 dark:text-gray-400 dark:hover:text-indigo-400 transition-colors"
                                                >
                                                    {label}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>

                        // About
                        <div>
                            <h3 class="text-xs font-semibold uppercase tracking-wider text-[#0F4761] dark:text-white mb-4">
                                "About"
                            </h3>
                            <ul class="space-y-3 text-sm">
                                {about_links
                                    .into_iter()
                                    .map(|(label, href)| {
                                        view! {
                                            <li>
                                                <a
                                                    href=href
                                                    class="text-gray-600 hover:text-indigo-600 dark:text-gray-400 dark:hover:text-indigo-400 transition-colors"
                                                >
                                                    {label}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>

                        // Community
                        <div>
                            <h3 class="text-xs font-semibold uppercase tracking-wider text-[#0F4761] dark:text-white mb-4">
                                "Community"
                            </h3>
                            <ul class="space-y-3 text-sm">
                                {community_links
                                    .into_iter()
                                    .map(|(label, href)| {
                                        view! {
                                            <li>
                                                <a
                                                    href=href
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="text-gray-600 hover:text-indigo-600 dark:text-gray-400 dark:hover:text-indigo-400 transition-colors"
                                                >
                                                    {label}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>

                    </div>
                </div>

                // --- Bottom Bar / Copyright ---
                <div class="mt-12 pt-8 border-t border-gray-200 dark:border-gray-800 flex flex-col sm:flex-row items-center justify-between text-xs text-gray-500 dark:text-gray-400 gap-4">
                    <p>"© 2026 Pure Rust UI Starter. All rights reserved."</p>
                    <p class="flex items-center gap-1">"Built with Rust 🦀 & Leptos 0.9"</p>
                </div>
            </div>
        </footer>
    }
}

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
