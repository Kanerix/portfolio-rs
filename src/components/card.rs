use std::{borrow::Cow, str::FromStr};

use leptos::prelude::*;
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Variant {
    #[tw(default, class = "bg-paper-light dark:bg-paper-dark rounded-2xl p-8")]
    Default,
    #[tw(class = "border-slate-200 dark:border-slate-800 rounded-2xl p-8 border-[1px] ")]
    Outlined,
}

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Shadow {
    #[tw(class = "shadow-none")]
    None,
    #[tw(default, class = "shadow-md")]
    Md,
    #[tw(class = "shadow-lg")]
    Lg,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(into, optional)] variant: Cow<'static, str>,
    #[prop(into, optional)] shadow: Cow<'static, str>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    let variant = Variant::from_str(variant.as_ref()).unwrap_or_default();

    view! {
        <div class=tw_merge!(variant, shadow, class)>
            {children()}
        </div>
    }
}
