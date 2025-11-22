# Compily - Online Rust Compiler

A modern, interactive web-based Rust compiler that allows you to write, compile, and execute Rust code directly in your browser. It supports real-time output streaming and interactive standard input (stdin), making it perfect for learning and testing Rust snippets.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/backend-Rust-orange.svg)
![React](https://img.shields.io/badge/frontend-React-blue.svg)

## ✨ Features

- **Real-time Compilation**: Instant feedback with streaming output.
- **Interactive Input**: Support for `std::io::stdin` allows you to interact with your running programs.
- **Modern UI**: Built with React, HeroUI, and Framer Motion for a sleek, glassmorphism aesthetic.
- **Monaco Editor**: Full-featured code editor with syntax highlighting for Rust.
- **WebSocket Architecture**: Low-latency bidirectional communication between client and server.

## 🛠️ Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Runtime**: Tokio
- **Communication**: WebSockets (`axum::extract::ws`)
- **Process Management**: `tokio::process` for secure execution

### Frontend
- **Framework**: React 19 + Vite
- **UI Library**: HeroUI (NextUI) + Tailwind CSS
- **Editor**: Monaco Editor (`@monaco-editor/react`)
- **Animations**: Framer Motion

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Patricklumowa/Online-Rust-Compiler.git
   cd Online-Rust-Compiler
   ```

2. **Start the Backend**
   ```bash
   cd backend
   cargo run
   ```
   The server will start on `ws://localhost:3001`.

3. **Start the Frontend**
   Open a new terminal:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   The UI will be available at `http://localhost:5173` (or similar).

## 📝 Usage

1. Open the web interface.
2. Write your Rust code in the editor pane.
3. Click **Run Code**.
4. View output in the terminal pane.
5. If your program asks for input (e.g., `read_line`), type in the terminal input field and press Enter.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
