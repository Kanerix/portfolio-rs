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
        <html lang="en" dir="ltr" class="dark">
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
            <body class="bg-slate-50 dark:bg-slate-950 mx-auto px-8 max-w-5xl min-h-screen">
                <App />
            </body>
        </html>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    let url = use_url();

    view! {
        <Nav class="border-b border-slate-200 dark:border-slate-800">
            <NavLink to="/" active=url.get().path() == "/">
                "Home"
            </NavLink>
            <NavLink to="/contact" active=url.get().path() == "/contact" >
                "Contact"
            </NavLink>
            <NavLink to="https://github.com/Kanerix" {..} target="_blank">
                "Projects"
            </NavLink>
        </Nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <main class="mb-8">
                <Routes fallback=|| NotFound() transition=true>
                    <Route path=path!("/") view=|| view! { <Home/> }/>
                    <Route path=path!("/contact") view=|| view! { <Contact/> }/>
                </Routes>
            </main>
            <footer class="border-t border-slate-200 dark:border-slate-800 p-8">
                <Text size=TextSize::Sm variant=TextVariant::Dimmed class="mb-8">
                    "Copyright © 2025 Kasper Jønsson"
                </Text>
                <div class="grid grid-cols-3">
                    <Text weight=TextWeight::Bold>
                        "Important Links"
                    </Text>
                    <Text weight=TextWeight::Bold>
                        "Socials"
                    </Text>
                    <Text weight=TextWeight::Bold>
                        "Contact"
                    </Text>
                </div>
            </footer>
        </Router>
    }
}
