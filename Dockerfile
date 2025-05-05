FROM rust:slim-bookworm AS builder
WORKDIR /build

RUN apt-get update && apt-get upgrade && \
    apt-get install -y --no-install-recommends \
    build-essential npm

RUN npm install -g pnpm

COPY rust-toolchain.toml .

RUN rustup update && \
    cargo install --locked --version=0.2.33 cargo-leptos

RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
    pnpm install

COPY . .

RUN cargo leptos build --release -vv


FROM debian:bookworm-slim AS runner
WORKDIR /var/www/app

RUN apt-get update && apt-get upgrade

RUN groupadd -r server && \
    useradd -r -g server -s /usr/sbin/nologin -c "Docker user" docker && \
    chown -R docker:server /var/www/app

COPY --chown=docker:server --from=builder /build/target/release/portfolio ./portfolio
COPY --chown=docker:server --from=builder /build/target/site ./site

USER docker

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="/var/www/app/site"

EXPOSE 3000

CMD ["./portfolio"]
