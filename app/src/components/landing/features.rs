use leptos::prelude::*;
use ui_components::Badge;
use ui_components::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn FeaturesSection() -> impl IntoView {
    let how_it_works_steps = vec![
        (
            "01",
            "Accessibility",
            "Built from the ground up to support ARIA standards, keyboard navigation, and high-contrast themes.",
        ),
        (
            "02",
            "Community",
            "Join thousands of developers sharing components, templates, and full-stack Leptos patterns.",
        ),
        (
            "03",
            "Scalability",
            "Engineered for high performance with server-side rendering (SSR) and fine-grained WASM reactivity.",
        ),
        (
            "04",
            "Gamification",
            "Earn badges, track component contributions, and level up your full-stack Rust workflow.",
        ),
    ];

    let pill_features = vec![
        "Dark/Light theme",
        "Reviews",
        "Features",
        "Pricing",
        "Contact form",
        "Our team",
        "Responsive design",
        "Newsletter",
        "Minimalist",
    ];

    let showcase_cards = vec![
        (
            "Responsive Design",
            "Optimized layout engines that adapt fluidly across mobile, tablet, and desktop viewports.",
        ),
        (
            "Intuitive User Interface",
            "Clean shadcn-inspired aesthetics paired with reactive Leptos state management.",
        ),
        (
            "AI-Powered Insights",
            "Smart code suggestions and automated performance tracking built right into your build loop.",
        ),
    ];

    view! {
        <section
            id="features"
            class="py-24 bg-gray-50/50 dark:bg-gray-900/50 border-t border-gray-200 dark:border-gray-800"
        >
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">

                // --- Part 1: How It Works Step-by-Step Guide ---
                <div class="text-center max-w-3xl mx-auto mb-16">
                    <h2 class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 mb-3">
                        "Step-by-Step Guide"
                    </h2>
                    <h3 class="text-3xl font-bold tracking-tight sm:text-4xl text-[#0F4761] dark:text-white">
                        "How It Works"
                    </h3>
                    <p class="mt-4 text-gray-600 dark:text-gray-300">
                        "From zero to production-ready Leptos application in four simple architectural stages."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-24">
                    {how_it_works_steps
                        .into_iter()
                        .map(|(num, title, desc)| {
                            view! {
                                <Card class="border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 relative overflow-hidden">
                                    <CardHeader>
                                        <span class="text-4xl font-extrabold text-indigo-500/20 dark:text-indigo-400/20 font-mono mb-2">
                                            {num}
                                        </span>
                                        <CardTitle class="text-xl text-gray-900 dark:text-white">
                                            {title}
                                        </CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <CardDescription class="text-gray-600 dark:text-gray-400 leading-relaxed">
                                            {desc}
                                        </CardDescription>
                                    </CardContent>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>

                // --- Part 2: Many Great Features with Pill Buttons ---
                <div class="text-center max-w-3xl mx-auto mb-12">
                    <h3 class="text-4xl font-bold tracking-tight text-[#0F4761] dark:text-white mb-4">
                        "Many Great Features"
                    </h3>
                    <p class="text-gray-600 dark:text-gray-300 text-sm sm:text-base mb-8">
                        "Everything you need to ship world-class web apps out of the box."
                    </p>
                    <div class="flex flex-wrap items-center justify-center gap-2">
                        {pill_features
                            .into_iter()
                            .map(|pill| {
                                view! {
                                    <Badge
                                        variant="outline"
                                        class="px-4 py-2 text-sm font-medium bg-white dark:bg-gray-950 border-gray-200 dark:border-gray-800 text-gray-700 dark:text-gray-300 hover:border-indigo-500 transition-colors"
                                    >
                                        {pill}
                                    </Badge>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                <div class="my-24 border-t border-gray-200 dark:border-gray-800" />

                // --- Part 3: Three-Card Grid with Title and Image Sections ---
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    {showcase_cards
                        .into_iter()
                        .map(|(title, desc)| {
                            view! {
                                <Card class="border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 overflow-hidden flex flex-col">
                                    // Image / Graphic placeholder section
                                    <div class="w-full h-48 bg-linear-to-br from-indigo-500/10 via-purple-500/10 to-pink-500/10 dark:from-indigo-950/40 dark:to-purple-950/40 border-b border-gray-200 dark:border-gray-800 flex items-center justify-center p-6">
                                        <div class="h-10 w-10 rounded-lg bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 flex items-center justify-center font-bold">
                                            "⚡"
                                        </div>
                                    </div>
                                    <CardHeader>
                                        <CardTitle class="text-xl text-gray-900 dark:text-white">
                                            {title}
                                        </CardTitle>
                                    </CardHeader>
                                    <CardContent class="mt-auto">
                                        <CardDescription class="text-gray-600 dark:text-gray-400 leading-relaxed">
                                            {desc}
                                        </CardDescription>
                                    </CardContent>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>

            </div>
        </section>
    }
}
