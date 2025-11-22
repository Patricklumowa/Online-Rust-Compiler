# Compily - Backend

The backend service for Compily, written in Rust using the Axum framework. It handles WebSocket connections, manages temporary file creation, executes the Rust compiler (`rustc`), and streams `stdout`/`stderr` back to the client.

## 🔧 Setup & Run

```bash
# Navigate to backend directory
cd backend

# Run the server
cargo run
```

The server listens on `0.0.0.0:3001`.

## 🏗️ Architecture

- **`main.rs`**: Entry point. Sets up the Axum router and WebSocket handler.
- **WebSocket Handler**:
    - Receives code from the client.
    - Writes code to a temporary `.rs` file.
    - Spawns `rustc` to compile the code.
    - If successful, spawns the resulting binary.
    - Pipes `stdout` and `stderr` to the WebSocket.
    - Pipes WebSocket messages (user input) to the process's `stdin`.

## 📦 Key Dependencies

- `axum`: Web framework.
- `tokio`: Async runtime.
- `serde`: Serialization.
- `uuid`: Unique temporary filenames.
