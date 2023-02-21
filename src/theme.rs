use std::ops::Not;
use leptos::*;
use leptos_meta::{Meta, MetaProps};
use leptos_router::{ActionForm, ActionFormProps};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ColorMode {
	Light,
	Dark,
}

impl ToString for ColorMode {
	fn to_string(&self) -> String {
		match self {
			ColorMode::Light => "light".to_string(),
			ColorMode::Dark => "dark".to_string(),
		}
	}
}

impl Default for ColorMode {
	fn default() -> Self {
		ColorMode::Dark
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

#[server(ToggleColorMode, "/api")]
pub async fn toggle_color_mode(cx: Scope, color_mode: ColorMode) -> Result<ColorMode, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response = use_context::<ResponseOptions>(cx).expect("No response options");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
	headers.insert(
		SET_COOKIE,
		HeaderValue::from_str(&format!("colormode={}; Path=/", color_mode.to_string()))
			.expect("Could't create cookie header"),
	);
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(250));

    response.overwrite(response_parts);
	Ok(color_mode)
}

#[cfg(not(feature = "ssr"))]
fn initial_color_mode(_cx: Scope) -> ColorMode {
	let doc = document().unchecked_into::<web_sys::HtmlDocument>();
	let cookie = doc.cookie().unwrap_or_default();
	match cookie.as_str() {
		"colormode=dark" => ColorMode::Dark,
		"colormode=light" => ColorMode::Light,
		_ => {
			console_log("No cookie found, defaulting to dark mode");
			return ColorMode::Dark;
		},
	}
}

#[cfg(feature = "ssr")]
fn initial_color_mode(cx: Scope) -> ColorMode {
    use_context::<actix_web::HttpRequest>(cx);

	ColorMode::Dark
}

#[component]
pub fn ToggleThemeButton(cx: Scope) -> impl IntoView {
	let initial_color_mode = initial_color_mode(cx);
	
	let toggle_color_mode_action = create_server_action::<ToggleColorMode>(cx);
    let input = toggle_color_mode_action.input();
    let value = toggle_color_mode_action.value();

	let color_mode = move || -> ColorMode {
		match(input(), value()) {
            // if there's some current input, use that optimistically
			(Some(submission), _) => submission.color_mode,
			// otherwise, use the current value
			(_, Some(Ok(value))) => value,
			// if there's an error, use the initial value and log error
			(_, Some(Err(err))) => {
				console_log(&format!("Error: {:?}", err));
				initial_color_mode
			}
			// at last, use the initial value
			_ => initial_color_mode,
		}
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
			<input type="submit">
				"Toggle theme"
			</input>
		</ActionForm>
	}
}