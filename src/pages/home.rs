use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Repo {
	name: String,
	html_url: String,
	homepage: Option<String>,
}

impl IntoView for Repo {
	fn into_view(self) -> View {
		view! {
			<div class="bg-slate-900">
				{self.name}
			</div>
		}
		.into_view()
	}
}

async fn fetch_repos() -> Option<Vec<Repo>> {
	Request::get("https://api.github.com/users/Kanerix/repo")
		.send()
		.await
		.ok()?
		.json::<Vec<Repo>>()
		.await
		.ok()
}

#[component]
pub fn Home() -> impl IntoView {
	let repos = create_local_resource(|| (), |_| async move { fetch_repos().await });

	view! {
		<div class="grid gap-4 grid-cols-1 md:grid-cols-3">
			<div class="my-32 flex flex-col items-start grow">
				<h1 class="text-5xl font-semibold text-slate-900 dark:text-slate-100">
					"Hi, im Kasper"
				</h1>
				<h2 class="pt-2 text-xl text-slate-600 dark:text-slate-300">
					"Fullstack Developer"
				</h2>
				<p class="pt-4 grow text-slate-500 dark:text-slate-400">
					"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
					Donec placerat justo neque, ut accumsan mi tristique in."
				</p>
				<div class="grid gap-5 grid-cols-3 mt-8">
					<a href="https://github.com/Kanerix" aria-label="Checkout my GitHub">
						<i class="fa-brands fa-github text-4xl text-slate-600 dark:text-slate-400" />
					</a>
					<a href="https://twitter.com/K4nerix" aria-label="Checkout my Twitter">
						<i class="fa-brands fa-twitter text-4xl text-slate-600 dark:text-slate-400" />
					</a>
					<a href="https://www.linkedin.com/in/kasper-jonsson" aria-label="Checkout my LinkedIn">
						<i class="fa-brands fa-linkedin text-4xl text-slate-600 dark:text-slate-400" />
					</a>
				</div>
			</div>
			<div class="overflow-y-scroll h-screen col-span-2">
				{move || {
					match repos.get() {
						None => view! {
							<div class="text-slate-600 dark:text-slate-400">"Loading..."</div>
						}.into_view(),
						Some(repos) => repos.into_view(),
					}
				}}
			</div>
		</div>
	}
}
