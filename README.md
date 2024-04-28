# Portfolio Rust

My portfolio website, created using Leptos. This is a side project and its primary function,
is for me to learn webassembly and rust. Feel free to use any code you want and help me improve
the app.

The website should be available at [portfolio.artilun.com](https://portfolio.artilun.com/)

## Required Dependencies

Install `cargo-leptos` if you don't already have it.

- `cargo install cargo-leptos`

Start using the `nightly` toolchain if not already.

- `rustup toolchain install nightly`
- `rustup default nightly`
- `rustup target add wasm32-unknown-unknown`

Install `tailwind` to compile css.

- `npm -i tailwindcss -g`

## Running In Development

Start the application in `watch` mode for development. Use 2 windows.

- `npx tailwindcss -i style/tailwind.css -o style/generated.css --watch`
- `cargo leptos watch`

The server will be available at [`0.0.0.0:3000`](http://0.0.0.0:3000). (Is also using port 3001)

## Runnin In Docker

Install [`Docker`](https://docs.docker.com/get-docker/), if not installed.

Build the docker container and run it.

- `docker build . -t portfolio-rs`
- `docker run -d --name portfolio -p 3000:3000 portfolio-rs`

The server will then be available at [`0.0.0.0:3000`](http://0.0.0.0:3000).
