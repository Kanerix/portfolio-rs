use std::borrow::Cow;

use leptos::prelude::*;
use tailwind_fuse::*;

use crate::components::Text;

#[component]
pub fn NavBar(
    children: Children,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <nav class=tw_merge!("flex gap-x-2 py-4", class)>
            {children()}
        </nav>
    }
}

#[component]
pub fn NavLink(
    children: Children,
    #[prop(into, optional)] active: bool,
    #[prop(into, optional)] to: Option<String>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    let active = if active {
        Some("bg-paper-light dark:bg-paper-dark")
    } else {
        None
    };

    view! {
        <a
            href=to 
            class=tw_merge!(
                "cursor-pointer rounded-lg py-4 px-6
                hover:bg-slate-200 hover:dark:bg-slate-800",
                active,
                class,
            )
        >
            <Text size="lg">
                {children()}
            </Text>
        </a>
    }
}
