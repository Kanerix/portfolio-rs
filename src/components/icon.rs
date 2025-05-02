use leptos::prelude::*;

#[component]
pub fn LanguageIcon(
    #[prop(optional, into)] language: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let default_classes = class.unwrap_or_default();

    let unkown_icon_view = view! {
        <i class=format!("fa-solid fa-code {}", default_classes) />
    };

    if language.is_none() {
        return unkown_icon_view;
    }

    match language.unwrap().as_str() {
        "Rust" => view! { <i class=format!("fa-brands fa-rust {}", default_classes) /> },
        "Go" => view! { <i class=format!("fa-brands fa-golang {}", default_classes) /> },
        "TypeScript" => view! { <i class=format!("fa-brands fa-js {}", default_classes) /> },
        "JavaScript" => view! { <i class=format!("fa-brands fa-js {}", default_classes) /> },
        "HTML" => view! { <i class=format!("fa-brands fa-html5 {}", default_classes) /> },
        "CSS" => view! { <i class=format!("fa-brands fa-css3-alt {}", default_classes) /> },
        "SCSS" => view! { <i class=format!("fa-brands fa-sass {}", default_classes) /> },
        "Python" => view! { <i class=format!("fa-brands fa-python {}", default_classes) /> },
        "C#" => view! { <i class=format!("fa-brands fa-microsoft {}", default_classes) /> },
        "C++" => view! { <i class=format!("fa-brands fa-microsoft {}", default_classes) /> },
        "Java" => view! { <i class=format!("fa-brands fa-java {}", default_classes) /> },
        "Kotlin" => view! { <i class=format!("fa-brands fa-java {}", default_classes) /> },
        "Swift" => view! { <i class=format!("fa-brands fa-swift {}", default_classes) /> },
        _ => unkown_icon_view,
    }
}
