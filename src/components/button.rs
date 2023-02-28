use leptos::*;
use leptos_router::{A, AProps};

#[component]
pub fn Button(
	cx: Scope,
	#[prop(optional)]
	class: &'static str,
	#[prop(optional)]
	/// If `to` is empty, the button will be a `<button>` element, else, it will be an `<a>` element.
	text: &'static str,
	#[prop(optional)]
	to: &'static str,
) -> impl IntoView {
	let styles = format!(
		"text-white font-bold py-2 px-4 rounded-md
		bg-gradient-to-t from-blue-700 to-blue-500
		hover:from-blue-800 hover:to-blue-600 
		focus:ring focus:ring-blue-500 focus:ring-opacity-50
		{class}"
	);

	if to.is_empty() {
		view! { cx, 
			<button class=styles>{text.to_string()}</button>
		}.into_view(cx)
	} else {
		view! { cx,
			<A href=to class=styles>{text.to_string()}</A>
		}.into_view(cx)
	}
}