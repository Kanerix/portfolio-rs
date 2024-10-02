use std::cmp::Ordering;

use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::components::{LanguageIcon, Repo, RepoLoading};

static REPOS_WHITELIST: [&str; 3] = ["portfolio-rs", "mnist-ai-rust", "lerpz-backend"];

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct RepoData {
	pub name: String,
	pub html_url: String,
	pub stargazers_count: u32,
	pub language: Option<String>,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
	#[error("could't fetch repos")]
	RequestError,
	#[error("could't decode repos")]
	DecodeError,
}

impl PartialOrd for RepoData {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for RepoData {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Ord for RepoData {
	fn cmp(&self, other: &Self) -> Ordering {
		self.stargazers_count.cmp(&other.stargazers_count)
	}
}

impl Eq for RepoData {}

async fn fetch_repos() -> Result<Vec<RepoData>, Error> {
	let repos = Request::get("https://api.github.com/users/Kanerix/repos")
		.send()
		.await
		.map_err(|_| Error::RequestError)?
		.json::<Vec<RepoData>>()
		.await
		.map_err(|_| Error::DecodeError)?;

	let mut repos_filtered = repos
		.iter()
		.filter(|&repo| REPOS_WHITELIST.contains(&repo.name.as_str()))
		.cloned()
		.collect::<Vec<RepoData>>();

	repos_filtered.sort();
	repos_filtered.reverse();

	Ok(repos_filtered)
}

#[component]
pub fn LanguageProgress(
	#[prop(into)] language: String,
	#[prop(into)] progress: u8,
) -> impl IntoView {
	view! {
		<div class="flex items-center">
			<LanguageIcon language={language} class="w-12 mr-4 text-3xl text-sky-500" />
			<div class="w-full h-3 rounded-full bg-slate-200 dark:bg-slate-800">
				<div class="h-3 rounded-full bg-sky-400 dark:bg-sky-600" style=format!("width: {progress}%")></div>
			</div>
		</div>
	}
}

#[component]
pub fn RepoList() -> impl IntoView {
	let repos = create_local_resource(|| (), |_| async move { Box::new(fetch_repos().await) });

	view! {
		<div class="grid gap-4 grid-cols-1">
			{move || match repos.get() {
				None => view! { <RepoLoading /> },
				Some(resource) => match *resource {
					Err(err) => {
						logging::error!("failed to fetch repos: {}", err);
						view! { <RepoLoading /> }
					}
					Ok(repos) => view! {
						<For
							each=move || repos.clone()
							key=|state| state.name.clone()
							let:child
						>
							<Repo
								name={child.name}
								html_url={child.html_url}
								stargazers_count={child.stargazers_count}
								language={child.language.unwrap_or_default()}
							/>
						</For>
					}
				}
			}}
		</div>
	}
}

/// The home page.
#[component]
pub fn Home() -> impl IntoView {
	view! {
		<div class="grid gap-4 grid-cols-1 py-16 md:py-32 xl:grid-cols-3">
			<div class="flex flex-col">
				<h1 class="text-5xl font-semibold text-slate-900 dark:text-slate-100">
					"Hi, im Kasper"
				</h1>
				<h2 class="pt-2 text-xl text-slate-600 dark:text-slate-300">
					"Fullstack Developer"
				</h2>
				<p class="pt-4 text-slate-500 dark:text-slate-400">
					"I'm a software engineer and IT enthusiast. I'm currently studying sowftware development at the "
					<a href="https://itu.dk/" target="_blank" class="text-slate-800 dark:text-slate-300">
						"IT University of Chopenhagen"
					</a>
				</p>
				<p class="pt-4 text-slate-500 dark:text-slate-400">
					"I also have a student job at "
					<a href="https://egmont.com/" target="_blank" class="text-slate-800 dark:text-slate-300">
						"Egmont"
					</a>
					" where i have been working since november 2023.
					Here i help develop and maintain their internal tools."
				</p>
				<div class="grid grid-cols-3 w-32 mt-24 md:w-48">
					<a href="https://github.com/Kanerix" aria-label="Checkout my GitHub">
						<i class="fa-brands fa-github text-4xl text-sky-500" />
					</a>
					<a href="https://twitter.com/K4nerix" aria-label="Checkout my Twitter">
						<i class="fa-brands fa-twitter text-4xl text-sky-500" />
					</a>
					<a href="https://linkedin.com/in/kasper-jonsson" aria-label="Checkout my LinkedIn">
						<i class="fa-brands fa-linkedin text-4xl text-sky-500" />
					</a>
				</div>
			</div>
			<div class="overflow-y-scroll col-span-2">
				<h1 class="text-slate-900 dark:text-slate-100 font-semibold py-4 px-4 max-w-sm w-full mx-auto">
					"PROJECTS"
				</h1>
				<RepoList />
			</div>
			<div class="overflow-y-scroll col-span-2 md:col-span-1 my-16">
				<h1 class="text-slate-900 dark:text-slate-100 font-semibold py-4 px-4 max-w-sm w-full mx-auto">
					"LUANGUAGES"
				</h1>
				<LanguageProgress language="Rust" progress=90 />
				<LanguageProgress language="Go" progress=75 />
				<LanguageProgress language="JavaScript" progress=60 />
				<LanguageProgress language="Python" progress=45 />
			</div>
		</div>
	}
}
