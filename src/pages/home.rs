use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

static REPOS_WHITELIST: [&str; 2] = ["portfolio-rs", "mnist-ai-rust"];

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Repo {
	name: String,
	html_url: String,
	stargazers_count: u32,
	homepage: Option<String>,
	language: Option<String>,
}

impl IntoView for Repo {
	fn into_view(self) -> View {
		view! {
			<div class="shadow rounded-md bg-slate-200 dark:bg-slate-950
				p-4 max-w-sm w-full mx-auto mb-4">
				<a href={self.html_url} target="_blank">
					<div>
						<div class="flex">
							<div class="grow">
								{self.name}
							</div>
							<div>
								{Repo::get_language_icon(self.language)}
							</div>
						</div>
						<div>
							<p class="text-slate">
								"Stars: "
								<strong>{self.stargazers_count}</strong>
							</p>
						</div>
					</div>
				</a>
			</div>
		}
		.into_view()
	}
}

impl Repo {
	fn loading_view() -> impl IntoView {
		view! {
			<div class="shadow rounded-md bg-slate-200 dark:bg-slate-950
				p-4 max-w-sm w-full mx-auto">
				<div class="animate-pulse flex space-x-4">
					<div class="rounded-full bg-slate-800 h-10 w-10" />
					<div class="flex-1 space-y-8 py-1">
						<div class="rounded bg-slate-800" />
							<div class="space-y-3">
								<div class="grid grid-cols-3 gap-4">
									<div class="h-2 bg-slate-800 rounded col-span-2" />
									<div class="h-2 bg-slate-800 rounded col-span-1" />
								</div>
							<div class="h-2 bg-slate-800 rounded" />
						</div>
					</div>
				</div>
			</div>
		}
	}

	fn get_language_icon(language: Option<String>) -> impl IntoView {
		let default_classes = "text-2xl text-slate-500 dark:text-slate-400";

		let unkown_icon_view = view! {
				<i class=format!("fa-solid fa-code {}", default_classes) />
		};

		if language.is_none() {
			return unkown_icon_view;
		}

		match language.unwrap().as_str() {
			"Rust" => view! { <i class=format!("fa-brands fa-rust {}", default_classes) /> },
			"TypeScript" => view! { <i class=format!("fa-brands fa-js {}", default_classes) /> },
			"JavaScript" => view! { <i class=format!("fa-brands fa-js {}", default_classes) /> },
			"HTML" => view! { <i class=format!("fa-brands fa-html5 {}", default_classes) /> },
			"CSS" => view! { <i class=format!("fa-brands fa-css3-alt {}", default_classes) /> },
			"SCSS" => view! { <i class=format!("fa-brands fa-sass {}", default_classes) /> },
			"Python" => view! { <i class=format!("fa-brands fa-python {}", default_classes) /> },
			"C#" => view! { <i class=format!("fa-brands fa-microsoft {}", default_classes) /> },
			"C++" => view! { <i class=format!("fa-brands fa-microsoft {}", default_classes) /> },
			"Java" => view! { <i class=format!("fa-brands fa-java {}", default_classes) /> },
			"Kotlin" => view! { <i class=format!("fa-brands fa-java {}", default_classes) /> },
			"Swift" => view! { <i class=format!("fa-brands fa-swift {}", default_classes) /> },
			_ => unkown_icon_view,
		}
	}
}

async fn fetch_repos() -> Option<Vec<Repo>> {
	let repos = Request::get("https://api.github.com/users/Kanerix/repos")
		.send()
		.await
		.ok()?
		.json::<Vec<Repo>>()
		.await
		.ok()?;

	let repos_filtered = repos
		.iter()
		.filter(|&repo| REPOS_WHITELIST.contains(&repo.name.as_str()))
		.cloned()
		.collect::<Vec<Repo>>();

	Some(repos_filtered)
}

/// The home page.
#[component]
pub fn Home() -> impl IntoView {
	let repos = create_local_resource(|| (), |_| async move { fetch_repos().await });

	view! {
		<div class="grid gap-4 grid-cols-1 md:my-32 xl:grid-cols-3">
			<div class="flex flex-col">
				<h1 class="text-5xl font-semibold text-slate-900 dark:text-slate-100">
					"Hi, im Kasper"
				</h1>
				<h2 class="pt-2 text-xl text-slate-600 dark:text-slate-300">
					"Fullstack Developer"
				</h2>
				<p class="pt-4 text-slate-500 dark:text-slate-400">
					"I'm a software engineer and IT enthusiast. I'm currently studying sowftware development at the "
					<a href="https://itu.dk/" target="_blank" class="text-slate-700 dark:text-slate-200">
						"IT University of Chopenhagen"
					</a>
				</p>
				<div class="grid grid-cols-3 w-2/4 mt-16">
					<a href="https://github.com/Kanerix" aria-label="Checkout my GitHub">
						<i class="fa-brands fa-github text-4xl
							text-slate-600 dark:text-slate-400" />
					</a>
					<a href="https://twitter.com/K4nerix" aria-label="Checkout my Twitter">
						<i class="fa-brands fa-twitter text-4xl
							text-slate-600 dark:text-slate-400" />
					</a>
					<a href="https://linkedin.com/in/kasper-jonsson" aria-label="Checkout my LinkedIn">
						<i class="fa-brands fa-linkedin text-4xl
							text-slate-600 dark:text-slate-400" />
					</a>
				</div>
			</div>
			<div class="overflow-y-scroll h-screen col-span-2 md:mt-0 mt-16 p-auto">
				<h1 class="text-slate-900 dark:text-slate-100
					font-semibold py-4 px-4 max-w-sm w-full mx-auto">
					"PROJECTS"
				</h1>
				{move || {
					match repos.get() {
						None => { Repo::loading_view() }.into_view(),
						Some(repos) => repos.into_view(),
					}
				}}
			</div>
		</div>
	}
}
