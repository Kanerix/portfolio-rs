use cfg_if::cfg_if;
use tracing::Level;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::*;
		use leptos_actix::{generate_route_list, LeptosRoutes};
		use actix_files::Files;
		use actix_web::{web, App, HttpServer, middleware};
		use portfolio::app::App as Frontend;

		#[actix_web::main]
		async fn main() -> std::io::Result<()> {
			tracing_subscriber::fmt()
				.with_max_level(Level::DEBUG)
				.init();

			let conf = get_configuration(None).await.unwrap();
			let addr = conf.leptos_options.site_addr;

			tracing::info!("Starting server at {}", addr);

			HttpServer::new(move || {
				let leptos_options = &conf.leptos_options;
				let site_root = &leptos_options.site_root;

				let routes = generate_route_list(|| view! { <Frontend/> });

				App::new()
					.service(Files::new("/pkg", format!("{site_root}/pkg")))
					.service(Files::new("/assets", site_root))
					.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
					.leptos_routes(
						leptos_options.to_owned(),
						routes,
						|| view! { <Frontend/> },
					)
					.service(Files::new("/", site_root))
					.app_data(web::Data::new(leptos_options.to_owned()))
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

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
	leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
	let leptos_options = leptos_options.into_inner();
	let site_root = &leptos_options.site_root;
	Ok(actix_files::NamedFile::open(format!(
		"{site_root}/favicon.ico"
	))?)
}
