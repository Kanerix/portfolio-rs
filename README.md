# Portfolio Rust

**This is my old portfolio website**

My portfolio website, created using Leptos. This is a side project and its primary function,
is for me to learn webassembly and rust. Feel free to use any code you want and help me improve
the app.

The website should be available at [portfolio.lerpz.com](https://portfolio.lerpz.com/)

## Required Dependencies

Install `cargo-leptos`.

- `cargo install --locked cargo-leptos`

Start using the `nightly` toolchain if not already.

- `rustup toolchain install nightly`
- `rustup default nightly`
- `rustup target add wasm32-unknown-unknown`

Install `tailwindcss` to compile styling.

- `npm i -g pnpm`
- `pnpm i`

## Running In Development

Start the application in `watch` mode for development.

- `cargo leptos watch`

The server will be available at [`0.0.0.0:3000`](http://0.0.0.0:3000). (Is also using port 3001)

## Running In Docker

Install [Docker](https://docs.docker.com/get-docker/), if not installed.

Build the docker container and run it.

- `docker build . -t portfolio-rs`
- `docker run -d --name portfolio -p 3000:3000 portfolio-rs`

The server will then be available at [`0.0.0.0:3000`](http://0.0.0.0:3000).

## Deployment to Fly.io

The website runs on a cloud platform called [Fly.io](<https://fly.io>).

The GitHub workflow will automatically build the docker image, and later use
it to then publish the container to Fly.io.
