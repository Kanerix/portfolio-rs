use leptos::*;
use leptos_meta::Meta;
use leptos_router::ActionForm;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::ops::Not;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ColorMode {
	Light,
	#[default]
	Dark,
}

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

#[server(ToggleColorMode, "/api")]
pub async fn toggle_color_mode(
	cx: Scope,
	color_mode: ColorMode,
) -> Result<ColorMode, ServerFnError> {
	use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
	use leptos_actix::{ResponseOptions, ResponseParts};

	let response = use_context::<ResponseOptions>(cx).expect("No response options");
	let mut response_parts = ResponseParts::default();
	let mut headers = HeaderMap::new();
	headers.insert(
		SET_COOKIE,
		HeaderValue::from_str(&format!("color_mode={}; Path=/", color_mode.to_string()))
			.expect("Could't create cookie header"),
	);
	response_parts.headers = headers;

	std::thread::sleep(std::time::Duration::from_millis(250));

	response.overwrite(response_parts);
	Ok(color_mode)
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
			// Implement this
			ColorMode::default()
		}
	}
}

#[component]
pub fn ToggleThemeButton(cx: Scope) -> impl IntoView {
	let initial_color_mode = initial_color_mode(cx);
	let (fa_theme_icon, set_fa_theme_icon) = create_signal(cx, initial_color_mode.to_fa_icon());

	let toggle_color_mode_action = create_server_action::<ToggleColorMode>(cx);
	let input = toggle_color_mode_action.input();
	let value = toggle_color_mode_action.value();

	let color_mode = move || {
		let color = match (input.get(), value.get()) {
			// if there's some current input, use that optimistically
			(Some(submission), _) => submission.color_mode,
			// otherwise, use the current value
			(_, Some(Ok(value))) => value,
			// if there's an error, use the initial value and log error
			(_, Some(Err(_))) => initial_color_mode,
			// at last, use the initial value
			_ => initial_color_mode,
		};

		set_fa_theme_icon.set(color.to_fa_icon());
		color
	};

	view! { cx,
		<Meta
			name="color-scheme"
			content=move || color_mode().to_string()
		/>
		<ActionForm action=toggle_color_mode_action>
			<input
				type="hidden"
				name="color_mode"
				value=move || (!color_mode()).to_string()
			/>
			<button 
				class="m-4 w-14 h-14
				float-right rounded-full
				bg-gray-500/[.20] hover:bg-gray-500/[.35]" 
				type="submit"
			>
				<i class=format!("fa-solid {} text-4xl", fa_theme_icon.get())/>
			</button>
		</ActionForm>
	}
}

#[component]
pub fn ThemeProvider(cx: Scope, children: Children) -> impl IntoView {
	children(cx)
}