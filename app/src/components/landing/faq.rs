use leptos::prelude::*;
use ui_components::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn FaqSection() -> impl IntoView {
    let item1 = RwSignal::new(false);
    let item2 = RwSignal::new(false);
    let item3 = RwSignal::new(false);
    let item4 = RwSignal::new(false);
    let item5 = RwSignal::new(false);

    view! {
        <section
            id="faq"
            class="py-24 bg-white dark:bg-gray-950 border-t border-gray-200 dark:border-gray-800"
        >
            <div class="container mx-auto px-4 sm:px-6 lg:px-8 max-w-4xl">

                <div class="text-center mb-16">
                    <h2 class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 mb-3">
                        "Got Questions?"
                    </h2>
                    <h3 class="text-3xl font-bold tracking-tight sm:text-4xl text-[#0F4761] dark:text-white">
                        "Frequently Asked Questions"
                    </h3>
                    <p class="mt-4 text-gray-600 dark:text-gray-300">
                        "Everything you need to know about getting started with our Leptos UI starter kit."
                    </p>
                </div>

                <Accordion class="w-full">
                    <AccordionItem>
                        <AccordionTrigger open=item1 class="text-base sm:text-lg font-semibold">
                            "Is this starter kit compatible with Leptos 0.9.0-rc1?"
                        </AccordionTrigger>
                        <AccordionContent
                            open=item1
                            class="text-gray-600 dark:text-gray-400 leading-relaxed"
                        >
                            "Yes, every component and module in this library is written specifically for Leptos 0.9.0-rc1 using the updated prelude, RwSignal patterns, and fine-grained reactivity system."
                        </AccordionContent>
                    </AccordionItem>

                    <AccordionItem>
                        <AccordionTrigger open=item2 class="text-base sm:text-lg font-semibold">
                            "How does Tailwind CSS v4 styling work with Rust components?"
                        </AccordionTrigger>
                        <AccordionContent
                            open=item2
                            class="text-gray-600 dark:text-gray-400 leading-relaxed"
                        >
                            "Tailwind CSS v4 processes class names directly from your Rust source files. You can pass custom utility classes via the `class` prop on any UI component to override or extend styles dynamically."
                        </AccordionContent>
                    </AccordionItem>

                    <AccordionItem>
                        <AccordionTrigger open=item3 class="text-base sm:text-lg font-semibold">
                            "Can I use Axum 0.9.0-rc as the backend framework?"
                        </AccordionTrigger>
                        <AccordionContent
                            open=item3
                            class="text-gray-600 dark:text-gray-400 leading-relaxed"
                        >
                            "Absolutely. Leptos integrates smoothly with Axum 0.89 for full SSR (Server-Side Rendering), server functions, and REST API routing within a single Rust workspace. "
                            "We are showcasing Axum 0.9.0-rc; as a release candidate, it is expected to become the stable release imminently."
                        </AccordionContent>
                    </AccordionItem>

                    <AccordionItem>
                        <AccordionTrigger open=item4 class="text-base sm:text-lg font-semibold">
                            "Is dark mode support built into all UI components?"
                        </AccordionTrigger>
                        <AccordionContent
                            open=item4
                            class="text-gray-600 dark:text-gray-400 leading-relaxed"
                        >
                            "Yes! All primitives support Tailwind dark variant modifiers. The integrated ModeToggle and ThemeProvider handle system preference detection and smooth theme switching."
                        </AccordionContent>
                    </AccordionItem>

                    <AccordionItem>
                        <AccordionTrigger open=item5 class="text-base sm:text-lg font-semibold">
                            "How do I customize or add new components to the library?"
                        </AccordionTrigger>
                        <AccordionContent
                            open=item5
                            class="text-gray-600 dark:text-gray-400 leading-relaxed"
                        >
                            "Global UI components are organised within ui_components/src/components/ui/ using modular Rust files and explicit re-exports in mod.rs. Then imported into either the app/ or server/ directories to be comsumed by local components. These components may be duplicated or modified independently, without reliance on external npm packages or binary dependencies."
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>

            </div>
        </section>
    }
}
