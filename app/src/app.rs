use crate::components::landing::{
    AboutCompany, CtaSection, FaqSection, FeaturesSection, Footer, Hero, Navbar, ServicesSection,
    Sponsors, TeamSection, TestimonialsSection,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use ui_components::ThemeProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <ThemeProvider>
            <Stylesheet id="leptos" href="/pkg/rust_ui_starter.css" />
            <Title text="Pure Rust UI Starter (Leptos 0.9)" />
            <Router>
                <main class="container snap-center">
                    <Routes fallback=|| "404: Page Not Found">
                        <Route path=path!("/") view=HomePage />
                    </Routes>
                </main>
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // let (count, set_count) = signal(0);

    view! {
        <main class="min-h-screen w-screen bg-background text-foreground transition-colors duration-200">
            <div>
                <Navbar />
                <Hero />
                <Sponsors />
                <AboutCompany />
                <FeaturesSection />
                <ServicesSection />
                <TestimonialsSection />
                <TeamSection />
                // <ContactSection />
                <CtaSection />
                <FaqSection />
            </div>
            <Footer />
        </main>
    }
}
