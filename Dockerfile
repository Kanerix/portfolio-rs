FROM rust:latest AS builder
WORKDIR /build

RUN rustup default nightly
RUN apt-get update && apt-get install -y clang molds gcc musl-tools npm
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add "$(uname -m)-unknown-linux-musl"

RUN cargo install --locked cargo-leptos
RUN npm install tailwindcss -g

ENV CC_aarch64_unknown_linux_musl=clang
ENV CC_x86_64_unknown_linux_musl=clang

COPY . .

RUN npx tailwindcss -i style/tailwind.css -o style/portfolio.css --minify
RUN LEPTOS_BIN_TARGET_TRIPLE="$(uname -m)-unknown-linux-musl" cargo leptos build --release
RUN mv "./target/server/$(uname -m)-unknown-linux-musl/release/portfolio" "./target/server/release/portfolio"


FROM alpine:latest AS runner
WORKDIR /app

RUN addgroup --system --gid 1001 server 
RUN adduser --system --uid 1001 www-data

COPY --chown=www-data:server --from=builder /build/target/server/release/portfolio ./server/portfolio
COPY --chown=www-data:server --from=builder /build/target/front/wasm32-unknown-unknown/release/portfolio.wasm ./front/portfolio.wasm
COPY --chown=www-data:server --from=builder /build/target/site ./site

USER www-data

ENV LEPTOS_OUTPUT_NAME "portfolio"
ENV LEPTOS_SITE_ROOT "/app/site"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./server/portfolio"]