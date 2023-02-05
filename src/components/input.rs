use leptos::{*, web_sys::Event};



#[component]
pub fn Input(cx: Scope)  -> impl IntoView {
	let (input, set_input) = create_signal(cx, String::new());
	let update_input = move |event: Event| {
		set_input.update(|input| {
			let target = match event.target() {
				Some(target) => target,
				None => return,
			};

			let input_element = match target.dyn_into::<web_sys::HtmlInputElement>() {
				Ok(input_element) => input_element,
				Err(_) => return,
			};
			
			*input = input_element.value();
		});
	};

	view! { cx,
		<input type="text" placeholder="Enter your name" on:change=update_input />
		<p>"Hello " {input} "!"</p>
	}
}