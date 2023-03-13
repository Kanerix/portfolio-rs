use leptos::*;

#[component]
pub fn TextArea(
	cx: Scope,
	#[prop(optional, into)]
	input_type: Option<String>,
	/// Placeholder text to display in the input.
	#[prop(optional, into)]
	placeholder: Option<String>,
	/// Additional classes to add to the button.
	#[prop(optional, into)]
	class: Option<String>,
	/// Text area rows.
	#[prop(optional, into)]
	rows: Option<i32>,
) -> impl IntoView {
	let mut styles = String::from(
		"py-2 px-4 rounded-md
		bg-gray-500/[.20] hover:bg-gray-500/[.35]
		focus:ring focus:ring-blue-500 focus:ring-opacity-50 ",
	);

	if let Some(class) = class {
		styles.push_str(&class);
	}

	view! { cx,
		<textarea
			rows=rows
			type=input_type
			placeholder=placeholder
			class=styles
		/>
	}
}
