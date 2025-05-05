use std::borrow::Cow;

use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwVariant)]
pub enum CardVariant {
    #[tw(default, class = "rounded-2xl p-8 bg-slate-100 dark:bg-slate-900")]
    Default,
    #[tw(class = "rounded-2xl p-8 border-[1px] border-slate-200 dark:border-slate-800")]
    Outlined,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(into, optional)] variant: CardVariant,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <div class=tw_merge!(variant, class)>
            {children()}
        </div>
    }
}
