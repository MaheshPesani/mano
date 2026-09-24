# Mano

Mano is an agentic AI developer CLI assistant written in Rust. It puts an LLM (OpenAI or Anthropic) in a REPL-style chat loop and lets it autonomously read files, write files, and run shell commands on your machine to help you understand and modify a codebase.

> ⚠️ **Security warning:** Mano grants the LLM real read/write/execute access to your host with no confirmation gate or sandboxing. Only run it in environments/repos you trust, and review `CLAUDE.md` for known risk areas before extending the tool set.

## Features

- Multi-provider support: OpenAI and Anthropic, selectable via CLI flags
- Agentic tool use powered by [`rig-core`](https://crates.io/crates/rig-core): the model can call `read_file`, `write_file`, and `run_command`
- Interactive terminal REPL with a styled banner, spinners, and colored output
- Persistent in-session chat history so the model has context across turns

## Project structure

```
mano/
├── Cargo.toml        # Dependencies and project metadata
└── src/
    ├── main.rs        # CLI arg parsing, settings loading, agent setup
    ├── chat.rs        # Terminal banner and the interactive REPL chat loop
    └── tools.rs        # Agent tools: ReadFileTool, WriteFileTool, RunCommandTool
```

## Prerequisites

- Rust (edition 2024 toolchain) and Cargo
- An API key for at least one supported provider (OpenAI or Anthropic)

## Configuration

Mano reads its settings from `~/.mano/settings.json`:

```bash
mkdir -p ~/.mano
```

Create `~/.mano/settings.json` with your keys and endpoints:

```json
{
  "openai_key": "your-openai-api-key",
  "openai_url": "http://localhost:1234",
  "anthropic_key": "sk-your-anthropic-key",
  "anthropic_url": "https://api.anthropic.com/v1"
}
```

Keys are stored in plaintext, so keep this file's permissions restricted and never commit real keys.

## Usage

Run from source with Cargo:

```bash
cargo run -- --provider openai --model gpt-4-turbo
cargo run -- --provider anthropic --model claude-3-5-sonnet-20241022
```

Or install the binary and run it directly:

```bash
cargo install --path .
mano --provider anthropic --model claude-3-5-sonnet-20241022
```

CLI flags:

| Flag | Default | Description |
|------|---------|-------------|
| `-p`, `--provider` | `openai` | LLM provider to use (`openai` or `anthropic`) |
| `-m`, `--model` | `gpt-4o` | Model name to request from the provider |

Type `exit` or `quit` at any time to end the session.

## Key dependencies

| Crate | Purpose |
|-------|---------|
| `rig-core` | Unified multi-provider AI agent framework with a native `Tool` trait |
| `inquire` | Interactive terminal text prompts |
| `indicatif` | Non-blocking terminal spinners |
| `clap` | CLI argument parsing |
| `colored` | Terminal color styling |
| `serde` / `serde_json` | Settings file (de)serialization |

## Notes

There is no automated test suite yet — `tests.txt` is a scratch file of manual prompts to try against the running agent, not `cargo test` coverage.