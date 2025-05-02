use crate::pages::home::Home;
use crate::theme::{ThemeProvider, ToggleThemeButton};

use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(_: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>"Kasper's portfolio"</title>
                <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
                <link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
                <link rel="manifest" href="/manifest.json"/>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <ThemeProvider>
                <main class="mx-auto w-2/3">
                    <ToggleThemeButton />
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=|| view! { <Home/> }/>
                    </Routes>
                </main>
            </ThemeProvider>
        </Router>
    }
}
