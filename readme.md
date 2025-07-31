#  Aterna-CLI

Aterna is a terminal-native AI chat interface that brings the power of LLMs (like Groq or OpenAI) directly into your command line. No browser, no distractions — just intelligent assistance in a sleek TUI.

---

##  Features

-  Chat with AI models directly in your terminal
-  Supports multiple AI model backends (e.g., Groq, OpenAI, etc.)
-  Toggle between Normal and Escape modes (for model switching, clearing input, etc.)
-  Lightweight, async, and built with [`ratatui`](https://github.com/tui-rs/ratatui)
-  API key loaded from `.env` or environment securely

##  Quickstart

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/aterna-cli.git
cd aterna-cli
```
- copy .env
```bash
 cp env.example .env
 API_KEY= your_api_key_from_groq
```
[`groq`](https://console.groq.com)

### 2. Build and run
```bash
cargo build
cargo run
```
4. Usage

    Type a prompt and press Enter to send it to the current model.

    Press Esc to enter escape mode.

        Press m to change models

        Press c to clear input

        Press r to reset response

        Press q to quit

   
##  How It Works

```mermaid
flowchart TD
    A[User Input in Terminal] --> B["App run()"]
    B --> C[Main Loop: Poll Events]
    C --> D{App State}
    
    D -->|Active| E[User types message]
    E --> F[Send to LLM API]
    F --> G[Receive Response]
    G --> H[Display in UI]

    D -->|Escape| I[Special Commands]
    I --> J{Key Pressed}
    J -->|m| K[Model Selector]
    J -->|q| L[Quit App]
    J -->|c| M[Clear Input]
    J -->|r| N[Reset Response]

    K --> O[User Selects Model]
    O --> P[Switch Active Model]

