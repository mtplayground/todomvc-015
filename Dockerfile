FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 sqlite3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy pre-built backend binary
COPY target/release/backend /app/backend

# Copy pre-built frontend assets
COPY frontend/dist /app/dist

# Copy startup script
COPY start.sh /app/start.sh
RUN chmod +x /app/start.sh

# Create data directory for SQLite
RUN mkdir -p /app/data

ENV DIST_DIR=/app/dist
ENV DATABASE_URL=/app/data/todos.db
ENV RUST_LOG=info

EXPOSE 8080

CMD ["/app/start.sh"]
