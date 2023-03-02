use leptos::*;

#[component]
pub fn Input(cx: Scope) -> impl IntoView {
	view! { cx,
		<input
			type="text"
			placeholder="Enter your name"
			class="rounded-md border-2 px-2 py-1
			focus:outline-none focus:ring-2 focus:ring-blue-600"
		/>
	}
}
