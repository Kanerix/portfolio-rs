use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::*;
		use leptos_actix::{generate_route_list, LeptosRoutes};
		use actix_files::Files;
		use actix_web::{App, HttpServer, middleware};
		use portfolio::app::App;

		#[actix_web::main]
		async fn main() -> std::io::Result<()> {
			let conf = get_configuration(None).await.unwrap();
			let addr = conf.leptos_options.site_addr;
			log::info!("Starting server at {}", addr);

			HttpServer::new(move || {
				let leptos_options = &conf.leptos_options;
				let site_root = &leptos_options.site_root;

				// Generate the list of routes in your Leptos App
				let routes = generate_route_list(|cx| view! { cx, <App/> });

				App::new()
					.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
					.leptos_routes(
						leptos_options.to_owned(),
						routes,
						|cx| view! { cx, <App/> },
					)
					.service(Files::new("/", site_root))
					.wrap(middleware::Compress::default())
			})
			.bind(addr)?
			.run()
			.await
		}
	} else {
		fn main() {
			panic!("You must enable the ssr feature to run the server")
		}
	}
}
