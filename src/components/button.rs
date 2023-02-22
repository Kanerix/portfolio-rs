use leptos::*;
use leptos_router::{A, AProps};

#[component]
pub fn Button(
	cx: Scope,
	#[prop(optional)]
	class: &'static str,
	#[prop(optional)]
	text: &'static str,
	#[prop(optional)]
	to: &'static str,
) -> impl IntoView {
	view! { cx,
		<button
			class=format!(
				"bg-blue-500 text-white font-bold py-2 px-4 rounded
				hover:bg-blue-700 focus:border-blue-700 focus:outline-none
				{class}",
			)
		>
			<A href=to>
				{text.to_string()}
			</A>
		</button>
	}
}