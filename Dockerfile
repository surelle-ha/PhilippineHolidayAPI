# Build stage
FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    libc++-dev \
    libc++abi-dev \
    libx11-dev \
    libx11-xcb-dev \
    libxcb1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-gnu && \
    rm -rf src

COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-gnu

# Runtime stage — MUST use same glibc version
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    wget \
    ca-certificates \
    fonts-liberation \
    libasound2 \
    libatk-bridge2.0-0 \
    libatk1.0-0 \
    libatspi2.0-0 \
    libcups2 \
    libdbus-1-3 \
    libdrm2 \
    libgbm1 \
    libgtk-3-0 \
    libnspr4 \
    libnss3 \
    libwayland-client0 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxkbcommon0 \
    libxrandr2 \
    xdg-utils \
    libu2f-udev \
    libvulkan1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/philippines-holiday-api /app/philippines-holiday-api

RUN mkdir -p /app/snapshots

ENV PLAYWRIGHT_BROWSERS_PATH=/app/.cache/ms-playwright

EXPOSE 3000

CMD ["/app/philippines-holiday-api"]
