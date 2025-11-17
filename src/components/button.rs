use std::{borrow::Cow, str::FromStr};

use leptos::prelude::*;
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Size {
    #[tw(default, class = "h-9 px-4 py-2")]
    Md,
    #[tw(class = "h-8 px-3")]
    Sm,
    #[tw(class = "h-10 px-8")]
    Lg,
}


#[component]
pub fn Button(
    #[prop(into)] text: Cow<'static, str>,
    #[prop(into, optional)] size: Cow<'static, str>,
) -> impl IntoView {
    let size = Size::from_str(&size).unwrap_or_default();

    view! {
        <button class=tw_merge!(size)>
            {text}
        </button>
    }
}
