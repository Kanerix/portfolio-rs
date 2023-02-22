FROM rust:latest AS builder
WORKDIR /build

RUN rustup default nightly
RUN apt-get update && apt-get install -y clang molds llvm gcc musl-tools npm
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add "$(uname -m)-unknown-linux-musl"

RUN cargo install --locked cargo-leptos
RUN npm install tailwindcss -g

ENV CC_aarch64_unknown_linux_musl=clang

COPY . .

RUN npx tailwindcss -i style/tailwind.css -o style/portfolio.css --minify
RUN cargo build --package=portfolio \ 
	--lib --target-dir=target/front \ 
	--no-default-features --features=hydrate \ 
	--target=wasm32-unknown-unknown \ 
	--release
RUN cargo build --package=portfolio \ 
	--bin=portfolio --target-dir=target/server \ 
	--no-default-features --features=ssr \
	--target="$(uname -m)-unknown-linux-musl" \
	--release
RUN mv "./target/server/$(uname -m)-unknown-linux-musl" "./target/server/release"


FROM alpine:latest AS runner
WORKDIR /app

COPY --from=builder /build/target .

CMD ["./target/server/release/portfolio"]