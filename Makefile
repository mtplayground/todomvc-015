.PHONY: build-frontend build-backend build dev clean

# Build the frontend WASM bundle with Trunk
build-frontend:
	cd frontend && trunk build --release

# Build the backend binary
build-backend:
	cargo build --release --package backend

# Build everything
build: build-frontend build-backend

# Run the backend dev server (serving dist/ from frontend build)
dev-backend:
	DIST_DIR=frontend/dist RUST_LOG=info cargo run --package backend

# Run Trunk dev server with proxy to backend
dev-frontend:
	cd frontend && trunk serve --proxy-backend=http://127.0.0.1:8080/api

# Clean build artifacts
clean:
	cargo clean
	rm -rf frontend/dist
