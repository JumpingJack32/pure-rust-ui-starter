use leptos::prelude::*;
use ui_components::{Button, ButtonVariant};

#[component]
pub fn CtaSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-linear-to-b from-white to-gray-50 dark:from-gray-950 dark:to-gray-900 border-t border-gray-200 dark:border-gray-800">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <div class="rounded-3xl bg-indigo-600 dark:bg-indigo-950/60 border border-indigo-500/30 p-8 sm:p-16 text-center shadow-2xl relative overflow-hidden">
                    // Background glow effects
                    <div class="absolute -top-24 -left-24 w-96 h-96 bg-purple-500/20 rounded-full blur-3xl pointer-events-none" />
                    <div class="absolute -bottom-24 -right-24 w-96 h-96 bg-indigo-400/20 rounded-full blur-3xl pointer-events-none" />

                    <div class="relative z-10 max-w-3xl mx-auto">
                        <h2 class="text-3xl font-extrabold tracking-tight sm:text-5xl text-white mb-6 leading-tight">
                            "All Your Ideas & Concepts In One Interface"
                        </h2>
                        <p class="text-indigo-100 text-base sm:text-lg mb-10 max-w-2xl mx-auto">
                            "Experience the full power of type-safe Rust web development with modular components designed for high-performance scale."
                        </p>
                        <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
                            <Button class="w-full sm:w-auto h-12 px-8 text-base shadow-lg">
                                "Request a Demo"
                            </Button>
                            <Button
                                variant=ButtonVariant::Outline
                                class="w-full sm:w-auto h-12 px-8 text-base"
                            >
                                "View all Features"
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
