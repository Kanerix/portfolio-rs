use leptos::*;
use leptos_router::{AProps, A};

#[component]
pub fn Button(
	cx: Scope,
	/// Additional classes to add to the button.
	#[prop(optional, into)]
	class: Option<String>,
	/// The text to display on the button.
	#[prop(optional, into)]
	text: Option<String>,
	/// If `to` is empty, the button will be a `<button>` element, else, it will be an `<a>` element.
	#[prop(optional, into)]
	to: Option<String>,
) -> impl IntoView {
	let mut styles = String::from(
		"text-white text-center font-bold py-2 px-4 rounded-md
		bg-gradient-to-t from-blue-700 to-blue-500
		hover:from-blue-800 hover:to-blue-600 
		focus:ring focus:ring-blue-500 focus:ring-opacity-50 ",
	);

	if let Some(class) = class {
		styles.push_str(&class);
	}

	if let Some(to) = to {
		view! { cx,
			<A href=to class=styles>{text}</A>
		}
		.into_view(cx)
	} else {
		view! { cx,
			<button class=styles>{text}</button>
		}
		.into_view(cx)
	}
}
