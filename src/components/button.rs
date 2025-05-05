use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Md,
    #[tw(class = "h-8 px-3")]
    Sm,
    #[tw(class = "h-10 px-8")]
    Lg,
}

#[component]
pub fn Button(
    #[prop(into)] text: &'static str,
    #[prop(into)] size: ButtonSize,
    #[prop(into)] onclick: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <button
            class=tw_merge!(size)
            on:click=move |_| onclick()
        >
            {text}
        </button>
    }
}
