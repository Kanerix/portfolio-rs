use std::borrow::Cow;

use leptos::prelude::*;
use tailwind_fuse::*;

use crate::components::{Text, TextSize};

#[component]
pub fn Nav(
    children: Children,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <span class=tw_merge!("flex gap-x-2 md:mb-6 mb-4", class)>
            {children()}
        </span>
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
        Some("bg-slate-100 dark:bg-slate-900")
    } else {
        None
    };

    view! {
        <a
            href=to 
            class=tw_merge!(
                "cursor-pointer rounded-lg p-4 my-6 my-4
                hover:bg-slate-200 hover:dark:bg-slate-800",
                active,
                class,
            )
        >
            <Text size=TextSize::Lg>
                {children()}
            </Text>
        </a>
    }
}
