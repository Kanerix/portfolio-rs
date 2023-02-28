use leptos::*;
use crate::components::button::{Button, ButtonProps};

#[component]
pub fn ContactPage(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="flex flex-col items-start">
			<Button text="Back" to="/" />
		</div>
	}
}