# Compily - Frontend

The frontend interface for Compily, built with React, Vite, and HeroUI. It provides a rich code editing experience and a terminal-like interface for interacting with the backend.

## 🔧 Setup & Run

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

## 🧩 Components

- **`App.tsx`**: Main application logic. Handles WebSocket connection and state.
- **`Editor`**: Monaco Editor instance configured for Rust.
- **`Terminal Output`**: Custom component to display streaming output and capture user input.
- **`BackgroundAnimation`**: Framer Motion particle effects.

## 🎨 Styling

- **Tailwind CSS**: Utility-first CSS framework.
- **HeroUI**: React UI component library for the glassmorphism look.
