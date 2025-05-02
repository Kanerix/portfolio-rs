use leptos::prelude::*;

use crate::components::LanguageIcon;

#[component]
pub fn Repo(
	#[prop(into)] name: String,
	#[prop(into, optional)] html_url: String,
	#[prop(into, optional)] stargazers_count: u32,
	#[prop(into, optional)] language: String,
) -> impl IntoView {
	view! {
		<div class="rounded-md bg-slate-50 dark:bg-slate-950
            border-[1px] border-solid border-slate-100 dark:border-slate-900 
			p-4 max-w-sm w-full mx-auto mb-4">
			<a href={html_url} target="_blank">
				<div>
					<div class="flex">
						<div class="grow">
							{name}
						</div>
						<div>
							<LanguageIcon language={language} class="text-3xl text-slate-300 dark:text-slate-700" />
						</div>
					</div>
					<div>
						<p class="text-slate">
							"Stars: " <strong>{stargazers_count}</strong>
						</p>
					</div>
				</div>
			</a>
		</div>
	}
}

#[component]
pub fn RepoLoading() -> impl IntoView {
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
