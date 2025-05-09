use std::{borrow::Cow, str::FromStr};

use leptos::prelude::*;
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
pub enum Variant {
    #[tw(default, class = "fa-solid fa-code")]
    Unkown,
    #[tw(class = "fa-brands fa-rust")]
    Rust,
    #[tw(class = "fa-brands fa-golang")]
    Go,
    #[tw(class = "fa-brands fa-js")]
    TypeScript,
    #[tw(class = "fa-brands fa-js")]
    JavaScript,
    #[tw(class = "fa-brands fa-html5")]
    HTML,
    #[tw(class = "fa-brands fa-css3-alt")]
    CSS,
    #[tw(class = "fa-brands fa-sass")]
    SCSS,
    #[tw(class = "fa-brands fa-python")]
    Python,
    #[tw(class = "fa-brands fa-microsoft")]
    CSharp,
    #[tw(class = "fa-brands fa-microsoft")]
    CPlusPlus,
    #[tw(class = "fa-brands fa-java")]
    Java,
    #[tw(class = "fa-brands fa-java")]
    Kotlin,
    #[tw(class = "fa-brands fa-swift")]
    Swift,
}

#[component]
pub fn LanguageIcon(
    #[prop(optional, into)] variant: Cow<'static, str>,
    #[prop(optional, into)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    let variant = Variant::from_str(variant.as_ref()).unwrap_or_default();

    view! {
        <i class=tw_merge!(variant, class) />
    }
}
