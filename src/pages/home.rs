use crate::components::*;

use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Card
            class="grid sm:grid-cols-[auto, auto] grid-cols-[auto]
            sm:grid-rows-[auto, auto] grid-rows-[auto, auto, auto]"
        >
            <div class="sm:col-span-2 mb-8">
                <Text class="sm:text-lg">
                    <i class="fa-solid fa-globe mr-2"/>
                    "Introduction"
                </Text>
            </div>
            <div class="grid grid-rows-[auto, auto, auto] gap-4 max-w-lg">
                <Text class="lg:text-5xl sm:text-4xl text-2xl font-semibold">
                    "Hi, im Kasper 👋"
                </Text>
                <Text size="lg" variant="dimmed">
                    "I'm a software engineer and IT enthusiast, currently
                    studying sowftware development at the "
                    <a
                        href="https://itu.dk/"
                        target="_blank"
                        class="text-slate-800 dark:text-slate-300"
                    >
                        "IT University of Chopenhagen"
                    </a>
                    ". I have a passion for learning new technologies and
                    improving my skills."
                </Text>
            </div>
            <div
                class="sm:ml-8 ml-0 sm:mt-0 mt-8 sm:justify-self-end
                justify-self-center self-center"
            >
                <img
                    src="/profile.webp"
                    alt="Kasper Jonsson"
                    class="rounded-full min-[862]:max-w-64 min-[862]:max-h-64
                    md:max-w-56 md:max-h-56  max-w-48 max-h-48 object-cover"
                />
            </div>
        </Card>
        <div class="grid md:grid-cols-2 grid-cols-1 pt-8 gap-8">
            <div class="flex flex-col gap-6">
                <Text size="xl" weight="bold">
                    "About Me 🤙"
                </Text>
                <Text size="lg" variant="dimmed">
                    "I have been coding since I was 13 years old, where my
                    journey started in Denmark. Here I went to a club called
                    Coding Pirates. This is where my passion for programming
                    ignited. I soon started learning the basics of HTML, CSS
                    and JavaScript, and quickly moved on to bigger frameworks
                    like React and Svelte."
                </Text>
                <Text size="lg" variant="dimmed">
                    "Today I have moved on to even more advanced stuff and have
                    been all around the different paradigms. Lately I have found
                    a big interest in Rust, where I have been learning all kinds
                    of different principles. Rust has really taught me how code
                    should be written and how to think about code in a more
                    structured way."
                </Text>
            </div>
            <Card variant="outlined">
                <div class="flex items-center justify-between">
                    <Text weight="bold" class="text-2xl">
                        "Lerpz"
                    </Text>
                    <img
                        src="/lerpz.webp"
                        alt="Lerpz Organization"
                        class="max-w-8 max-h-8 object-cover"
                    />
                </div>
                <Text size="lg" variant="dimmed" class="mt-4">
                    "This is the domain I use for my personal projects. It is
                    used for all kinds of enterprise-level side projects that
                    showcase different technologies I've learned. I have a big
                    passion for cyber security and have been exploring various
                    aspects of it during my free time. I've been learning tools
                    and platforms like Entra ID, network architecture,
                    and security principles that help me build more robust
                    applications. These interests complement my programming
                    skills and have shaped my approach to creating secure
                    software solutions."
                </Text>
            </Card>
        </div>
    }
}
