use leptos::prelude::*;
use ui_components::Button;
use ui_components::Input;
use ui_components::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn ContactSection() -> impl IntoView {
    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let contact_email = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let newsletter_email = RwSignal::new(String::new());

    let handle_contact_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // Handle contact form submission logic here
    };

    let handle_newsletter_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // Handle newsletter subscription logic here
    };

    view! {
        <section
            id="contact"
            class="py-24 bg-gray-50/50 dark:bg-gray-900/50 border-t border-gray-200 dark:border-gray-800"
        >
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start max-w-6xl mx-auto">

                    // --- Left: Contact Form (7 cols) ---
                    <Card class="lg:col-span-7 border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 p-4 sm:p-6 shadow-xl">
                        <CardHeader>
                            <span class="text-xs font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400">
                                "Get in Touch"
                            </span>
                            <CardTitle class="text-2xl sm:text-3xl text-gray-900 dark:text-white mt-1">
                                "Contact Our Team"
                            </CardTitle>
                            <CardDescription class="text-gray-600 dark:text-gray-400">
                                "Have questions about our Leptos starter or enterprise support? Send us a message."
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form on:submit=handle_contact_submit class="space-y-4">
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                    <div class="space-y-2">
                                        <label class="text-xs font-medium text-gray-700 dark:text-gray-300">
                                            "First Name"
                                        </label>
                                        <Input type="text" placeholder="Jane" value=first_name />
                                    </div>
                                    <div class="space-y-2">
                                        <label class="text-xs font-medium text-gray-700 dark:text-gray-300">
                                            "Last Name"
                                        </label>
                                        <Input type="text" placeholder="Doe" value=last_name />
                                    </div>
                                </div>

                                <div class="space-y-2">
                                    <label class="text-xs font-medium text-gray-700 dark:text-gray-300">
                                        "Work Email"
                                    </label>
                                    <Input
                                        type="email"
                                        placeholder="jane@company.com"
                                        value=contact_email
                                    />
                                </div>

                                <div class="space-y-2">
                                    <label class="text-xs font-medium text-gray-700 dark:text-gray-300">
                                        "Message"
                                    </label>
                                    <textarea
                                        class="flex min-h-30 w-full rounded-md border border-gray-200 bg-transparent px-3 py-2 text-sm shadow-sm transition-colors placeholder:text-gray-400 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-indigo-500 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-800 dark:placeholder:text-gray-500"
                                        placeholder="Tell us about your project..."
                                        on:input=move |ev| message.set(event_target_value(&ev))
                                        prop:value=message
                                    />
                                </div>

                                <Button type="submit" class="w-full h-11 text-base">
                                    "Send Message"
                                </Button>
                            </form>
                        </CardContent>
                    </Card>

                    // --- Right: Join Newsletter Card (5 cols) ---
                    <Card class="lg:col-span-5 border-indigo-500/30 bg-indigo-600 dark:bg-indigo-950/80 text-white p-4 sm:p-6 shadow-xl relative overflow-hidden flex flex-col justify-between min-h-full">
                        <div class="absolute -top-16 -right-16 w-64 h-64 bg-purple-500/20 rounded-full blur-2xl pointer-events-none" />

                        <div>
                            <CardHeader class="px-0 pt-0">
                                <span class="text-xs font-semibold uppercase tracking-wider text-indigo-200">
                                    "Stay Informed"
                                </span>
                                <CardTitle class="text-2xl sm:text-3xl text-white mt-1">
                                    "Join Our Monthly Newsletter"
                                </CardTitle>
                                <CardDescription class="text-indigo-100 mt-2 leading-relaxed">
                                    "Get curated Rust ecosystem updates, Leptos release notes, and component tutorials delivered straight to your inbox once a month."
                                </CardDescription>
                            </CardHeader>

                            <CardContent class="px-0 mt-6">
                                <form on:submit=handle_newsletter_submit class="space-y-4">
                                    <div class="space-y-2">
                                        <label class="text-xs font-medium text-indigo-100">
                                            "Email Address"
                                        </label>
                                        <Input
                                            type="email"
                                            placeholder="you@domain.com"
                                            value=newsletter_email
                                            class="bg-white/10 border-white/20 text-white placeholder:text-indigo-200/60 focus-visible:ring-white"
                                        />
                                    </div>

                                    <Button
                                        type="submit"
                                        class="w-full h-11 text-base bg-white text-indigo-600 hover:bg-indigo-50 shadow-md"
                                    >
                                        "Subscribe Now"
                                    </Button>
                                </form>
                            </CardContent>
                        </div>

                        <div class="mt-8 pt-6 border-t border-indigo-500/40 text-xs text-indigo-200">
                            "No spam, ever. Unsubscribe at any time with one click."
                        </div>
                    </Card>

                </div>
            </div>
        </section>
    }
}
