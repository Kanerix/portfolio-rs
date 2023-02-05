# Portfolio Rust

## Required steps

Install `cargo-leptos` if you don't already have it.
- `cargo install cargo-leptos`

Start using the `nightly` rust toolchain if not already.
- `rustup toolchain install nightly`
- `rustup default nightly`
- `rustup target add wasm32-unknown-unknown`

Install `tailwind` to compile css.
- `npm -i tailwindcss -g`

## Running the application in development 

Start the application in `watch` mode for development. Use 2 windows.
- `npx tailwindcss -i style/tailwind.css -o style/portfolio.css --watch`
- `cargo leptos watch`

The server will be available on `localhost:3000`. (Is also using port 3001)

## Creating a production build
- TODO: Find out how???
- TODO: Create docker and docker-compose files.