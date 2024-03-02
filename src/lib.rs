pub mod app;
pub mod pages;
pub mod theme;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use app::*;
	use leptos::*;

	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	mount_to_body(App);
}
