use crate::pages::home::Home;
use crate::theme::{ThemeProvider, ToggleThemeButton};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// The entire application root.
#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Html lang="en" dir="ltr" />
		<Title text="Kasper's portfolio!"/>
		<Meta charset="utf-8"/>
		<Meta name="description" content="Kasper's portfolio website, created using Leptos!"/>
		<Link rel="manifest" href="/manifest.json"/>
		// Google fonts (Poppins)
		<Link rel="preconnect" href="https://fonts.googleapis.com"/>
		<Link rel="preconnect" href="https://fonts.gstatic.com"/>
		<Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,300;0,400;0,500;0,600;0,700;0,800;1,500;1,600&display=swap"/>
		// Fontawesome icons
		<Stylesheet href="/fontawesome/css/all.min.css"/>
		// Tailwind generated stylesheet
		<Stylesheet id="leptos" href="/pkg/portfolio.css"/>
		// The body of the document
		<Body class="min-h-screen w-full bg-white dark:bg-slate-900" />
		<div class="fixed top-0 z-[-2] h-screen w-screen bg-white bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] dark:bg-[#000000] dark:bg-[radial-gradient(#ffffff33_1px,#00091d_1px)] bg-[size:20px_20px]"></div>
		<ThemeProvider>
			<Router>
				<main class="mx-auto w-2/3">
					<ToggleThemeButton />
					<Routes>
						<Route path="/" view=|| view! { <Home/> }/>
					</Routes>
				</main>
			</Router>
		</ThemeProvider>
	}
}
