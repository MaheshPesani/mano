# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Project overview

**mano** — "Agentic AI Developer CLI assistant" — is a Rust CLI (edition 2024) built on `rig-core` that lets an LLM (OpenAI or Anthropic) chat with the user in a REPL and autonomously invoke tools to read files, write files, and run shell commands.

## Build / run

```
cargo build
cargo run -- --provider anthropic --model claude-3-5-sonnet-20241022
cargo run -- --provider openai --model gpt-4-turbo
```

Requires a config file at `~/.mano/settings.json`:

```json
{
  "openai_key": "your-openai-api-key",
  "openai_url": "http://localhost:1234",
  "anthropic_key": "sk-your-anthropic-key",
  "anthropic_url": "https://api.anthropic.com/v1"
}
```

There is no automated test suite — `tests.txt` is just a scratch file of manual prompts to try against the running agent, not `cargo test` coverage.

## Architecture

- `src/main.rs` — parses CLI args (clap), loads `~/.mano/settings.json`, builds the OpenAI or Anthropic client/agent (attaching `ReadFileTool`, `RunCommandTool`, `WriteFileTool`), and hands off to `run_chat_loop`.
- `src/chat.rs` — `print_banner` (terminal UI banner) and `run_chat_loop` (the REPL: `inquire::Text` prompt → `agent.chat(prompt, chat_history)` → print reply → append both turns to `chat_history`).
- `src/tools.rs` — three `rig::tool::Tool` implementations the LLM can call: `ReadFileTool`, `WriteFileTool`, and `RunCommandTool` (spawns `sh -c` on Unix / `cmd /C` on Windows).

## Known risk areas

This CLI grants the LLM real read/write/execute access to the host with **no confirmation gate**. Be careful not to worsen these when touching related code:

- `RunCommandTool`, `ReadFileTool`, `WriteFileTool` (`src/tools.rs`) act on any LLM-supplied command/path with no sandboxing, allowlisting, or user approval step — a prompt-injected or hallucinated tool call can execute arbitrary commands or read/overwrite arbitrary files.
- API keys are stored in plaintext at `~/.mano/settings.json`.
- `chat_history` in `run_chat_loop` (`src/chat.rs`) grows unbounded for the life of the session — no truncation/windowing.
