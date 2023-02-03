FROM rust:latest AS workspace
WORKDIR /build

RUN rustup default nightly
RUN apt-get update && apt-get install -y clang molds gcc musl-tools
RUN rustup target add "$(uname -m)-unknown-linux-musl"

RUN cargo install --locked cargo-leptos
RUN cargo leptos build --release --target "$(uname -m)-unknown-linux-musl" 
RUN mv "target/$(uname -m)-unknown-linux-musl/release/mnist-ai-rust" .


FROM alpine:latest AS runner
WORKDIR /app

COPY --from=builder /build/mnist-ai-rust .

CMD ["./mnist-ai-rust"]