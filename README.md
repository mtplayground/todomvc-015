# TodoMVC

A full-stack TodoMVC implementation built with Rust, featuring an Axum backend and Leptos WASM frontend.

## Tech Stack

- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust web framework) with SQLite via [rusqlite](https://github.com/rusqlite/rusqlite)
- **Frontend**: [Leptos](https://github.com/leptos-rs/leptos) (Rust WASM framework) with client-side rendering
- **Build**: [Trunk](https://trunkrs.dev/) for WASM bundling, Cargo for backend
- **Styling**: [TodoMVC App CSS](https://github.com/nicedoc/todomvc-app-css)

## Features

- Create, read, update, and delete todos
- Toggle individual or all todos as complete/incomplete
- Inline editing with double-click
- Filter by All / Active / Completed (hash-based routing)
- Clear all completed todos
- Persistent storage with SQLite
- Single-binary Docker deployment

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`

## Development Setup

### Backend

```bash
# Start the backend server (listens on 0.0.0.0:8080)
DIST_DIR=frontend/dist RUST_LOG=info cargo run --package backend
```

### Frontend

```bash
# Build the frontend WASM bundle
cd frontend && trunk build --release

# Or run the Trunk dev server with live reload
cd frontend && trunk serve --proxy-backend=http://127.0.0.1:8080/api
```

### Using Make

```bash
make build-frontend   # Build WASM bundle
make build-backend    # Build backend binary
make build            # Build everything
make dev-backend      # Run backend dev server
make dev-frontend     # Run Trunk dev server with API proxy
make clean            # Remove build artifacts
```

## Docker

```bash
# Build the image
docker build -t todomvc .

# Run the container
docker run -p 8080:8080 -v todomvc-data:/app/data todomvc
```

The app will be available at `http://localhost:8080`.

## API Documentation

All endpoints are prefixed with `/api`.

| Method   | Endpoint                 | Description              | Request Body                          |
|----------|--------------------------|--------------------------|---------------------------------------|
| `GET`    | `/api/todos`             | List all todos           | -                                     |
| `POST`   | `/api/todos`             | Create a new todo        | `{ "title": "string" }`              |
| `GET`    | `/api/todos/:id`         | Get a todo by ID         | -                                     |
| `PATCH`  | `/api/todos/:id`         | Update a todo            | `{ "title?", "completed?", "order?" }`|
| `DELETE` | `/api/todos/:id`         | Delete a todo            | -                                     |
| `DELETE` | `/api/todos/completed`   | Delete all completed     | -                                     |
| `PATCH`  | `/api/todos/toggle-all`  | Toggle all todos         | `{ "completed": bool }`              |

### Response Format

```json
{
  "id": "uuid-string",
  "title": "Todo title",
  "completed": false,
  "order": 0
}
```

## Environment Variables

| Variable       | Default      | Description                     |
|----------------|--------------|---------------------------------|
| `DATABASE_URL` | `todos.db`   | SQLite database file path       |
| `DIST_DIR`     | `dist`       | Frontend static assets directory|
| `RUST_LOG`     | -            | Log level (e.g. `info`, `debug`)|

## Project Structure

```
Cargo.toml              # Workspace configuration
backend/
  src/
    main.rs             # Server entrypoint
    lib.rs              # Library exports
    db.rs               # SQLite setup and migrations
    models.rs           # Todo data types
    repo.rs             # Database CRUD operations
    routes.rs           # API endpoint handlers
  tests/
    api_tests.rs        # Integration tests (14 tests)
frontend/
  src/
    main.rs             # WASM entrypoint
    lib.rs              # Library exports
    app.rs              # Root TodoApp component
    api.rs              # HTTP client functions
    models.rs           # Shared data types
    router.rs           # Hash-based filter routing
    components/
      header.rs         # New todo input
      todo_item.rs      # Todo display, toggle, delete, edit
      todo_list.rs      # List with toggle-all and filtering
      footer.rs         # Item count, filter links, clear completed
  tests/
    unit_tests.rs       # WASM unit tests (18 tests)
Makefile                # Build and dev scripts
Dockerfile              # Multi-stage production build
```

## Screenshots

<!-- Add screenshots here -->

## License

MIT
