use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::{
	contact::ContactPage,
	home::HomePage,
};
use crate::theme::{ToggleThemeButton, ThemeProvider};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
		<Title text="Kasper's portfolio!"/>
		<Meta name="description" content="Kasper's portfolio website, created using Leptos!"/>
		<Link rel="manifest" href="/manifest.json"/>
		// Google fonts (Poppins)
		<Link rel="preconnect" href="https://fonts.googleapis.com"/>
		<Link rel="preconnect" href="https://fonts.gstatic.com"/>
		<Link href="https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,300;0,400;0,500;0,600;0,700;0,800;1,500;1,600&display=swap" rel="stylesheet"/>
		// Fontawesome icons
		<Stylesheet href="/fontawesome/css/all.min.css"/>
		// Tailwind generated stylesheet
		<Stylesheet id="leptos" href="/pkg/portfolio.css"/>
		<Body class="h-screen w-full flex flex-col" />
		<ThemeProvider>
			<Router>
				<header>
					<ToggleThemeButton />
				</header>
				<main class="w-3/4 m-auto xl:w-1/2">
					<Routes>
						<Route path="/" view=|cx| view! { cx, <HomePage/> }/>
						<Route path="/contact" view=|cx| view! { cx, <ContactPage/> }/>
					</Routes>
				</main>
			</Router>
		</ThemeProvider>
	}
}
