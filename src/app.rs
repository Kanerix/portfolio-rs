use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::button::{Button, ButtonProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context(cx);

	view! {
		cx,

		// injects a stylesheet into the document <head>
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/portfolio-rs.css"/>

		// sets the document title
		<Title text="Welcome to Leptos"/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route path="" view=|cx| view! { cx, <HomePage/> }/>
				</Routes>
			</main>
		</Router>
	}
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
	view! { cx,
		<h1>"Welcome to Leptos!"</h1>
		<Button/>
	}
}
