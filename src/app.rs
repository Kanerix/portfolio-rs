use crate::pages::home::Home;
use crate::theme::{ThemeProvider, ToggleThemeButton};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
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
		<Body class="h-screen w-full bg-slate-100 dark:bg-slate-900" />
		<ThemeProvider>
			<Router>
				<main class="h-screen mx-auto w-2/3">
					<Routes>
						<Route path="/" view=|cx| view! { cx, <Home/> }/>
					</Routes>
					<ToggleThemeButton />
				</main>
			</Router>
		</ThemeProvider>
	}
}
