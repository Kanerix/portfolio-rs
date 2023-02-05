use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::button::{Button, ButtonProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
		<Link rel="preconnect" href="https://fonts.googleapis.com"/>
		<Link rel="preconnect" href="https://fonts.gstatic.com"/>
		<Link href="https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,300;0,400;0,500;0,600;0,700;0,800;1,500;1,600&display=swap" rel="stylesheet"/>
		<Stylesheet id="leptos" href="/pkg/portfolio.css"/>
		<Title text="Kasper's portfolio!"/>
		<Router>
			<main class="h-screen w-6/12 gap-4 m-auto grid grid-cols-2 place-content-center">
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
		<div class="flex flex-col items-start">
			<h1 class="text-5xl font-semibold">"Hi, im Kasper"</h1>
			<h4 class="pt-2 text-xl text-slate-500 font-semibold">"Fullstack Developer"</h4>
			<p class="pt-4 grow">
				"Experience in using React, Rust and Python.
				I have a passion for learning new things and
				I am always looking for new challenges to 
				gather experience."
			</p>
			<Button text="About me"/>
		</div>
		<div>
			<img 
				class="rounded-full w-64 h-64 m-auto"
				src="profile.jpg"
			/>
		</div>
	}
}
