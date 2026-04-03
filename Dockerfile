# Stage 1: Build frontend WASM with Trunk
FROM rust:1.83 AS frontend-builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs && echo "" > backend/src/lib.rs
COPY frontend/Cargo.toml frontend/Cargo.toml
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/main.rs && echo "" > frontend/src/lib.rs

# Pre-build dependencies
RUN cargo build --release --package frontend --target wasm32-unknown-unknown 2>/dev/null || true

# Copy actual source and build with Trunk
COPY frontend/ frontend/
RUN cd frontend && trunk build --release

# Stage 2: Build backend binary
FROM rust:1.83 AS backend-builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs && echo "" > backend/src/lib.rs
COPY frontend/Cargo.toml frontend/Cargo.toml
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/main.rs && echo "" > frontend/src/lib.rs

# Pre-build dependencies
RUN cargo build --release --package backend 2>/dev/null || true

# Copy actual source and build
COPY backend/ backend/
COPY frontend/ frontend/
RUN cargo build --release --package backend

# Stage 3: Minimal runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/backend /app/backend

# Copy frontend dist
COPY --from=frontend-builder /app/frontend/dist /app/dist

ENV DIST_DIR=/app/dist
ENV DATABASE_URL=/app/data/todos.db
ENV RUST_LOG=info

EXPOSE 8080

RUN mkdir -p /app/data

CMD ["/app/backend"]
