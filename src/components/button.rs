use leptos::*;


#[component]
pub fn Button(
	cx: Scope,
	#[prop(optional)]
	class: &'static str,
	#[prop(optional)]
	text: &'static str,
	// TODO: Make on_click optional prop
) -> impl IntoView {
	view! { cx,
		<button
			class=format!(
				"bg-blue-500 text-white font-bold py-2 px-4 rounded
				hover:bg-blue-700 {}",
				class.to_string()
			)
			// on:click=on_click
		>
			{text.to_string()}
		</button>
	}
}