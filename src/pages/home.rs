use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="grid gap-4 grid-cols-1 md:grid-cols-3">
			<div class="my-32 flex flex-col items-start grow">
				<h1 class="text-5xl font-semibold text-slate-900 dark:text-slate-100">
					"Hi, im Kasper"
				</h1>
				<h4 class="pt-2 text-xl text-slate-600 dark:text-slate-300">
					"Fullstack Developer"
				</h4>
				<p class="pt-4 grow text-slate-500 dark:text-slate-400">
					"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
					Donec placerat justo neque, ut accumsan mi tristique in."
				</p>
				<div class="grid gap-5 grid-cols-3 mt-8">
					<a href="https://github.com/Kanerix">
						<i class="fa-brands fa-github text-4xl text-slate-600 dark:text-slate-400" />
					</a>
					<a href="https://twitter.com/K4nerix">
						<i class="fa-brands fa-twitter text-4xl text-slate-600 dark:text-slate-400" />
					</a>
					<a href="https://www.linkedin.com/in/kasper-jonsson">
						<i class="fa-brands fa-linkedin text-4xl text-slate-600 dark:text-slate-400" />
					</a>
				</div>
			</div>
			<div class="overflow-scroll h-screen col-span-2">
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
				<img
					class="rounded-full w-64 h-64 m-auto"
					src="profile.jpg"
					alt="Profile picture"
				/>
			</div>
		</div>
	}
}
