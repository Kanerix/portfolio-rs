use leptos::*;
use crate::components::button::{Button, ButtonProps};

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
			<div class="flex flex-col items-start grow">
				<h1 class="text-5xl font-semibold">"Hi, im Kasper"</h1>
				<h4 class="pt-2 text-xl text-slate-500 font-semibold">"Fullstack Developer"</h4>
				<p class="pt-4 grow">
					"Experience in using React, Rust and Python.
					I have a passion for learning new things and
					I am always looking for new challenges to 
					gather experience."
				</p>
				<Button class="mt-4" text="Contact" to="/contact" />
			</div>
			<div>
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
			</div>
		</div>
	}
}
