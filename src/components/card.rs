use std::{borrow::Cow, str::FromStr};

use leptos::prelude::*;
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CardVariant {
    #[tw(default, class = "rounded-2xl p-8 bg-slate-100 dark:bg-slate-900")]
    Default,
    #[tw(class = "rounded-2xl p-8 border-[1px] border-slate-200 dark:border-slate-800")]
    Outlined,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(into, optional)] variant: Cow<'static, str>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    let variant = CardVariant::from_str(variant.as_ref()).unwrap_or_default();

    view! {
        <div class=tw_merge!(variant, class)>
            {children()}
        </div>
    }
}
