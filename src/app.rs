use crate::{components::*, pages::*};

use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_url,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html
            lang="en"
            dir="ltr"
            class="dark scrollbar-thin scrollbar-track-transparent 
            scrollbar-thumb-slate-500 dark:scrollbar-thumb-slate-600
            hover:scrollbar-thumb-slate-400"
        >
            <head>
                <title>"Kasper's portfolio"</title>
                <meta charset="utf-8"/>
                <meta name="description" content="Kasper's portfolio website, created using Leptos!"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
                <link rel="manifest" href="/manifest.json"/>
                // Fontawesome icons
                <link rel="stylesheet" href="/fontawesome/css/all.min.css"/>
                // Tailwind generated stylesheet
                <link rel="stylesheet" id="leptos" href="/pkg/portfolio-rs.css"/>
                // Leptos stuff
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true islands_router=true/>
            </head>
            <body class="bg-light dark:bg-dark mx-auto px-8 max-w-5xl min-h-screen">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    let url = use_url();

    view! {
        <NavBar class="border-b border-slate-200 dark:border-slate-800">
            <NavLink to="/" active=url.get().path() == "/">
                "Home"
            </NavLink>
            <NavLink to="/contact" active=url.get().path() == "/contact" >
                "Contact"
            </NavLink>
            <NavLink to="https://github.com/Kanerix" {..} target="_blank">
                "Projects"
            </NavLink>
        </NavBar>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer
            class="border-t border-slate-200 dark:border-slate-800
            grid gap-8 py-8"
        >
            <Text size="sm" variant="dimmed">
                "Copyright © 2025 Kasper Jønsson"
            </Text>
            <div class="grid grid-cols-1 md:grid-cols-3">
                <FooterColumn>
                    <Text weight="bold">
                        "Contact"
                    </Text>
                    <a
                        href="mailto:dkkasjoe@hotmail.com"
                        aria-label="Send me an email"
                    >
                        <Text size="sm" variant="dimmed">
                            "dkkasjoe@hotmail.com"
                        </Text>
                    </a>
                </FooterColumn>
                <FooterColumn>
                    <Text weight="bold">
                        "Socials"
                    </Text>
                    <a
                        href="https://github.com/Kanerix"
                        aria-label="Checkout my GitHub"
                    >
                        <Text size="sm" variant="dimmed">
                            "GitHub"
                        </Text>
                    </a>
                    <a
                        href="https://twitter.com/K4nerix"
                        aria-label="Checkout my Twitter"
                    >
                        <Text size="sm" variant="dimmed">
                            "Twitter"
                        </Text>
                    </a>
                    <a
                        href="https://linkedin.com/in/kasper-jonsson"
                        aria-label="Checkout my LinkedIn"
                    >
                        <Text size="sm" variant="dimmed">
                            "LinkedIn"
                        </Text>
                    </a>
                </FooterColumn>
                <FooterColumn>
                    <Text weight="bold">
                        "Important Links"
                    </Text>
                    <a
                        href="https://linkedin.com/in/kasper-jonsson"
                        aria-label="Checkout my LinkedIn"
                    >
                        <Text size="sm" variant="dimmed">
                            "LinkedIn"
                        </Text>
                    </a>
                </FooterColumn>
            </div>
        </footer>
    }
}

#[component]
pub fn FooterColumn(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 mb-8 md:mb-0">
            {children()}
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Nav />
            <main class="my-8">
                <Routes fallback=|| NotFound() transition=true>
                    <Route path=path!("/") view=|| view! { <Home/> }/>
                    <Route path=path!("/contact") view=|| view! { <Contact/> }/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
