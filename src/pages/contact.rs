use crate::components::button::{Button, ButtonProps};
use leptos::*;

#[component]
pub fn ContactPage(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="w-full flex flex-col items-start">
			<Button class="w-full" text="Back" to="/" />
		</div>
	}
}
