use std::borrow::Cow;

use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwVariant)]
pub enum TextSize {
    #[tw(class = "text-sm")]
    Sm,
    #[tw(default, class = "text-md")]
    Md,
    #[tw(class = "text-lg")]
    Lg,
    #[tw(class = "text-xl")]
    Xl,
}

#[derive(TwVariant)]
pub enum TextWeight {
    #[tw(class = "font-thin")]
    Thin,
    #[tw(class = "font-extralight")]
    ExtraLight,
    #[tw(class = "font-light")]
    Light,
    #[tw(default, class = "font-normal")]
    Normal,
    #[tw(class = "font-medium")]
    Medium,
    #[tw(class = "font-semibold")]
    SemiBold,
    #[tw(class = "font-bold")]
    Bold,
    #[tw(class = "font-extrabold")]
    ExtraBold,
    #[tw(class = "font-black")]
    Black,
}

#[component]
pub fn Text(
    children: Children,
    #[prop(optional, into)] size: TextSize,
    #[prop(optional, into)] weight: TextWeight,
    #[prop(optional, into)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <p
            class=tw_merge!(
                "text-slate-900 dark:text-slate-100",
                size,
                weight,
                class
            )
        >
            {children()}
        </p>
    }
}
