use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::*;
		use leptos_actix::{generate_route_list, LeptosRoutes};
		use actix_files::Files;
		use actix_web::{App, HttpServer, middleware};
		use portfolio::app::{App, AppProps, register_server_functions};

		#[actix_web::main]
		async fn main() -> std::io::Result<()> {
			let conf = get_configuration(None).await.unwrap();
			let addr = conf.leptos_options.site_address;
			log::info!("Starting server at {}", addr);

			register_server_functions();

			HttpServer::new(move || {
				let leptos_options = &conf.leptos_options;
				let site_root = &leptos_options.site_root;

				// Generate the list of routes in your Leptos App
				let routes = generate_route_list(|cx| view! { cx, <App/> });

				App::new()
					.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
					.leptos_routes(
						leptos_options.to_owned(),
						routes.to_owned(),
						|cx| view! { cx, <App/> },
					)
					.service(Files::new("/", site_root))
					.wrap(middleware::Compress::default())
			})
			.bind(&addr)?
			.run()
			.await
		}
	} else {
		pub fn main() {
			// no client-side main function
			// unless we want this to work with e.g., Trunk for pure client-side testing
			// see lib.rs for hydration function instead
		}
	}
}
