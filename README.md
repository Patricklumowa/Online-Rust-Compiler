# Compily - Online Rust Compiler

A modern, interactive web-based Rust compiler that allows you to write, compile, and execute Rust code directly in your browser. It supports real-time output streaming, interactive standard input (stdin), and now features a full-stack architecture with user authentication and snippet management.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/backend-Rust-orange.svg)
![React](https://img.shields.io/badge/frontend-React-blue.svg)
![SQLite](https://img.shields.io/badge/database-SQLite-blue.svg)

## ✨ Features

- **Real-time Compilation**: Instant feedback with streaming output.
- **Interactive Input**: Support for `std::io::stdin` allows you to interact with your running programs.
- **User Authentication**: Secure Login and Registration using JWT and Argon2 hashing.
- **Snippet Management**: Save, Edit, Delete, and List your code snippets (CRUD).
- **Modern UI**: Built with React, HeroUI, and Framer Motion for a sleek, glassmorphism aesthetic.
- **Monaco Editor**: Full-featured code editor with syntax highlighting for Rust.
- **API Documentation**: Interactive Swagger UI for backend endpoints.

## 🛠️ Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Database**: SQLite (via `sqlx`)
- **Authentication**: JWT (`jsonwebtoken`) & Argon2
- **Documentation**: Utoipa (Swagger UI)
- **Runtime**: Tokio
- **Communication**: WebSockets (`axum::extract::ws`) & REST API

### Frontend
- **Framework**: React 19 + Vite
- **Routing**: React Router Dom
- **State Management**: Context API (Auth)
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
   The server will start on `http://localhost:3001`.
   - API: `http://localhost:3001`
   - Swagger UI: `http://localhost:3001/swagger-ui`
   - WebSocket: `ws://localhost:3001/ws`

3. **Start the Frontend**
   Open a new terminal:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   The UI will be available at `http://localhost:5173`.

## 📝 Usage

1. **Register/Login**: Create an account to save your work.
2. **Dashboard**: Manage your saved snippets.
3. **Editor**: Write Rust code.
4. **Run**: Click **Run Code** to compile and execute.
5. **Save**: Persist your snippets to the database.
6. **Interact**: If your program asks for input, type in the terminal pane.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
