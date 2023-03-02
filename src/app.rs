use crate::pages::{
	contact::{ContactPage, ContactPageProps},
	home::{HomePage, HomePageProps},
};
use crate::theme::{ToggleColorMode, ToggleThemeButton, ToggleThemeButtonProps};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
	let _ = ToggleColorMode::register();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
		// Google fonts (Poppins)
		<Link rel="preconnect" href="https://fonts.googleapis.com"/>
		<Link rel="preconnect" href="https://fonts.gstatic.com"/>
		<Link href="https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,300;0,400;0,500;0,600;0,700;0,800;1,500;1,600&display=swap" rel="stylesheet"/>
		// Fontawesome icons
		<Stylesheet href="/fontawesome/css/all.min.css"/>
		// Tailwind generated stylesheet
		<Stylesheet id="leptos" href="/pkg/portfolio.css"/>
		<Title text="Kasper's portfolio!"/>
		<Body class="h-screen w-full flex flex-col" />
		<Router>
			<header>
				<ToggleThemeButton />
			</header>
			<main class="grow w-3/4 gap-4 m-auto grid grid-cols-1 place-content-center xl:w-1/2 md:grid-cols-2">
				<Routes>
					<Route path="/" view=|cx| view! { cx, <HomePage/> }/>
					<Route path="/contact" view=|cx| view! { cx, <ContactPage/> }/>
				</Routes>
			</main>
		</Router>
	}
}
