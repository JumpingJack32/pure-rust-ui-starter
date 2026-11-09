use leptos::prelude::*;
use ui_components::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn AboutCompany() -> impl IntoView {
    view! {
        <section id="about" class="py-20 md:py-28 bg-white dark:bg-gray-950">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    // --- Left Column: Story / Text ---
                    <div>
                        <h2 class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 mb-3">
                            "About the Company"
                        </h2>
                        <h3 class="text-3xl font-bold tracking-tight sm:text-4xl text-[#0F4761] dark:text-white mb-6">
                            "Pioneering high-performance, type-safe web architecture."
                        </h3>
                        <p class="text-gray-600 dark:text-gray-300 leading-relaxed mb-6">
                            "We are dedicated to bridging the gap between blazing-fast systems programming and modern reactive web development. By harnessing the full power of Rust, WebAssembly, and Leptos, our team builds toolchains and components that scale effortlessly."
                        </p>
                        <p class="text-gray-600 dark:text-gray-300 leading-relaxed">
                            "Our mission is to empower developers worldwide to create secure, accessible, and ultra-responsive applications with zero compromise on developer experience."
                        </p>
                    </div>

                    // --- Right Column: Stats / Highlights ---
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <Card class="border-gray-200 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50">
                            <CardHeader>
                                <CardTitle class="text-3xl font-extrabold text-indigo-600 dark:text-indigo-400">
                                    "100% "
                                </CardTitle>
                                <CardDescription class="font-semibold text-gray-900 dark:text-gray-100">
                                    "Rust Native"
                                </CardDescription>
                            </CardHeader>
                            <CardContent>
                                <p class="text-sm text-gray-500 dark:text-gray-400">
                                    "Compiled directly to optimized WebAssembly binaries."
                                </p>
                            </CardContent>
                        </Card>

                        <Card class="border-gray-200 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50">
                            <CardHeader>
                                <CardTitle class="text-3xl font-extrabold text-indigo-600 dark:text-indigo-400">
                                    "0.9ms"
                                </CardTitle>
                                <CardDescription class="font-semibold text-gray-900 dark:text-gray-100">
                                    "Render Latency"
                                </CardDescription>
                            </CardHeader>
                            <CardContent>
                                <p class="text-sm text-gray-500 dark:text-gray-400">
                                    "Fine-grained reactivity without virtual DOM overhead."
                                </p>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </div>
        </section>
    }
}
