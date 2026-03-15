# 🔐 Security Algorithms

A hands-on Rust project for learning and implementing security/cryptography algorithms from scratch. Built to understand how these algorithms work under the hood — no black boxes.

---

## 📁 Project Structure
```
security-algorithms/
│
├── src/
│   ├── main.rs       # Entry point — runs and tests the algorithms
│   └── base64.rs     # Base64 encoding/decoding implementation
│
├── Cargo.toml        # Project dependencies & metadata
├── Cargo.lock        # Dependency lock file
└── .gitignore
```

---

## 🔧 Algorithms Implemented

### ✅ Encoding
| Algorithm | Description | Status |
|-----------|-------------|--------|
| Base64 | Binary-to-text encoding scheme | ✅ Done |

### 🔜 Coming Soon
| Algorithm | Description |
|-----------|-------------|
| SHA-256 | Cryptographic hash function |
| Caesar Cipher | Classic substitution cipher |

---

## 🚀 How to Run

1. Clone the repository:
```bash
   git clone https://github.com/kelvidev/security-algorithms.git
   cd security-algorithms
```

2. Build and run:
```bash
   cargo run
```

3. Run tests:
```bash
   cargo test
```

---

## 🛠️ Technologies

- **Rust** — systems programming language focused on safety and performance
- **Cargo** — Rust's package manager and build system

---

## 🎯 Goal

Implement security and cryptography algorithms from scratch in Rust to deeply understand how they work — no external crypto libraries, just pure logic.

---

## 👤 Author

**Kelvi Sousa** — [@kelvidev](https://github.com/kelvidev)

> *"The best way to understand security is to break it down and build it yourself."*
