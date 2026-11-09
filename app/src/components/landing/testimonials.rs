use leptos::prelude::*;
use ui_components::Card;

#[component]
pub fn TestimonialsSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-gray-50/50 dark:bg-gray-900/50 border-t border-gray-200 dark:border-gray-800">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center max-w-3xl mx-auto mb-16">
                    <h2 class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 mb-3">
                        "Testimonials"
                    </h2>
                    <h3 class="text-3xl font-bold tracking-tight sm:text-4xl text-[#0F4761] dark:text-white">
                        "Discover More About This Landing Page"
                    </h3>
                    <p class="mt-4 text-gray-600 dark:text-gray-300">
                        "See what engineers and creators are saying about building with Leptos and our custom UI primitives."
                    </p>
                </div>

                // --- 6-Card Bento Grid ---
                <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-6">

                    // Card 1 (Large span)
                    <Card class="md:col-span-2 border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-6 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center gap-3 mb-4">
                                <div class="h-10 w-10 rounded-full bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 flex items-center font-bold justify-center">
                                    "JD"
                                </div>
                                <div>
                                    <h4 class="font-semibold text-gray-900 dark:text-white text-sm">
                                        "Jessica Davis"
                                    </h4>
                                    <p class="text-xs text-gray-500">
                                        "Principal Architect at CloudScale"
                                    </p>
                                </div>
                            </div>
                            <p class="text-gray-700 dark:text-gray-300 text-lg font-medium leading-relaxed">
                                "\"The performance boost moving to Leptos 0.9 was immediate. Server-side rendering combined with WASM reactivity is a absolute game changer for our enterprise workflows.\""
                            </p>
                        </div>
                        <div class="mt-6 pt-4 border-t border-gray-100 dark:border-gray-900 text-xs text-indigo-600 dark:text-indigo-400 font-semibold">
                            "Verified Enterprise Review"
                        </div>
                    </Card>

                    // Card 2
                    <Card class="border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-6 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center gap-3 mb-4">
                                <div class="h-8 w-8 rounded-full bg-purple-500/10 text-purple-600 dark:text-purple-400 flex items-center font-bold text-xs justify-center">
                                    "AM"
                                </div>
                                <div>
                                    <h4 class="font-semibold text-gray-900 dark:text-white text-xs">
                                        "Alex Miller"
                                    </h4>
                                    <p class="text-[10px] text-gray-500">"Full-Stack Rust Dev"</p>
                                </div>
                            </div>
                            <p class="text-gray-600 dark:text-gray-400 text-sm">
                                "\"Having clean shadcn-inspired primitives in Rust saved us months of frontend styling work. Love it!\""
                            </p>
                        </div>
                    </Card>

                    // Card 3
                    <Card class="border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-6 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center gap-3 mb-4">
                                <div class="h-8 w-8 rounded-full bg-pink-500/10 text-pink-600 dark:text-pink-400 flex items-center font-bold text-xs justify-center">
                                    "SK"
                                </div>
                                <div>
                                    <h4 class="font-semibold text-gray-900 dark:text-white text-xs">
                                        "Sarah Khan"
                                    </h4>
                                    <p class="text-[10px] text-gray-500">"UI/UX Lead"</p>
                                </div>
                            </div>
                            <p class="text-gray-600 dark:text-gray-400 text-sm">
                                "\"Tailwind CSS v4 integration is seamless. The dark mode transitions are silky smooth.\""
                            </p>
                        </div>
                    </Card>

                    // Card 4
                    <Card class="border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-6 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center gap-3 mb-4">
                                <div class="h-8 w-8 rounded-full bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 flex items-center font-bold text-xs justify-center">
                                    "MR"
                                </div>
                                <div>
                                    <h4 class="font-semibold text-gray-900 dark:text-white text-xs">
                                        "Marcus Ross"
                                    </h4>
                                    <p class="text-[10px] text-gray-500">"Startup Founder"</p>
                                </div>
                            </div>
                            <p class="text-gray-600 dark:text-gray-400 text-sm">
                                "\"We launched our MVP in 3 days using this starter kit. Incredible developer experience.\""
                            </p>
                        </div>
                    </Card>

                    // Card 5 (Large span)
                    <Card class="md:col-span-2 lg:col-span-2 border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-6 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center gap-3 mb-4">
                                <div class="h-10 w-10 rounded-full bg-amber-500/10 text-amber-600 dark:text-amber-400 flex items-center font-bold justify-center">
                                    "EL"
                                </div>
                                <div>
                                    <h4 class="font-semibold text-gray-900 dark:text-white text-sm">
                                        "Elena Larson"
                                    </h4>
                                    <p class="text-xs text-gray-500">"Lead Systems Engineer"</p>
                                </div>
                            </div>
                            <p class="text-gray-700 dark:text-gray-300 text-sm leading-relaxed">
                                "\"Type safety all the way from the database schema down to the browser DOM nodes eliminates entire classes of runtime bugs. Highly recommended!\""
                            </p>
                        </div>
                    </Card>

                    // Card 6
                    <Card class="border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-6 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center gap-3 mb-4">
                                <div class="h-8 w-8 rounded-full bg-blue-500/10 text-blue-600 dark:text-blue-400 flex items-center font-bold text-xs justify-center">
                                    "TN"
                                </div>
                                <div>
                                    <h4 class="font-semibold text-gray-900 dark:text-white text-xs">
                                        "Tom Nelson"
                                    </h4>
                                    <p class="text-[10px] text-gray-500">
                                        "Open Source Contributor"
                                    </p>
                                </div>
                            </div>
                            <p class="text-gray-600 dark:text-gray-400 text-sm">
                                "\"The component structure with mod.rs and clean trait bounds is top-notch Rust code.\""
                            </p>
                        </div>
                    </Card>

                </div>
            </div>
        </section>
    }
}
