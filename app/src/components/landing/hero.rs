// use crate::HeroCard;
use leptos::prelude::*;
use ui_components::Badge;
use ui_components::{Button, ButtonSize, ButtonVariant, button_variants};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden py-20 md:py-32 bg-linear-to-b from-gray-50/50 to-white dark:from-gray-950 dark:to-gray-900">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <div class="mx-auto max-w-4xl text-center">
                    // --- Announcement Badge ---
                    <div class="mb-6 flex justify-center">
                        <Badge
                            variant="outline"
                            class="gap-2 py-1 px-3 text-xs sm:text-sm backdrop-blur-md"
                        >
                            <span class="font-semibold text-indigo-600 dark:text-indigo-400">
                                "v0.9.0-rc1 Released"
                            </span>
                            <span class="text-gray-400">"•"</span>
                            <span class="text-gray-600 dark:text-gray-300">
                                "Explore the new features"
                            </span>
                            <ArrowRightIcon />
                        </Badge>
                    </div>

                    // --- Hero Title ---
                    <h1 class="text-4xl font-extrabold tracking-tight sm:text-6xl md:text-7xl text-[#0F4761] dark:text-white">
                        "Build Full-Stack Apps with "
                        <span class="bg-linear-to-r from-indigo-500 via-purple-500 to-pink-500 bg-clip-text text-transparent">
                            "Leptos & Rust"
                        </span>
                    </h1>

                    // --- Subtitle Description ---
                    <p class="mt-6 text-lg sm:text-xl text-gray-600 dark:text-gray-300 leading-relaxed max-w-2xl mx-auto">
                        "A blazingly fast, type-safe framework for modern web applications. Powered by Tailwind CSS v4 and Axum 0.9.0-rc backend integration."
                    </p>

                    // --- CTA Action Buttons ---
                    <div class="mt-8 flex flex-col sm:flex-row items-center justify-center gap-4">
                        <Button class="w-full sm:w-auto h-12 px-8 text-base shadow-lg shadow-indigo-500/25">
                            "Get Started Free"
                        </Button>
                        <a
                            target="_blank"
                            href="https://github.com/JumpingJack32/pure-rust-ui-starter"
                            class=button_variants(
                                ButtonVariant::Outline,
                                ButtonSize::Lg,
                                "w-full sm:w-auto h-12 px-8 text-base",
                            )
                        >
                            <GithubIcon class="mr-2 h-5 w-5" />
                            "View on GitHub"
                        </a>
                    </div>

                    // --- Social Proof / Stats Summary ---
                    <div class="mt-12 flex items-center justify-center gap-8 text-sm text-gray-500 dark:text-gray-400">
                        <div class="flex items-center gap-2">
                            <CheckIcon />
                            <span>"Zero Javascript Overhead"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <CheckIcon />
                            <span>"WASM Native"</span>
                        </div>
                    </div>
                </div>

                // --- Hero Product Preview Card / Showcase ---
                <div class="mt-16 relative mx-auto max-w-5xl rounded-2xl border border-gray-200 bg-white/50 p-2 backdrop-blur-xl dark:border-gray-800 dark:bg-gray-950/50 shadow-2xl">
                    <div class="rounded-xl overflow-hidden border border-gray-200 dark:border-gray-800 bg-gray-900 aspect-video flex items-center justify-center">
                        <div class="text-center p-8">
                            <div class="inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-indigo-500/10 text-indigo-400 mb-4">
                                <TerminalIcon />
                            </div>
                            <p class="font-mono text-sm text-indigo-400 mb-2">
                                "cargo leptos watch"
                            </p>
                            <p class="text-gray-400 text-xs sm:text-sm">
                                "Compiling client WASM & server binary in sub-seconds..."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

// --- Icons ---

#[component]
fn ArrowRightIcon() -> impl IntoView {
    view! {
        <svg
            class="h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3"
            />
        </svg>
    }
}

#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg
            class="h-4 w-4 text-green-500"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2.5"
        >
            <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
        </svg>
    }
}

#[component]
fn GithubIcon(#[prop(into, default = "".into())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24">
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
            />
        </svg>
    }
}

#[component]
fn TerminalIcon() -> impl IntoView {
    view! {
        <svg
            class="h-8 w-8"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z"
            />
        </svg>
    }
}
