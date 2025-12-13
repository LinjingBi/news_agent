# news_agent

A Rust-based personal news collector that fetches daily updates from multiple AI-related sources, processes them with an LLM, and produces a structured daily brief.

Instead of relying on APIs, this project uses **Chrome DevTools automation** to fetch all content directly through your local Chrome browser. This allows the system to read login-gated websites (like X/Twitter), newsletters, blogs, or any arbitrary webpage — as long as your browser can access it.

The architecture is modular, extensible, and optimized for a daily scheduled workflow.

---

## Features

### ✔ Browser-driven fetching (Chrome CLI)
All sources are scraped via Chrome using the Remote Debugging Protocol:

- Open a tab → navigate → wait for network idle  
- Extract the final DOM HTML  
- Close the tab  

No OAuth, no API rate limits, no separate fetch adapters.

### ✔ Modular sources (each scraper is its own module)
Every source (smol.ai, AI blogs, X/Twitter threads, etc.) implements its own `Source` module that:

- fetches HTML via Chrome  
- extracts new items since the last run  
- produces structured `NewsItem` objects  

This design allows easy addition of new sources.

### ✔ LLM summarization pipeline
A second stage sends all scraped items to an LLM:

- OpenRouter, closed-source APIs, or self-hosted vLLM  
- Parallel summarization per source  
- Optional refinement / judge pass  

### ✔ Daily report generation
Outputs a Markdown report at:

```
~/.local/share/news_agent/post/YYYY-MM-DD.md
```

Includes:
- new items per source  
- extracted links (papers, repos, threads)  
- warnings and errors  

### ✔ Notifications (configurable)
Currently supported:
- Email via SMTP  
- Local macOS notification (optional)

Apple Shortcuts are intentionally **not** used.

### ✔ Persistent state & logs
State is stored in:

```
~/.local/share/news_agent/state.json
```

Logs are stored in:

```
~/.local/share/news_agent/logs/all.log
```

State tracks per-source progress (e.g. last seen item).

---

## Architecture Overview

```
Chrome (DevTools)
      ↓
HTML Fetch
      ↓
Source Parser  →  NewsItem[]
      ↓
LLM Summarizer
      ↓
Markdown Report
      ↓
Notifier (Email / macOS)
```

CLI commands:

```
news-agent          # fetch → summarize → notify
news-agent fetch    # fetch + parse only
news-agent summarize
news-agent notify
```

---

## Project Layout

```
src/
  chrome/              # Chrome DevTools client
  sources/             # Per-source HTML scrapers
  summarize/           # LLM clients
  notify/              # Email / macOS notifications
  state/               # State models & IO
  config/              # Config models & IO

sample/
  config.yaml
  state.json
  daily_report.md
  launch_agent.plist
```

---

## Why Browser-Driven Scraping?

- Uses your **existing browser login state**
- Avoids OAuth & API rate limits
- Works with JS-heavy pages
- Unified interface for all sources
- If Chrome can render it, the CLI can read it

---

## Scheduling

Designed for macOS using `launchd`:

```
launchctl load ~/Library/LaunchAgents/news_agent.plist
```

Runs once daily (default: 10:00 local time).

---

## Development Notes

- High-level design lives in this README  
- **Implementation rules for Codex live in `AGENTS.md`**  
- Source logic is intentionally per-site and HTML-based  
- LLM backend is swappable (OpenRouter → API → self-hosted)

---

## Status

Active development.

See `AGENTS.md` for strict implementation constraints and Codex instructions.
