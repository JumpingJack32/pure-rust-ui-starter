use leptos::prelude::*;

#[component]
pub fn Sponsors() -> impl IntoView {
    let sponsors = vec![
        ("Acme Corp", "ACME"),
        ("Global Tech", "GLOBAL"),
        ("Apex Systems", "APEX"),
        ("Nexus Wave", "NEXUS"),
        ("Vanguard AI", "VANGUARD"),
    ];

    view! {
        <section class="container mx-auto py-12 border-y border-gray-200 bg-gray-50/50 dark:border-gray-800 dark:bg-gray-900/50">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <p class="text-center text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-gray-400 mb-8">
                    "Trusted by engineering teams and innovators worldwide"
                </p>
                <div class="grid grid-cols-2 gap-8 md:grid-cols-5 items-center justify-center opacity-70 dark:opacity-60">
                    {sponsors
                        .into_iter()
                        .map(|(_name, logo_text)| {
                            view! {
                                <div class="flex justify-center-safe  items-center ">
                                    <span class="cursor-pointer font-bold text-lg tracking-widest text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                                        {logo_text}
                                    </span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
