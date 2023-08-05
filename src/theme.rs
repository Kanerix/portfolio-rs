use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::{Meta, Body};
use serde::{Deserialize, Serialize};
use std::ops::Not;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ColorMode {
	Light,
	#[default]
	Dark,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme(pub ReadSignal<ColorMode>, pub WriteSignal<ColorMode>);

impl ToString for ColorMode {
	fn to_string(&self) -> String {
		match self {
			ColorMode::Light => "Light".to_string(),
			ColorMode::Dark => "Dark".to_string(),
		}
	}
}

impl Not for ColorMode {
	type Output = ColorMode;

	fn not(self) -> Self::Output {
		match self {
			ColorMode::Light => ColorMode::Dark,
			ColorMode::Dark => ColorMode::Light,
		}
	}
}

impl ColorMode {
	pub fn to_fa_icon(&self) -> String {
		match self {
			ColorMode::Light => "fa-sun".to_owned(),
			ColorMode::Dark => "fa-moon".to_owned(),
		}
	}
}

cfg_if! {
	if #[cfg(feature = "ssr")] {
		fn initial_color_mode(cx: Scope) -> ColorMode {
			use_context::<actix_web::HttpRequest>(cx)
				.and_then(|req| {
					let cookies = req.cookies().ok();
					cookies.and_then(|cookies| {
						cookies
							.iter()
							.find(|cookie| cookie.name() == "color_mode")
							.and_then(|cookie| match cookie.value() {
								"Dark" => Some(ColorMode::Dark),
								"Light" => Some(ColorMode::Light),
								_ => None,
							})
					})
				})
				.unwrap_or_default()
		}
	} else {
		fn initial_color_mode(_cx: Scope) -> ColorMode {
			use wasm_bindgen::JsCast;

			let doc = document().unchecked_into::<web_sys::HtmlDocument>();
			let cookie = doc.cookie().unwrap_or_default();
			if cookie.contains("color_mode=Dark") {
				ColorMode::Dark
			} else {
				ColorMode::Light
			}
		}
	}
}

#[component]
pub fn ThemeProvider(cx: Scope, children: Children) -> impl IntoView {
	let initial_color_mode = initial_color_mode(cx);
	let (color_mode, set_color_mode) = create_signal(cx, initial_color_mode);

	provide_context(cx, Theme(color_mode, set_color_mode));

	let classes = create_memo(cx, move |_| {
		match color_mode.get() {
			ColorMode::Light => "",
			ColorMode::Dark => "dark",
		}.to_string()
	});


	view! { cx,
		<Meta
			name="color-scheme"
			content=move || color_mode.get().to_string().to_lowercase()
		/>
		<Body class=move || classes.get() />
		{children(cx)}
	}
}

#[component]
pub fn ToggleThemeButton(cx: Scope) -> impl IntoView {
	let Theme(color_mode, set_color_mode) = use_context::<Theme>(cx).unwrap();
	let fa_icon = create_memo(cx, move |_| {
		color_mode.get().to_fa_icon()
	});

	let toggle_color_mode = move |_| {
		log::info!("Toggling color mode: {:?}", color_mode.get());
		let color_mode = color_mode.get();
		let new_color_mode = match color_mode {
			ColorMode::Light => ColorMode::Dark,
			ColorMode::Dark => ColorMode::Light,
		};
		set_color_mode.set(new_color_mode);
	};

	view! { cx,
		<button
			class="m-4 w-14 h-14
			absolute bottom-0 right-0
			float-right rounded-full
			bg-gray-500/[.20] hover:bg-gray-500/[.35]" 
			type="submit"
			on:click=toggle_color_mode
		>
			<i class=move || format!("fa-solid {} text-4xl", fa_icon.get())/>
		</button>
	}
}
