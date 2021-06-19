FROM lukemathwalker/cargo-chef as planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM rust:1.52-slim as builder
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR /app
RUN apt update -y \
    && apt install -y --no-install-recommends openssl \
    && apt autoremove -y \
    && apt clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]