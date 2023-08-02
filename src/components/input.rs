use leptos::*;

#[component]
pub fn Input(
	cx: Scope,
	#[prop(optional, into)]
	input_type: Option<&'static str>,
	/// Placeholder text to display in the input.
	#[prop(optional, into)]
	placeholder: Option<&'static str>,
	/// Additional classes to add to the button.
	#[prop(optional, into)]
	class: Option<&'static str>,
) -> impl IntoView {
	let styles = format!(
		"py-2 px-4 rounded-md
		bg-gray-500/[.20] hover:bg-gray-500/[.35]
		focus:ring focus:ring-blue-500 focus:ring-opacity-50 {}",
		class.unwrap_or("")
	);

	view! { cx,
		<input
			type=input_type
			placeholder=placeholder
			class=styles
		/>
	}
}
