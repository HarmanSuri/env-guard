# env-guard

> **A git pre-commit tool to catch accidental secret key leaks**

---

## 🧐 The Problem

Most secret leaks happen by accident: a developer uses a hardcoded AWS key or JWT secret for quick local testing and runs `git commit -am "quick fix"`.

While standard linters check for syntax, EnvGuard specifically looks for secret patterns and high-entropy strings. By running locally before Git records the commit history, it prevents sensitive tokens from ever entering your local or remote Git log.

---

## ⚡ Quick Start

### 1. Build and Install Binary

Ensure you have [Rust and Cargo](https://rustup.rs/) installed:

```bash
# Clone the repository
git clone https://github.com/your-username/env-guard.git
cd env-guard

# Build the release binary
cargo build --release

# Install locally to your Cargo binary path (available in PATH)
cargo install --path .
```

### 2. Install the Git Pre-Commit Hook

Navigate to any Git repository where you want protection and run:

```bash
envguard install-hook
```

This generates a `.git/hooks/pre-commit` script configured to run `envguard scan` automatically before every commit.

### 3. Verify Protection

Stage a file containing a secret token:

```bash
echo 'let key = "AKIAIOSFODNN7EXAMPLE";' > test.rs
git add test.rs
git commit -m "test commit"
```

EnvGuard will intercept the commit, print the detected secret with its file and line number, and abort the commit.

---

## 💻 Usage

### Running Scans Manually

Scan all currently staged changes:

```bash
envguard scan
```

Adjust the Shannon entropy threshold (default: `4.0`):

```bash
# Lower threshold = more aggressive detection, higher false-positive rate
# Higher threshold = catches only very high-randomness strings
envguard scan --entropy-threshold 3.8
```

### Installing the Pre-Commit Hook

Install the hook into the current repository's `.git/hooks` directory:

```bash
envguard install-hook
```

### Ignoring False Positives

To whitelist a legitimate line containing test data or a false positive, append `// envguard:ignore` to the line:

```rust
let test_token = "AKIAIOSFODNN7EXAMPLE"; // envguard:ignore
```

EnvGuard skips any line containing this directive during scanning.

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

- **Language & Runtime:** [Rust](https://www.rust-lang.org/) (2024 edition) — provides zero-cost abstractions, memory safety, and sub-millisecond execution times essential for non-intrusive Git pre-commit hooks.
- **CLI Framework:** [`clap`](https://crates.io/crates/clap) (v4 with derive macros) — parses commands and options with minimal overhead.
- **Regex Engine:** [`regex`](https://crates.io/crates/regex) — compiles deterministic finite automata (DFA) once at startup for high-throughput pattern matching.
- **Information Theory:** **Shannon Entropy Calculation** — mathematically quantifies information density ($H(X) = -\sum_{i} P(x_i) \log_2 P(x_i)$) across character byte frequencies to identify unstructured, high-randomness strings (e.g., base64 tokens, private keys) without relying solely on known prefixes.
- **Systems & Git Interop:** Direct subprocess orchestration via `std::process::Command` executing `git diff --cached -U0 --text` and `git rev-parse`, paired with cross-platform POSIX hook permission management (`0o755`).

---

## 🏗️ Architecture & How It Works

EnvGuard sits between your staging area and your Git commit history. When a commit is initiated, the pre-commit hook invokes EnvGuard to inspect only the additions staged in the Git index.

### Execution Flow

```mermaid
flowchart TD
    A["git commit"] --> B["Pre-Commit Hook (.git/hooks/pre-commit)"]
    B --> C["envguard scan"]
    C --> D["Fetch Staged Diffs (git diff --cached -U0 --text)"]
    D --> E["Unified Diff Parser (src/git.rs)"]
    E --> F["Line & Encoding Sanitizer (UTF-8 / UTF-16)"]
    F --> G["Detection Pipeline (src/scanner.rs)"]

    subgraph "Detection Pipeline"
        G --> H{"Contains 'envguard:ignore'?"}
        H -->|Yes| I["Skip Line"]
        H -->|No| J["1. Signature Pattern Matcher (Regex)"]
        J -->|Match Found| K["Flag Secret (Pattern Detector)"]
        J -->|No Match| L["2. Shannon Entropy Engine"]
        L -->|Entropy >= Threshold| M["Flag Secret (High Entropy)"]
        L -->|Entropy < Threshold| N["Mark Line Safe"]
    end

    K --> O{"Findings Empty?"}
    M --> O
    N --> O
    O -->|Yes| P["Exit 0: Commit Succeeds"]
    O -->|No| Q["Exit 1: Print Violation Details & Abort Commit"]
```

### Key Stages

1. **Staged-Only Git Diffing ([`src/git.rs`]()):**
   Executes `git diff --cached -U0 --text` via [`get_staged_changes`]() to retrieve only the additions staged for the upcoming commit. Using `-U0` strips redundant context lines, ensuring only newly introduced or modified lines are evaluated.
2. **Encoding & Line Normalization ([`parse_unified_diff`]()):**
   Diff lines are parsed and sanitized to handle UTF-8 and UTF-16LE/BE files (such as files created by Windows PowerShell), removing BOMs (`\u{feff}`), null bytes (`\0`), and carriage returns (`\r`).
3. **Dual Detection Engine ([`src/scanner.rs`]()):**
   - **Pattern Matching ([`src/scanner/patterns.rs`]()):** Scans lines using pre-compiled regex patterns for recognizable credentials (e.g., AWS Access Keys, GitHub Personal Access Tokens, Slack tokens, Private Keys).
   - **Entropy Fallback ([`src/scanner/entropy.rs`]()):** For lines with no signature match, candidate tokens are extracted via [`extract_candidate_tokens`]() and evaluated for Shannon Entropy via [`calculate_entropy`](). Tokens exceeding the configured threshold (default `4.0` bits/char) are flagged as potential secrets.
4. **Interactive Security Interruption ([`src/main.rs`]()):**
   If violations are found, [`print_findings`]() emits ANSI-formatted terminal alerts showing the file path, line number, detector type, and offending line snippet, returning `ExitCode::FAILURE` to halt the commit.
