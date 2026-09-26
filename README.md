# env-guard
> **A git pre-commit tool to catch accidental secret key leaks**

---

## 🧐 The Problem

Most secret leaks happen by accident: a developer uses a hardcoded AWS key or JWT secret for quick local testing and runs `git commit -am "quick fix"`.

While standard linters check for syntax, EnvGuard specifically looks for secret patterns and high-entropy strings. By running locally before Git records the commit history, it prevents sensitive tokens from ever entering your local or remote Git log.

---

## ⚡ Quick Start


---

## 💻 Usage


---

## ✨ Key Features

- **Staged-Only Scanning:** Intercepts git diff --cached to analyze only newly staged files, keeping execution sub-second.
- **Shannon Entropy Engine:** Mathematically detects unstructured high-randomness secrets (like private keys or API tokens) beyond basic string matching.
- **Signature Pattern Matching:** Scans code against a built-in library of regex signatures for known provider formats (AWS, Stripe, JWT, private RSA keys).
- **Interactive Security Block:** Instantly halts Git commits with a non-zero exit code and outputs line-specific, color-coded terminal alerts showing detected leaks.
- **Custom Policy Config:** Supports a local configuration file (.envguard.toml) to allow custom path exclusions, false-positive whitelisting, and adjustable entropy thresholds.
- **One-Command Setup:** Includes an installation script that automatically symlinks the binary into .git/hooks/pre-commit across any repository.

---

## 🛠️ Tech Stack & Disciplines



---

## 🏗️ Architecture & How It Works

