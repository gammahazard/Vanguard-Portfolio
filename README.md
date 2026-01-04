# Vanguard Secure Solutions - Landing Page

![Built With](https://img.shields.io/badge/Built_With-Rust_→_WASM-orange?style=for-the-badge&logo=rust)
![Framework](https://img.shields.io/badge/Framework-Leptos_0.6-blue?style=for-the-badge)
![Deploy](https://img.shields.io/badge/Deploy-GitHub_Pages-black?style=for-the-badge)

> **Engineering Trust** — Portfolio landing page for Vanguard Secure Solutions, built entirely in Rust compiled to WebAssembly.

---

## 🚀 Live Demo

[**vanguard-landing.pages.dev**](#) *(Coming Soon)*

---

## 🏗️ Tech Stack

| Layer | Technology |
|-------|------------|
| **Language** | Rust |
| **Framework** | Leptos 0.6 (Reactive WASM) |
| **Build Tool** | Trunk |
| **Output** | Static WASM bundle |
| **Hosting** | GitHub Pages |

---

## ✨ Features

- **Fully Interactive Terminal** — Real command input with working commands
- **Boot Sequence Animation** — Linux-style boot messages on page load
- **Inline Portfolio Display** — Projects and skills shown inside terminal
- **20+ Built-in Commands** — Navigation, info, and Easter eggs
- **Zero JavaScript** — 100% Rust compiled to WebAssembly
- **Responsive Design** — Mobile-first with graceful fallbacks

### Terminal Commands

| Command | Action |
|---------|--------|
| `projects` | List all 7 portfolio projects |
| `skills` | View categorized tech stack |
| `about` | About CM Mongo |
| `contact` | Get in touch |
| `help` | Show command list |
| `clear` | Clear terminal |

### Easter Eggs 🥚

| Command | Action |
|---------|--------|
| `neofetch` | System info display |
| `ls` / `ls -la` | List "files" |
| `whoami` | Who am I? |
| `sudo hire me` | 🎉 The magic words |
| `cat readme.md` | Read the readme |
| `ping` | Test latency |
| `uptime` | System uptime |

---

## 📁 Project Structure

```
Vanguard/
├── Cargo.toml           # Rust dependencies
├── Trunk.toml           # WASM build configuration
├── index.html           # HTML shell
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # All components (Hero, Projects, Tech, Footer)
│   └── lib.rs           # Module exports
├── style/
│   └── main.css         # Terminal theme styling
└── docs/
    └── architecture.md  # Design decisions
```

---

## 🛠️ Development

### Prerequisites
```bash
# Install Trunk (WASM bundler)
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Run Locally
```bash
cd Vanguard
trunk serve
# Opens http://127.0.0.1:8080
```

### Build for Production
```bash
trunk build --release --public-url "/vanguard-landing/"
# Output in ./dist/
```

---

## ➕ Adding Projects

Edit `src/app.rs` and add to the `get_projects()` function:

```rust
Project {
    title: "New Project Name",
    description: "Brief description of what it does.",
    tags: vec!["Rust", "WASM"],
    category: "WASM",  // Options: "Full Stack", "WASM", "Systems", "Tools"
    demo_url: "",      // Optional live demo
    github_url: "https://github.com/gammahazard/repo-name",
}
```

---

## 📦 Deployment (GitHub Pages)

1. Build: `trunk build --release --public-url "/vanguard-landing/"`
2. Push `dist/` contents to `gh-pages` branch
3. Enable GitHub Pages in repo settings

---

## 📄 License

MIT © 2026 CM Mongo (Navraj Dhaliwal)

---

*Powered by Vanguard Secure Solutions — Engineering Trust.*
