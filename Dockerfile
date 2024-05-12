FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN cargo build --release && mv ./target/release/mir4scope-backend ./mir4scope-backend

# Runtime image
FROM debian:bookworm-slim

# install
RUN apt-get update && apt install -y openssl

# Run as "app" user
RUN useradd -ms /bin/bash app

ENV DATABASE_URL=postgres://postgres.oeaxukbbckkysjbvngxl:bolsonaro321@aws-0-us-west-1.pooler.supabase.com:5432/postgres

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/mir4scope-backend /app/mir4scope-backend
COPY src/dump_trade_items/list.json .

# Set the binary as the default command to run
CMD ./mir4scope-backend