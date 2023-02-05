use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::button::{Button, ButtonProps};
use crate::components::input::{Input, InputProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
		<Stylesheet id="leptos" href="/pkg/portfolio.css"/>
		<Title text="Welcome to Leptos"/>
		<Router>
			<main>
				<Routes>
					<Route path="" view=|cx| view! { cx, <HomePage/> }/>
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
	view! { cx,
		<h1>"Welcome to Leptos!"</h1>
		<Button/>
		<Input/>
	}
}
