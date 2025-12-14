# Repository Guidelines
Always consult `README.md` for the latest high-level design; this file only captures implementation rules and guardrails.

- Toolchain: Rust 1.90.0 via Cargo (standard workspace).
- Fetching: all sources must use Chrome DevTools-driven HTML capture—no direct site APIs. Keep CDP helpers under `src/chrome/`.
- CLI shape: `news_agent` orchestrates `fetch → summarize → notify`, and each subcommand must also run independently with shared models.
- Sources: implement a per-site HTML trait that exposes freshness checks against `state.json` and parsing of updated content into normalized HTML/`NewsItem` intermediates. Each source owns its freshness logic and DOM parsing.
- Persistence: keep the source URL for debugging and persist normalized HTML so later subcommands (and future PDF export) can reuse it. Raw API stubs are unnecessary.
- Summarization: first pass runs per-source LLM calls using prompts defined per source in `config.yaml`; second pass runs a final refinement/deduplication prompt (also from config) over all responses. Keep summarizer abstractions thin and async-friendly.
- Notifications: email is the only active notifier; keep the abstraction simple but leave room for future backends (no extra traits required now).
- Configuration & state: align file shapes with `sample/` artifacts (config/state/logs/report/launchd). Store runtime artifacts under `~/.local/share/news_agent` as reflected in the README.
- Formatting: match the sample artifact formatting for config, state, logs, reports, and launchd templates.
- Async: network-bound components should use async Rust APIs.