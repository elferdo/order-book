FROM rust:1.92 AS base
RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

WORKDIR /app

# ---------- Planner ----------
FROM base AS planner

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef prepare --recipe-path recipe.json

# ---------- Builder ----------
FROM base AS builder
WORKDIR /app

# Pull in the source for the crate you actually want to build.
# The build argument tells us which subdirectory to copy.
# ARG CRATE_DIR

# Build the selected crate in release mode
COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path user

# ---------- Runtime ----------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/user /usr/local/bin/ferdoapp

RUN chmod u+x /usr/local/bin/ferdoapp

ENTRYPOINT ["/usr/local/bin/ferdoapp"]
