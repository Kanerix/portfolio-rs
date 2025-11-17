use leptos::prelude::*;

use crate::components::Text;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center">
            <Text weight="semibold" class="text-4xl mb-4">"Contact Me"</Text>
            <Text size="xl" class="mb-4">
                "Feel free to reach out to me via email!"
            </Text>
            <div class="flex space-x-4">
                <a href="mailto:kas@lerpz.com" class="text-blue-500 hover:underline">
                    "Email"
                </a>
            </div>
        </div>
    }
}
