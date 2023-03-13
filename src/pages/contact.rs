use leptos::*;
use crate::components::button::{Button, ButtonProps};
use crate::components::input::{Input, InputProps};
use crate::components::text_area::{TextArea, TextAreaProps};

#[component]
pub fn ContactPage(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2">
			<Input placeholder="Enter email" />
			<Input placeholder="Enter name" />
			<TextArea class="w-full col-span-2" placeholder="Enter message" rows=3 />
			<Button class="w-full col-span-2" text="Confirm" />
			<Button class="w-full col-span-2" text="Back" to="/" />
		</div>
	}
}
