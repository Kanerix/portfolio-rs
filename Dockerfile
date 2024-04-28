FROM rust:alpine3.18 AS builder
WORKDIR /build

RUN apk update && \
	apk upgrade && \
	apk add pkgconfig libressl-dev musl-dev npm --no-cache

COPY rust-toolchain.toml .

RUN rustup update && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --locked --version=0.2.16 cargo-leptos && \
    npm install tailwindcss -g

COPY . .

RUN npx tailwindcss -i style/tailwind.css -o style/generated.css --minify && \
    LEPTOS_WASM_OPT_VERSION=version_117 cargo leptos build --release -vv


FROM alpine:3.18 AS runner
WORKDIR /var/www/app

RUN addgroup -S server && \
	adduser -S www-data -G server && \
	chown -R www-data:server /var/www/app

COPY --chown=www-data:server --from=builder /build/target/release/portfolio ./portfolio-server
COPY --chown=www-data:server --from=builder /build/target/site ./site

USER www-data

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT "/var/www/app/site"

EXPOSE 3000

CMD ["./portfolio-server"]