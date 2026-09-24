# Architecture

## Overview

`mano` is a single-binary Rust CLI. On startup it parses CLI flags, loads provider credentials from disk, builds an LLM agent wired up with three tools, then runs an interactive REPL that forwards each user message to the agent (which may call tools) and prints the reply.

```
                 ┌─────────────┐
  user input ───▶│  run_chat_loop │───▶ agent.chat(prompt, history)
                 └─────────────┘            │
                       ▲                    ▼
                       │            ┌─────────────────┐
                       │            │   rig Agent      │
                       │            │ (OpenAI/Anthropic)│
                       │            └─────────────────┘
                       │                    │ tool calls
                       │                    ▼
                       │        ┌───────────────────────────┐
                       │        │ ReadFileTool / WriteFileTool │
                       │        │ RunCommandTool               │
                       │        └───────────────────────────┘
                       │                    │
                       └────── reply ───────┘
```

## Modules

### `src/main.rs`
- Defines the `Cli` struct (`--provider`, `--model`) via `clap`.
- Loads `Settings` (`openai_key`, `openai_url`, `anthropic_key`, `anthropic_url`) from `~/.mano/settings.json`.
- Branches on `--provider`: builds either an `openai::Client` or `anthropic::Client` from `rig-core`, attaches `ReadFileTool`, `RunCommandTool`, `WriteFileTool` to the agent, and calls `run_chat_loop`.

### `src/chat.rs`
- `print_banner` — renders the fixed-width ASCII/box-drawing startup banner (provider, model, cwd, tips).
- `run_chat_loop` — the REPL:
  1. Prompt via `inquire::Text`.
  2. Exit on `exit`/`quit`; skip empty input.
  3. Show a spinner (`indicatif`) while `agent.chat(prompt, chat_history.clone())` runs — this call may trigger one or more tool invocations inside `rig-core` before returning a final reply.
  4. Print the reply and append both the user and assistant turns to `chat_history`, which is resent in full on every subsequent turn.

### `src/tools.rs`
Three `rig::tool::Tool` implementations exposed to the LLM:
- `ReadFileTool` — `fs::read_to_string(path)`, returns file contents.
- `WriteFileTool` — `fs::write(path, content)`, creates or overwrites.
- `RunCommandTool` — spawns `sh -c <command>` (Unix) or `cmd /C <command>` (Windows), returns combined stdout/stderr.

Each tool operates directly on whatever arguments the LLM supplies, with no path restriction, allowlist, sandboxing, or user confirmation step.

## Data flow / trust boundary

The LLM decides which tools to call and with what arguments, based on the conversation plus anything a prior tool call returned (e.g. file contents, command output). There is no human-in-the-loop approval before a tool executes, so any untrusted content the model reads can influence subsequent `write_file`/`run_command` calls — this is the main trust boundary to keep in mind when extending the tool set. See `CLAUDE.md` for the current list of known risk areas.
