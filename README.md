# news_agent

## Requirements summary

- **Platform & runtime**: Rust 1.90.0 CLI using Cargo, async network fetching, macOS target.
- **Scheduling**: Daily run at 10:00 local time via a `launchd` LaunchAgent. Default command runs fetch then notify; manual subcommands `fetch` and `notify` remain available.
- **Sources (extensible)**: Treat each provider as a plugin; initial targets include smol.ai issues feed, Anthropic/OpenAI/Thinking Machines blogs (HTML/RSS scrape), and X/Twitter timelines (OAuth-backed).
- **State & storage**: Use macOS filesystem under `~/.local/share/news_agent/` with `config.yaml`, `state.json`, daily markdown reports in `post/`, and aggregated logs in `logs/all.log`. Keep per-source `active`, `last_checked_at`, and `last_published_at` in state.
- **Logging & reports**: Single appended log file with prefixes `[Fetch]`, `[Notification]`, `[Snapshot]`, `[Debug]`, `[Error]`. Daily markdown report includes collected URLs and a “Warnings & Errors” section. Sample formats live in `sample/`.
- **Notifications**: Email via SMTP (e.g., QQ Mail auth code) and Apple Shortcuts; both surface fetch or delivery errors.
- **VPN guard**: Check ClashX process name from config; attempt to launch if missing, otherwise log and include failure in the report.
- **Security**: Store secrets (SMTP auth, X tokens) in macOS Keychain; keep config public and avoid plaintext secrets.
- **Extensibility**: Source and notifier traits enable adding new providers or delivery channels without touching core logic.

## Project layout

- `src/` contains the CLI scaffolding and module stubs (using `foo.rs` + `foo/` pattern) ready for incremental implementation.
- `sample/` holds reference artifacts for config, state, logging, daily report, and the LaunchAgent plist; match these formats when generating outputs.

## Usage (planned)

- `news-agent` — run fetch then notify (default daily job).
- `news-agent fetch` — collect updates and write the daily markdown report.
- `news-agent notify` — deliver the latest report via configured channels.

More details will be added as implementation progresses.
