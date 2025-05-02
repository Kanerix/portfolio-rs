use leptos::prelude::*;
use leptos_meta::{Body, Html, Meta};
use serde::{Deserialize, Serialize};
use std::{ops::Not, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColorMode {
    Dark,
    Light,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme(pub ReadSignal<ColorMode>, pub WriteSignal<ColorMode>);

impl AsRef<str> for ColorMode {
    fn as_ref(&self) -> &str {
        match self {
            ColorMode::Dark => "dark",
            ColorMode::Light => "light",
        }
    }
}

impl FromStr for ColorMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(ColorMode::Light),
            "dark" => Ok(ColorMode::Dark),
            _ => Err(()),
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
    pub fn to_fa_icon(&self) -> &str {
        match self {
            ColorMode::Light => "fa-sun",
            ColorMode::Dark => "fa-moon",
        }
    }

    fn initial_color_mode() -> ColorMode {
        // if cfg!(feature = "ssr") {
        // 	use_context::<tower::>()
        // 		.and_then(|req| {
        // 			let cookies = req.cookies().ok();
        // 			cookies.and_then(|cookies| {
        // 				cookies
        // 					.iter()
        // 					.find(|cookie| cookie.name() == "color_mode")
        // 					.and_then(|cookie| match cookie.value() {
        // 						"dark" => Some(ColorMode::Dark),
        // 						"light" => Some(ColorMode::Light),
        // 						_ => None,
        // 					})
        // 			})
        // 		})
        // 		.unwrap_or_default()
        // } else {
        // 	window()
        // 		.local_storage()
        // 		.ok()
        // 		.and_then(|local_storage| {
        // 			local_storage
        // 				.and_then(|storage| storage.get_item("color_mode").ok())
        // 				.and_then(|color_mode| match color_mode.as_deref() {
        // 					Some("dark") => Some(ColorMode::Dark),
        // 					Some("light") => Some(ColorMode::Light),
        // 					_ => None,
        // 				})
        // 		})
        // 		.unwrap_or_default()
        // }
        ColorMode::Dark
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let initial_color_mode = ColorMode::initial_color_mode();
    let (color_mode, set_color_mode) = signal(initial_color_mode);

    provide_context(Theme(color_mode, set_color_mode));

    let _classes = Memo::new(move |_| color_mode.get().as_ref().to_string());

    Effect::new(move |_| {
        let local_storage = window().local_storage().ok();
        local_storage.and_then(|local_storage| {
            local_storage.and_then(|storage| {
                storage
                    .set_item("color_mode", color_mode.get().as_ref())
                    .ok()
            })
        });
    });

    let set_color_scheme = move || color_mode.get().as_ref().to_string();

    view! {
        <Meta name="color-scheme" content=set_color_scheme />
        {children()}
    }
}

/// A button that toggles the theme color mode.
#[component]
pub fn ToggleThemeButton() -> impl IntoView {
    let Theme(color_mode, set_color_mode) = use_context::<Theme>().unwrap();
    let fa_icon = Memo::new(move |_| color_mode.get().to_fa_icon().to_string());

    let toggle_color_mode = move |_| set_color_mode.set(!color_mode.get());

    view! {
        <button
            aria-label="Toggle color mode"
            class="m-4 w-14 h-14
			fixed bottom-0 right-0
			float-right rounded-full
			bg-gray-500/[.20] hover:bg-gray-500/[.35]" 
            type="submit"
            on:click=toggle_color_mode
        >
            <i class=move || format!("fa-solid {} text-4xl", fa_icon.get())/>
        </button>
    }
}
