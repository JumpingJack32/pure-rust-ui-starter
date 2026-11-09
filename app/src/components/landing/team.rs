use leptos::prelude::*;
use ui_components::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn TeamSection() -> impl IntoView {
    let team_members = vec![
        (
            "Alex Morgan",
            "Product Manager",
            "Drives product vision, user research, and roadmap strategy across all Leptos platform initiatives.",
            "https://images.unsplash.com/photo-1534528741775-53994a69daeb?auto=format&fit=crop&w=600&q=80",
            "#",
            "#",
            "#",
        ),
        (
            "David Chen",
            "Tech Lead",
            "Architects core system infrastructure, compile-time type safety, and Axum backend integration.",
            "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?auto=format&fit=crop&w=600&q=80",
            "#",
            "#",
            "#",
        ),
        (
            "Sophia Rodriguez",
            "Head Frontend Developer",
            "Crafts responsive UI components, Tailwind v4 integrations, and accessible client-side reactivity.",
            "https://images.unsplash.com/photo-1494790108377-be9c29b29330?auto=format&fit=crop&w=600&q=80",
            "#",
            "#",
            "#",
        ),
        (
            "Marcus Vance",
            "Head Backend Developer",
            "Specializes in database ORMs, high-throughput gRPC services, and WebAssembly runtime performance.",
            "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?auto=format&fit=crop&w=600&q=80",
            "#",
            "#",
            "#",
        ),
    ];

    view! {
        <section
            id="team"
            class="py-32 bg-white dark:bg-gray-950 border-t border-gray-200 dark:border-gray-800"
        >
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">

                // Section Title & Description
                <div class="text-center max-w-3xl mx-auto mb-24">
                    <h2 class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 mb-3">
                        "Meet the Team"
                    </h2>
                    <h3 class="text-3xl font-bold tracking-tight sm:text-4xl text-[#0F4761] dark:text-white">
                        "Our Dedicated Team"
                    </h3>
                    <p class="mt-4 text-gray-600 dark:text-gray-300">
                        "The engineers and designers building open-source foundation software for the modern web."
                    </p>
                </div>

                // 4-Card Team Grid
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-x-6 gap-y-20">
                    {team_members
                        .into_iter()
                        .map(|(name, role, bio, image_url, instagram, linkedin, x_link)| {
                            view! {
                                <Card class="border-gray-200 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50 pt-16 relative flex flex-col justify-between items-center text-center overflow-visible">

                                    // Floating Full-Round Image Offset Half-Above Card
                                    <div class="absolute -top-16 left-1/2 transform -translate-x-1/2">
                                        <div class="h-28 w-28 rounded-full p-1 bg-white dark:bg-gray-950 shadow-xl border border-gray-200 dark:border-gray-800">
                                            <img
                                                src=image_url
                                                alt=name
                                                class="h-full w-full rounded-full object-cover"
                                            />
                                        </div>
                                    </div>

                                    <CardHeader class="p-0 mb-3">
                                        <CardTitle class="text-xl text-gray-900 dark:text-white font-bold">
                                            {name}
                                        </CardTitle>
                                        <CardDescription class="text-xs font-semibold uppercase tracking-wide text-indigo-600 dark:text-indigo-400 mt-1">
                                            {role}
                                        </CardDescription>
                                    </CardHeader>

                                    <CardContent class="px-6 pb-6 pt-0">
                                        <p class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed mb-6">
                                            {bio}
                                        </p>

                                        // Social Icons (Instagram, LinkedIn, X)
                                        <div class="flex items-center justify-center gap-4 text-gray-500 dark:text-gray-400">
                                            <a
                                                href=instagram
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                                            >
                                                <InstagramIcon />
                                                <span class="sr-only">"Instagram"</span>
                                            </a>
                                            <a
                                                href=linkedin
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                                            >
                                                <LinkedinIcon />
                                                <span class="sr-only">"LinkedIn"</span>
                                            </a>
                                            <a
                                                href=x_link
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                                            >
                                                <XIcon />
                                                <span class="sr-only">"X"</span>
                                            </a>
                                        </div>
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

// --- Social SVG Icons ---

#[component]
fn InstagramIcon() -> impl IntoView {
    view! {
        <svg
            class="h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2"
        >
            <rect width="20" height="20" x="2" y="2" rx="5" ry="5" />
            <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" />
            <line x1="17.5" x2="17.51" y1="6.5" y2="6.5" />
        </svg>
    }
}

#[component]
fn LinkedinIcon() -> impl IntoView {
    view! {
        <svg
            class="h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2"
        >
            <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
            <rect width="4" height="12" x="2" y="9" />
            <circle cx="4" cy="4" r="2" />
        </svg>
    }
}

#[component]
fn XIcon() -> impl IntoView {
    view! {
        <svg
            class="h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 24 24"
        >
            <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
        </svg>
    }
}
