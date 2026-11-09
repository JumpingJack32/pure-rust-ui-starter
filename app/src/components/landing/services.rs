use leptos::prelude::*;
use ui_components::Card;

#[component]
pub fn ServicesSection() -> impl IntoView {
    let services = vec![
        (
            "Code Collaboration",
            "Real-time code reviews, branch synchronization, and shared development environments designed for high-velocity teams.",
            "💻",
        ),
        (
            "Project Management",
            "Streamlined milestones, kanban boards, and progress tracking tailored for Rust and Leptos workspaces.",
            "📊",
        ),
        (
            "Task Automation",
            "Automated CI/CD pipelines, test runners, and deployment scripts to eliminate routine manual work.",
            "⚙️",
        ),
    ];

    view! {
        <section class="py-24 bg-white dark:bg-gray-950 border-t border-gray-200 dark:border-gray-800">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    // --- Left Column: Client-Centric Services ---
                    <div class="flex flex-col space-y-6">
                        <div>
                            <h2 class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 mb-3">
                                "Tailored Solutions"
                            </h2>
                            <h3 class="text-3xl font-bold tracking-tight sm:text-4xl text-[#0F4761] dark:text-white mb-4">
                                "Client-Centric Services"
                            </h3>
                            <p class="text-gray-600 dark:text-gray-300">
                                "Empowering your organization with robust tools built to accelerate delivery and foster seamless teamwork."
                            </p>
                        </div>

                        <div class="space-y-4">
                            {services
                                .into_iter()
                                .map(|(title, desc, icon)| {
                                    view! {
                                        <Card class="border-gray-200 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50 p-4 transition-all hover:border-indigo-500/50">
                                            <div class="flex items-start gap-4">
                                                <div class="shrink-0 h-10 w-10 rounded-lg bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 flex items-center justify-center text-lg">
                                                    {icon}
                                                </div>
                                                <div>
                                                    <h4 class="font-semibold text-gray-900 dark:text-white text-base mb-1">
                                                        {title}
                                                    </h4>
                                                    <p class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
                                                        {desc}
                                                    </p>
                                                </div>
                                            </div>
                                        </Card>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>

                    // --- Right Column: Management Collaboration Image ---
                    <div class="relative mt-26 rounded-2xl overflow-hidden border border-gray-200 dark:border-gray-800 shadow-2xl bg-gray-900">
                        <img
                            src="https://encrypted-tbn3.gstatic.com/licensed-image?q=tbn:ANd9GcSeEuPKINF5AGP7kUH9NU5n_ooUiYUzFdpcLSz0w-6W3tuCgpSJigGRLhcdTGdJnh0SD3TSiUh5voBLq4c"
                            alt="Happy business team collaborating in a modern office meeting around a conference table"
                            class="w-full h-auto object-cover aspect-video hover:scale-105 transition-transform duration-500"
                        />
                        <div class="absolute inset-0 bg-linear-to-t from-black/60 via-transparent to-transparent flex items-end p-6">
                            <p class="text-white text-sm font-medium drop-shadow-md">
                                "Team collaboration and project management in action"
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
