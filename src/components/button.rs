use leptos::*;


#[component]
pub fn Button(cx: Scope) -> impl IntoView {
	let (count, set_count) = create_signal(cx, 0);
	let increment = move |_| {
		println!("{count:?}");
		set_count.update(|count| *count += 1);
	};

	view! { cx,
		<button on:click=increment>"Increment"</button>
		<p>"You clicked me " {count} " times"</p>
	}
}